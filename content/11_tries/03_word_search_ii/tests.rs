// Tests for Word Search II — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Sort the answer; also insist that no word is reported twice.
fn normalized<'w>(mut v: Vec<&'w str>, input: &str) -> Vec<&'w str> {
    v.sort_unstable();
    let len = v.len();
    v.dedup();
    assert_eq!(v.len(), len, "a word was reported more than once: {v:?}\n{input}");
    v
}

fn check(rows: &[&str], ws: &[&str], expected: &[&str]) {
    let input = format!("board = {rows:?}, words = {ws:?}");
    let got = normalized(find_words(rows, ws), &input);
    let mut expected = expected.to_vec();
    expected.sort_unstable();
    assert_eq!(got, expected, "{input}");
}

/// Single-word search: DFS from every cell.
fn brute_exists(b: &[&str], w: &[u8]) -> bool {
    fn go(b: &[&[u8]], used: &mut [Vec<bool>], r: usize, c: usize, w: &[u8]) -> bool {
        if used[r][c] || b[r][c] != w[0] {
            return false;
        }
        if w.len() == 1 {
            return true;
        }
        used[r][c] = true;
        let (m, n) = (b.len(), b[0].len());
        let ok = (r > 0 && go(b, used, r - 1, c, &w[1..]))
            || (r + 1 < m && go(b, used, r + 1, c, &w[1..]))
            || (c > 0 && go(b, used, r, c - 1, &w[1..]))
            || (c + 1 < n && go(b, used, r, c + 1, &w[1..]));
        used[r][c] = false;
        ok
    }
    let b: Vec<&[u8]> = b.iter().map(|row| row.as_bytes()).collect();
    let mut used = vec![vec![false; b[0].len()]; b.len()];
    (0..b.len()).any(|r| (0..b[0].len()).any(|c| go(&b, &mut used, r, c, w)))
}

fn brute_find<'w>(b: &[&str], words: &[&'w str]) -> Vec<&'w str> {
    let mut v: Vec<&str> = words
        .iter()
        .copied()
        .filter(|w| brute_exists(b, w.as_bytes()))
        .collect();
    v.sort_unstable();
    v
}

/// Tiny deterministic PRNG (xorshift64) so random tests are reproducible.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }

    fn letter(&mut self, alphabet: usize) -> char {
        char::from(b'a' + self.below(alphabet) as u8)
    }

    fn board(&mut self, m: usize, n: usize, alphabet: usize) -> Vec<String> {
        (0..m).map(|_| (0..n).map(|_| self.letter(alphabet)).collect()).collect()
    }

    fn word(&mut self, max_len: usize, alphabet: usize) -> String {
        let len = 1 + self.below(max_len);
        (0..len).map(|_| self.letter(alphabet)).collect()
    }

    /// The letters along a random walk on the board (may revisit cells, so
    /// it's only *sometimes* a valid word).
    fn walk(&mut self, b: &[&str], max_len: usize) -> String {
        let (m, n) = (b.len(), b[0].len());
        let (mut r, mut c) = (self.below(m), self.below(n));
        let mut s = String::new();
        for _ in 0..1 + self.below(max_len) {
            s.push(char::from(b[r].as_bytes()[c]));
            match self.below(4) {
                0 if r > 0 => r -= 1,
                1 if r + 1 < m => r += 1,
                2 if c > 0 => c -= 1,
                3 if c + 1 < n => c += 1,
                _ => {}
            }
        }
        s
    }
}

fn dedup(mut v: Vec<String>) -> Vec<String> {
    v.sort_unstable();
    v.dedup();
    v
}

fn as_strs(v: &[String]) -> Vec<&str> {
    v.iter().map(String::as_str).collect()
}

#[test]
fn example_1() {
    check(&["oaan", "etae", "ihkr", "iflv"], &["oath", "pea", "eat", "rain"], &["eat", "oath"]);
}

#[test]
fn example_2() {
    check(&["ab", "cd"], &["abcb"], &[]);
}

#[test]
fn single_cell() {
    check(&["a"], &["a"], &["a"]);
    check(&["a"], &["b"], &[]);
    check(&["a"], &["aa"], &[]);
}

#[test]
fn no_words_or_no_board() {
    check(&["ab", "cd"], &[], &[]);
    check(&[], &["a", "ab"], &[]);
    check(&[], &[], &[]);
}

#[test]
fn cell_cannot_be_reused() {
    check(&["ab"], &["aba", "ab", "ba"], &["ab", "ba"]);
    check(&["aa"], &["aaa", "aa"], &["aa"]);
}

#[test]
fn word_found_on_many_paths_is_reported_once() {
    check(&["aa", "aa"], &["a", "aa", "aaaa"], &["a", "aa", "aaaa"]);
    check(&["ab", "ba"], &["ab", "aba"], &["ab", "aba"]);
}

#[test]
fn no_diagonal_moves() {
    check(&["ab", "cd"], &["ad", "bc", "abdc", "acdb"], &["abdc", "acdb"]);
}

#[test]
fn words_that_are_prefixes_of_each_other() {
    check(&["abc", "xyd"], &["a", "ab", "abc", "abcd", "abcdy"], &["a", "ab", "abc", "abcd", "abcdy"]);
}

#[test]
fn backtracking_restores_cells() {
    // After exploring "ab..." from the top-left, the same cells must be usable
    // again for words starting elsewhere.
    check(&["ab", "cd"], &["abdc", "cdba", "dcab", "bacd"], &["abdc", "bacd", "cdba", "dcab"]);
}

#[test]
fn word_longer_than_board() {
    check(&["ab", "cd"], &["abdca", "abdcab"], &[]);
}

#[test]
fn snake_through_whole_board() {
    check(&["abc", "fed", "ghi"], &["abcdefghi", "ihgfedcba", "adg"], &["abcdefghi", "ihgfedcba"]);
}

#[test]
fn single_row_and_column() {
    check(&["hello"], &["hello", "olleh", "hell", "lo", "helo"], &["hell", "hello", "lo", "olleh"]);
    check(&["c", "a", "t"], &["cat", "tac", "act"], &["cat", "tac"]);
}

#[test]
fn results_borrow_the_callers_words() {
    // The answer hands back the caller's own string slices, not copies, and
    // they outlive the board they were found on.
    let words: Vec<String> = vec!["oath".into(), "pea".into(), "eat".into()];
    let refs = as_strs(&words);
    let found = {
        let board = vec![String::from("oaan"), String::from("etae"), String::from("ihkr")];
        find_words(&as_strs(&board), &refs)
    };
    assert_eq!(found.len(), 2, "{found:?}");
    for f in found {
        assert!(
            words.iter().any(|w| std::ptr::eq(w.as_str(), f)),
            "{f:?} is not one of the input slices"
        );
    }
}

#[test]
fn randomized_vs_brute_force() {
    let mut rng = Rng(63);
    for _ in 0..300 {
        let (m, n) = (1 + rng.below(4), 1 + rng.below(4));
        let rows = rng.board(m, n, 3);
        let b = as_strs(&rows);
        let mut ws = vec![];
        for _ in 0..1 + rng.below(12) {
            let w = if rng.below(2) == 0 { rng.walk(&b, 7) } else { rng.word(5, 3) };
            ws.push(w);
        }
        let ws = dedup(ws);
        let ws = as_strs(&ws);
        let input = format!("board = {b:?}, words = {ws:?}");
        let expected = brute_find(&b, &ws);
        assert_eq!(normalized(find_words(&b, &ws), &input), expected, "{input}");
    }
}

#[test]
fn large_random_board_many_words() {
    let mut rng = Rng(6363);
    let rows = rng.board(12, 12, 5);
    let b = as_strs(&rows);
    let mut ws = vec![];
    for i in 0..30_000 {
        ws.push(if i % 3 == 0 { rng.walk(&b, 10) } else { rng.word(10, 5) });
    }
    let ws = dedup(ws);
    let ws = as_strs(&ws);
    let expected = brute_find(&b, &ws);
    assert!(expected.len() > 100);
    assert_eq!(normalized(find_words(&b, &ws), "large random board"), expected);
}

#[test]
fn large_uniform_board_with_unfindable_word() {
    // Every path spells a's, so the search can't prune by letter; the trick
    // word "aaaaaaaaab" forces exploring long paths everywhere.
    let row = "a".repeat(12);
    let b = vec![row.as_str(); 12];
    let mut ws: Vec<String> = (1..=10).map(|k| "a".repeat(k)).collect();
    ws.push("aaaaaaaaab".to_string());
    let mut expected: Vec<String> = (1..=10).map(|k| "a".repeat(k)).collect();
    expected.sort_unstable();
    assert_eq!(normalized(find_words(&b, &as_strs(&ws)), "12x12 board of a's"), as_strs(&expected));
}

#[test]
fn many_words_sharing_a_long_prefix() {
    // A 12x12 board of a's with the other 25 letters on cells two apart, so no
    // two of them touch. Words: "a", "aaaaaaa", and "aaaaaaa" followed by any
    // three of the letters b..=p (3375 words, none findable: the last two
    // letters can't be adjacent). A trie walks the a-paths once; searching
    // word by word walks every a-path of length 7 again for each word, ~10^5
    // steps each: ~3 * 10^8 in all.
    let others: Vec<u8> = (b'b'..=b'z').collect();
    let tails = &others[..15];
    let mut ws = vec!["a".to_string(), "aaaaaaa".to_string()];
    for &x in tails {
        for &y in tails {
            for &z in tails {
                ws.push(String::from_utf8(vec![b'a', b'a', b'a', b'a', b'a', b'a', b'a', x, y, z]).expect("ASCII"));
            }
        }
    }
    let ws = as_strs(&ws);
    let mut rows: Vec<Vec<u8>> = vec![vec![b'a'; 12]; 12];
    for (i, &ch) in others.iter().enumerate() {
        rows[2 * (i / 5)][2 * (i % 5)] = ch;
    }
    let rows: Vec<String> = rows.into_iter().map(|r| String::from_utf8(r).expect("ASCII")).collect();
    let input = format!("board {rows:?}, {} words", ws.len());
    assert_eq!(normalized(find_words(&as_strs(&rows), &ws), &input), ["a", "aaaaaaa"], "{input}");
}
