// Tests for Longest Repeating Character Replacement — shared by template.rs, solution.rs and your attempt.
use super::*;

fn check(s: &str, k: usize, expected: usize) {
    let got = character_replacement(s, k);
    if s.len() <= 100 {
        assert_eq!(got, expected, "s = {s:?}, k = {k}");
    } else {
        assert_eq!(got, expected, "s.len() = {}, k = {k}", s.len());
    }
}

/// O(n²) oracle: every window, fixable iff len - max_count <= k.
fn brute_replacement(s: &str, k: usize) -> usize {
    let b = s.as_bytes();
    let mut best = 0;
    for i in 0..b.len() {
        let mut count = [0usize; 26];
        let mut mx = 0;
        for (len, &byte) in (1..).zip(&b[i..]) {
            let c = usize::from(byte - b'A');
            count[c] += 1;
            mx = mx.max(count[c]);
            if len - mx <= k {
                best = best.max(len);
            }
        }
    }
    best
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
}

#[test]
fn example_1() {
    check("ABAB", 2, 4);
}

#[test]
fn example_2() {
    check("AABABBA", 1, 4);
}

#[test]
fn example_no_budget() {
    check("ABCDE", 0, 1);
}

#[test]
fn example_budget_exceeds_length() {
    check("ABC", 10, 3);
}

#[test]
fn empty() {
    check("", 0, 0);
    check("", 3, 0);
}

#[test]
fn single_char() {
    check("A", 0, 1);
    check("A", 1, 1);
}

#[test]
fn k_zero_longest_run() {
    check("AABBBCC", 0, 3);
    check("ABCDE", 0, 1);
}

#[test]
fn k_equals_length() {
    // Answer can't exceed the string length.
    check("ABCD", 4, 4);
}

#[test]
fn k_larger_than_needed() {
    check("ABCDE", 3, 4);
}

#[test]
fn huge_budget_does_not_overflow() {
    // Trap: computing `max_count + k` or `left + k`.
    check("ABCDE", usize::MAX, 5);
    check("Z", usize::MAX, 1);
}

#[test]
fn all_same() {
    check("ZZZZZ", 0, 5);
    check("ZZZZZ", 2, 5);
}

#[test]
fn letters_at_alphabet_ends() {
    check("AZAZZ", 1, 4);
    check("ZAAAZ", 0, 3);
}

#[test]
fn not_just_most_frequent_letter_overall() {
    // Trap: only trying the globally most common letter ('A', 4 times) —
    // the best window is "ABBB" -> "BBBB".
    check("ACACACABBB", 1, 4);
}

#[test]
fn replacements_inside_window() {
    check("AAABAAAABBBB", 1, 8);
}

#[test]
fn answer_at_end() {
    check("ABCDEFFFFF", 1, 6);
}

#[test]
fn randomized_against_brute() {
    let mut rng = Rng(0x510E_527F_ADE6_82D1);
    for _ in 0..500 {
        let n = rng.below(21) as usize;
        let alpha = 1 + rng.below(4);
        let s: String = (0..n)
            .map(|_| char::from(b'A' + rng.below(alpha) as u8))
            .collect();
        let k = rng.below(n as u64 + 3) as usize;
        check(&s, k, brute_replacement(&s, k));
    }
}

#[test]
fn large_input() {
    // 10^5 letters, every 10th is not 'A'.
    let s: String = (0..100_000)
        .map(|i| if i % 10 == 9 { 'B' } else { 'A' })
        .collect();
    check(&s, 0, 9);
    // With k = 100 we can absorb 100 'B's: 1000 chars from index 0, plus the
    // 9 'A's up to (not including) the next 'B'.
    check(&s, 100, 1009);
}

#[test]
fn all_letters_cycling() {
    let s: String = (0..2_000u32)
        .map(|i| char::from(b'A' + (i % 26) as u8))
        .collect();
    check(&s, 2_000, 2_000);
    // Any 27 consecutive letters contain one letter twice: 27 - 2 = 25 changes.
    check(&s, 25, 27);
}

#[test]
fn large_cycling_with_long_answer() {
    // n letters cycling through all 26, with a budget k that makes the answer
    // more than half the string. Growing a window from every start, even one
    // that stops as soon as the window can't be fixed, is ~0.4 * n^2 =
    // 2.7 * 10^8 steps here; one sliding pass is 2n.
    let n: usize = 26_000;
    let (m, k) = (26, n / 2);
    let s: String = (0..n).map(|i| char::from(b'A' + (i % m) as u8)).collect();
    // Any window of length `len` holds ceil(len / m) copies of its most
    // common letter.
    let expected = (1..=n).rev().find(|&len| len - len.div_ceil(m) <= k).expect("len = 1 always fits");
    assert_eq!(character_replacement(&s, k), expected, "n = {n}, m = {m}, k = {k}");
}
