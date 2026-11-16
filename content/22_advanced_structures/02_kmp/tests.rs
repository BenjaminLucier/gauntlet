// Tests for Knuth–Morris–Pratt — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::cell::Cell;

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

    fn bytes(&mut self, max_len: usize, alphabet: &[u8]) -> Vec<u8> {
        let len = self.below(max_len + 1);
        (0..len)
            .map(|_| alphabet[self.below(alphabet.len())])
            .collect()
    }
}

/// Every start where `pattern` occurs, by trying them all: O(n·m).
fn brute_find_all(text: &[u8], pattern: &[u8]) -> Vec<usize> {
    if pattern.len() > text.len() {
        return Vec::new();
    }
    (0..=text.len() - pattern.len())
        .filter(|&i| &text[i..i + pattern.len()] == pattern)
        .collect()
}

/// The prefix function straight from its definition.
fn brute_prefix_function(pattern: &[u8]) -> Vec<usize> {
    (0..pattern.len())
        .map(|i| {
            let s = &pattern[..=i];
            (0..s.len())
                .rev()
                .find(|&k| s[..k] == s[s.len() - k..])
                .unwrap_or(0)
        })
        .collect()
}

thread_local! {
    static COMPARISONS: Cell<usize> = const { Cell::new(0) };
}

/// A byte that counts how many times it is compared.
#[derive(Debug, Clone, Copy)]
struct Counted(u8);

impl PartialEq for Counted {
    fn eq(&self, other: &Self) -> bool {
        COMPARISONS.with(|c| c.set(c.get() + 1));
        self.0 == other.0
    }
}

impl Eq for Counted {}

#[test]
fn readme_example() {
    assert_eq!(prefix_function(b"abacaba"), [0, 0, 1, 0, 1, 2, 3]);
    assert_eq!(prefix_function(b"aabaaab"), [0, 1, 0, 1, 2, 2, 3]);
    assert_eq!(find_all("abababa", "aba"), [0, 2, 4]);
    assert_eq!(find_first("abababa", "bab"), Some(1));
    assert_eq!(find_first("abc", "abcd"), None);
    assert_eq!(find_all("abc", ""), [0, 1, 2, 3]);
    assert_eq!(find_all("héllo, héllo", "llo"), [3, 11]);
    assert_eq!(find_all_in(&[1, 2, 1, 2, 1], &[1, 2, 1]), [0, 2]);
    assert_eq!(find_first_in(&[3, 1, 2, 1, 2], &[1, 2]), Some(1));
}

#[test]
fn prefix_function_small_tables() {
    assert_eq!(prefix_function::<u8>(&[]), Vec::<usize>::new());
    assert_eq!(prefix_function(b"z"), [0]);
    assert_eq!(prefix_function(b"aaaa"), [0, 1, 2, 3]);
    assert_eq!(prefix_function(b"abcd"), [0, 0, 0, 0]);
    assert_eq!(prefix_function(b"abab"), [0, 0, 1, 2]);
}

#[test]
fn prefix_function_falls_back_instead_of_resetting() {
    // At the last 'b', the border "aba" can't be extended ('c' != 'b'), but
    // its own border "a" can: the answer is 2, not 0.
    assert_eq!(prefix_function(b"abacabab"), [0, 0, 1, 0, 1, 2, 3, 2]);
    assert_eq!(prefix_function(b"aabaaaab"), [0, 1, 0, 1, 2, 2, 2, 3]);
}

#[test]
fn overlapping_matches() {
    assert_eq!(find_all("aaaaaaa", "aaaa"), [0, 1, 2, 3]);
    assert_eq!(find_all("aaa", "a"), [0, 1, 2]);
    assert_eq!(find_all("abcabcabc", "abcabc"), [0, 3]);
}

#[test]
fn empty_pattern_matches_everywhere() {
    assert_eq!(find_all("", ""), [0]);
    assert_eq!(
        find_all("hé", ""),
        [0, 1, 2, 3],
        "byte positions, like the other offsets"
    );
    assert_eq!(find_all_in::<u32>(&[7, 8], &[]), [0, 1, 2]);
    assert_eq!(find_first_in::<u32>(&[7, 8], &[]), Some(0));
    assert_eq!(find_first("", ""), Some(0));
    assert_eq!(find_first("xyz", ""), Some(0));
}

#[test]
fn pattern_longer_than_text_or_empty_text() {
    assert!(find_all("ab", "abc").is_empty());
    assert!(find_all("", "a").is_empty());
    assert_eq!(find_first("", "a"), None);
    assert!(find_all_in(&[1u8], &[1, 1]).is_empty());
    assert_eq!(find_first_in(&[1u8], &[1, 1]), None);
}

#[test]
fn match_positions_at_the_edges() {
    assert_eq!(find_all("needle in a haystack", "needle"), [0]);
    assert_eq!(find_all("a haystack with a needle", "needle"), [18]);
    assert_eq!(find_all("same", "same"), [0]);
    assert!(find_all("haystack", "needle").is_empty());
    assert_eq!(find_first("haystack", "k"), Some(7));
}

#[test]
fn mismatch_after_partial_match_does_not_skip_text() {
    // Restarting the pattern *after* the mismatching character misses these.
    assert_eq!(find_all("aab", "ab"), [1]);
    assert_eq!(find_all("abababc", "ababc"), [2]);
    assert_eq!(find_all("aabaabaaab", "aaab"), [6]);
}

#[test]
fn utf8_text_gives_byte_offsets_on_char_boundaries() {
    let text = "日本語の本と日本";
    let hits = find_all(text, "日本");
    assert_eq!(hits, [0, 18]);
    for &i in &hits {
        assert_eq!(&text[i..i + "日本".len()], "日本");
    }
    assert_eq!(find_first("crab 🦀 crab 🦀", "🦀"), Some(5));
    assert_eq!(find_all("crab 🦀 crab 🦀", "🦀 c"), [5]);
    assert!(
        find_all("é", "e").is_empty(),
        "'é' is not 'e' plus something"
    );
}

#[test]
fn generic_over_other_element_types() {
    let text: Vec<char> = "mississippi".chars().collect();
    let pattern: Vec<char> = "issi".chars().collect();
    assert_eq!(find_all_in(&text, &pattern), [1, 4]);
    assert_eq!(find_all_in(&[-1i64, 0, -1, 0, -1], &[-1, 0, -1]), [0, 2]);
    let words = ["to", "be", "or", "not", "to", "be"];
    assert_eq!(find_all_in(&words, &["to", "be"]), [0, 4]);
    assert_eq!(prefix_function(&["x", "y", "x"]), [0, 0, 1]);
}

#[test]
fn comparison_count_is_linear() {
    // "aaa…ab" in "aaa…a": a naive search does about n·m comparisons.
    let n = 20_000;
    let m = 2_000;
    let text = vec![Counted(b'a'); n];
    let mut pattern = vec![Counted(b'a'); m - 1];
    pattern.push(Counted(b'b'));
    COMPARISONS.with(|c| c.set(0));
    assert!(find_all_in(&text, &pattern).is_empty());
    let used = COMPARISONS.with(Cell::get);
    assert!(
        used <= 8 * (n + m),
        "find_all_in: {used} comparisons for n = {n}, m = {m}"
    );
    // With no match, `find_first_in` reads the whole text too.
    COMPARISONS.with(|c| c.set(0));
    assert_eq!(find_first_in(&text, &pattern), None);
    let used = COMPARISONS.with(Cell::get);
    assert!(
        used <= 8 * (n + m),
        "find_first_in: {used} comparisons for n = {n}, m = {m}"
    );
}

#[test]
fn prefix_function_comparison_count_is_linear() {
    // Trying every candidate length from the definition, or falling back one
    // step at a time instead of through `pi`, is quadratic on "aaa…ab".
    let m = 4_000;
    let mut pattern = vec![Counted(b'a'); m - 1];
    pattern.push(Counted(b'b'));
    COMPARISONS.with(|c| c.set(0));
    let pi = prefix_function(&pattern);
    let used = COMPARISONS.with(Cell::get);
    assert!(pi[..m - 1].iter().enumerate().all(|(i, &k)| k == i), "pi wrong on a run of 'a's, m = {m}");
    assert_eq!(pi[m - 1], 0, "pi of the final 'b', m = {m}");
    assert!(used <= 8 * m, "{used} comparisons for m = {m}");
}

#[test]
fn find_first_in_stops_at_the_first_match() {
    // The first match ends at index 11 of a 200k text: stopping there costs
    // a few dozen comparisons; finishing the scan costs at least n.
    let n = 200_000;
    let mut text = vec![Counted(b'a'); n];
    text[11] = Counted(b'b');
    text[n - 1] = Counted(b'b');
    let pattern = [Counted(b'a'), Counted(b'a'), Counted(b'b')];
    COMPARISONS.with(|c| c.set(0));
    assert_eq!(find_first_in(&text, &pattern), Some(9));
    let used = COMPARISONS.with(Cell::get);
    assert!(
        used <= 1_000,
        "{used} comparisons to find a match at index 9 of {n} elements"
    );
}

#[test]
fn find_first_in_matches_find_all_in() {
    let mut rng = Rng(0x5EED_F00D_F1E5_7A11);
    for _ in 0..500 {
        let text = rng.bytes(30, b"ab");
        let pattern = rng.bytes(4, b"ab");
        let all = find_all_in(&text, &pattern);
        assert_eq!(
            find_first_in(&text, &pattern),
            all.first().copied(),
            "text {:?} pattern {:?}",
            String::from_utf8_lossy(&text),
            String::from_utf8_lossy(&pattern)
        );
    }
}

#[test]
fn adversarial_no_match_large() {
    // A character-by-character naive search makes ~(n - m) * m = 10^8
    // comparisons per call here; KMP makes ~2 (n + m). (Comparing whole
    // windows with `==` on bytes is a vectorised memcmp, much faster than a
    // loop; that is why the README makes it a restriction.)
    let text = "a".repeat(20_000);
    let pattern = format!("{}b", "a".repeat(9_999));
    assert!(find_all(&text, &pattern).is_empty());
    assert_eq!(find_first(&text, &pattern), None);
    let text_b = format!("{text}b");
    assert_eq!(find_all(&text_b, &pattern), [10_001]);
    assert_eq!(find_first(&text_b, &pattern), Some(10_001));
}

#[test]
fn many_overlapping_matches_large() {
    let text = "a".repeat(20_000);
    let pattern = "a".repeat(10_000);
    let hits = find_all(&text, &pattern);
    assert_eq!(hits.len(), 10_001);
    assert!(hits.iter().copied().eq(0..=10_000));
}

#[test]
fn periodic_text_large() {
    let text = "ab".repeat(100_000);
    let hits = find_all(&text, &"ab".repeat(1_000));
    assert_eq!(hits.len(), 99_001);
    assert!(hits.iter().enumerate().all(|(j, &i)| i == 2 * j));
    assert!(find_all(&text, &format!("{}b", "ab".repeat(1_000))).is_empty());
}

#[test]
fn randomized_prefix_function_vs_definition() {
    let mut rng = Rng(0xDEAD_BEEF_1234_5678);
    for _ in 0..500 {
        let p = rng.bytes(14, b"ab");
        assert_eq!(
            prefix_function(&p),
            brute_prefix_function(&p),
            "pattern {:?}",
            String::from_utf8_lossy(&p)
        );
    }
}

#[test]
fn randomized_matches_vs_brute() {
    let mut rng = Rng(0x0123_4567_89AB_CDEF);
    for round in 0..2_000 {
        let alphabet: &[u8] = if round % 2 == 0 { b"ab" } else { b"abc" };
        let text = rng.bytes(40, alphabet);
        let pattern = rng.bytes(6, alphabet);
        let expected = if pattern.is_empty() {
            (0..=text.len()).collect()
        } else {
            brute_find_all(&text, &pattern)
        };
        let t = String::from_utf8(text.clone()).expect("ascii");
        let p = String::from_utf8(pattern.clone()).expect("ascii");
        assert_eq!(
            find_all_in(&text, &pattern),
            expected,
            "find_all_in({t:?}, {p:?})"
        );
        assert_eq!(find_all(&t, &p), expected, "find_all({t:?}, {p:?})");
        assert_eq!(
            find_first(&t, &p),
            expected.first().copied(),
            "find_first({t:?}, {p:?})"
        );
    }
}
