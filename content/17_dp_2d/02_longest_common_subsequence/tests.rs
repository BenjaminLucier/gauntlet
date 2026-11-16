// Tests for Longest Common Subsequence — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Shorthand for ASCII strings.
fn lcs(a: &str, b: &str) -> usize {
    longest_common_subsequence(a.as_bytes(), b.as_bytes())
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

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }

    /// Random length in 1..=max_len.
    fn string_upto(&mut self, max_len: u64, alphabet: &[u8]) -> String {
        let len = 1 + self.below(max_len) as usize;
        self.string(len, alphabet)
    }

    fn string(&mut self, len: usize, alphabet: &[u8]) -> String {
        (0..len)
            .map(|_| alphabet[self.below(alphabet.len() as u64) as usize] as char)
            .collect()
    }
}

/// Plain exponential recursion, no memo.
fn brute_lcs<T: Eq>(a: &[T], b: &[T]) -> usize {
    match (a, b) {
        ([], _) | (_, []) => 0,
        ([x, a_rest @ ..], [y, b_rest @ ..]) if x == y => 1 + brute_lcs(a_rest, b_rest),
        ([_, a_rest @ ..], [_, b_rest @ ..]) => brute_lcs(a_rest, b).max(brute_lcs(a, b_rest)),
    }
}

#[test]
fn example_1() {
    assert_eq!(longest_common_subsequence(b"abcde", b"ace"), 3);
}

#[test]
fn example_2() {
    assert_eq!(longest_common_subsequence(b"abc", b"abc"), 3);
}

#[test]
fn example_3() {
    assert_eq!(longest_common_subsequence(b"abc", b"def"), 0);
}

#[test]
fn example_words() {
    assert_eq!(longest_common_subsequence(&["the", "cat", "sat"], &["the", "dog", "sat"]), 2);
}

#[test]
fn empty_inputs() {
    assert_eq!(lcs("abc", ""), 0);
    assert_eq!(lcs("", "abc"), 0);
    assert_eq!(longest_common_subsequence::<u8>(&[], &[]), 0);
}

#[test]
fn single_chars() {
    assert_eq!(lcs("a", "a"), 1);
    assert_eq!(lcs("a", "b"), 0);
}

#[test]
fn one_string_is_subsequence_of_other() {
    assert_eq!(lcs("ace", "abcde"), 3);
    assert_eq!(lcs("z", "abcdz"), 1);
    assert_eq!(lcs("abcdz", "a"), 1);
}

#[test]
fn reversed_strings() {
    assert_eq!(lcs("abcd", "dcba"), 1);
    assert_eq!(lcs("abba", "abba"), 4);
}

#[test]
fn greedy_first_match_fails() {
    // Matching 'x' first locks out "ab".
    assert_eq!(lcs("xab", "abx"), 2);
    assert_eq!(lcs("bl", "yby"), 1);
    assert_eq!(lcs("abcba", "abcbcba"), 5);
}

#[test]
fn repeated_characters() {
    assert_eq!(lcs("aaaa", "aa"), 2);
    assert_eq!(lcs("aaa", "aaaaaa"), 3);
    assert_eq!(lcs("ababab", "bababa"), 5);
}

#[test]
fn not_substring() {
    // A longest common *substring* solution returns 2 here.
    assert_eq!(lcs("axbxc", "abc"), 3);
    assert_eq!(lcs("oxcpqrsvwf", "shmtulqrypy"), 2);
}

#[test]
fn generic_over_chars_counts_characters_not_bytes() {
    // 'é' and 'è' share their first UTF-8 byte: a byte-level LCS would report 4.
    let a: Vec<char> = "café".chars().collect();
    let b: Vec<char> = "cafè".chars().collect();
    assert_eq!(longest_common_subsequence(&a, &b), 3);
    assert_eq!(longest_common_subsequence("café".as_bytes(), "cafè".as_bytes()), 4);
}

#[test]
fn generic_over_numbers_and_strings() {
    assert_eq!(longest_common_subsequence(&[1, 3, 4, 1, 2], &[3, 4, 1, 2, 1, 3]), 4);
    assert_eq!(longest_common_subsequence(&[i64::MIN, 0, i64::MAX], &[i64::MAX, i64::MIN]), 1);
    let a: Vec<String> = ["fn", "main", "(", ")"].iter().map(|s| s.to_string()).collect();
    let b: Vec<String> = ["pub", "fn", "main", "(", "x", ")"].iter().map(|s| s.to_string()).collect();
    assert_eq!(longest_common_subsequence(&a, &b), 4);
}

#[test]
fn works_without_clone_or_ord() {
    #[derive(PartialEq, Eq)]
    struct Token(u8);
    let a = [Token(1), Token(2), Token(3)];
    let b = [Token(2), Token(3), Token(1)];
    assert_eq!(longest_common_subsequence(&a, &b), 2);
}

#[test]
fn symmetric() {
    let mut rng = Rng(0x1234_5678);
    for _ in 0..100 {
        let a = rng.string_upto(15, b"abcd");
        let b = rng.string_upto(15, b"abcd");
        assert_eq!(lcs(&a, &b), lcs(&b, &a), "a = {a:?}, b = {b:?}");
    }
}

#[test]
fn random_vs_brute_force() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for _ in 0..400 {
        let a = rng.string_upto(9, b"abc");
        let b = rng.string_upto(9, b"abc");
        let expected = brute_lcs(a.as_bytes(), b.as_bytes());
        assert_eq!(lcs(&a, &b), expected, "a = {a:?}, b = {b:?}");
    }
}

#[test]
fn large_identical() {
    let mut rng = Rng(42);
    let a = rng.string(1000, b"abcdefghijklmnopqrstuvwxyz");
    assert_eq!(lcs(&a, &a), 1000);
}

#[test]
fn large_disjoint() {
    let a = "ab".repeat(500);
    let b = "cd".repeat(500);
    assert_eq!(lcs(&a, &b), 0);
}

#[test]
fn large_known_answer() {
    // b = a with every third char replaced by 'z'; 'z' never occurs in a,
    // so the LCS is exactly the number of surviving chars of a.
    let mut rng = Rng(7);
    let a = rng.string(1000, b"abcd");
    let mut b = String::new();
    let mut kept = 0;
    for (i, ch) in a.chars().enumerate() {
        if i % 3 == 0 {
            b.push('z');
        } else {
            b.push(ch);
            kept += 1;
        }
    }
    assert_eq!(lcs(&a, &b), kept);
    assert_eq!(lcs(&b, &a), kept);
}

#[test]
fn large_all_same_letter() {
    let a = "a".repeat(1000);
    let b = format!("{}{}", "b".repeat(700), "a".repeat(300));
    assert_eq!(lcs(&a, &b), 300);
}

#[test]
fn large_unbalanced_lengths() {
    let a = "abc".repeat(333);
    assert_eq!(lcs(&a, "cba"), 3);
    assert_eq!(lcs("x", &a), 0);
}
