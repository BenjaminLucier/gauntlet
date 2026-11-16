// Tests for Longest Palindromic Substring — shared by template.rs, solution.rs and your attempt.
use super::*;

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
}

fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

/// Brute force O(n^3): char length of the longest palindromic substring.
fn brute_max_len(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let mut best = 0;
    for i in 0..chars.len() {
        for j in i + 1..=chars.len() {
            let sub = &chars[i..j];
            if j - i > best && sub.iter().eq(sub.iter().rev()) {
                best = j - i;
            }
        }
    }
    best
}

/// Interval-DP oracle O(n^2): pal[i][j] = chars[i..=j] is a palindrome.
fn dp_max_len(s: &str) -> usize {
    let c: Vec<char> = s.chars().collect();
    let n = c.len();
    let mut pal = vec![vec![false; n]; n];
    let mut best = 0;
    for i in (0..n).rev() {
        for j in i..n {
            if c[i] == c[j] && (j - i < 2 || pal[i + 1][j - 1]) {
                pal[i][j] = true;
                best = best.max(j - i + 1);
            }
        }
    }
    best
}

/// Byte offset of `got` inside `s`, asserting that `got` really is a slice of
/// `s`'s buffer (not a copy, not a slice of something else).
fn offset_in(s: &str, got: &str) -> usize {
    let range = s.as_bytes().as_ptr_range();
    let start = got.as_ptr();
    assert!(
        range.start <= start && start <= range.end,
        "{got:?} does not point into the input {s:?}"
    );
    let offset = start as usize - s.as_ptr() as usize;
    assert!(offset + got.len() <= s.len(), "{got:?} runs past the end of {s:?}");
    offset
}

/// Checks that `got` is a palindromic slice of `s` with `want_len` chars.
fn check(s: &str, got: &str, want_len: usize) {
    let offset = offset_in(s, got);
    assert_eq!(&s[offset..offset + got.len()], got);
    assert!(is_palindrome(got), "{got:?} is not a palindrome (s = {s:?})");
    assert_eq!(
        got.chars().count(),
        want_len,
        "{got:?} is not the longest palindrome in {s:?}"
    );
}

#[test]
fn example_1() {
    let got = longest_palindrome("babad");
    assert!(got == "bab" || got == "aba", "got {got:?}");
}

#[test]
fn example_2() {
    assert_eq!(longest_palindrome("cbbd"), "bb");
}

#[test]
fn empty_string() {
    assert_eq!(longest_palindrome(""), "");
}

#[test]
fn single_char() {
    assert_eq!(longest_palindrome("a"), "a");
    assert_eq!(longest_palindrome("é"), "é");
}

#[test]
fn two_chars() {
    assert_eq!(longest_palindrome("aa"), "aa");
    let got = longest_palindrome("ab");
    assert!(got == "a" || got == "b", "got {got:?}");
}

#[test]
fn no_palindrome_longer_than_one() {
    let s = "abcdefg";
    check(s, longest_palindrome(s), 1);
}

#[test]
fn whole_string_odd_and_even() {
    assert_eq!(longest_palindrome("racecar"), "racecar");
    assert_eq!(longest_palindrome("abccba"), "abccba");
}

#[test]
fn even_length_center_trap() {
    // Only checking odd centers finds "a" / "b" at best; the answer is even.
    assert_eq!(longest_palindrome("xabbay"), "abba");
    assert_eq!(longest_palindrome("aabbbbcc"), "bbbb");
}

#[test]
fn palindrome_at_start_and_end() {
    assert_eq!(longest_palindrome("abacdef"), "aba");
    assert_eq!(longest_palindrome("fedcaba"), "aba");
}

#[test]
fn digits_and_mixed_case() {
    // Case matters: 'A' != 'a'.
    assert_eq!(longest_palindrome("Aa1221aB"), "a1221a");
    assert_eq!(longest_palindrome("AbcbA"), "AbcbA");
}

#[test]
fn reversed_substring_trap() {
    // "longest common substring of s and reverse(s)" gives "abacd"/"dcaba",
    // which isn't a palindrome. Correct answer has length 3 ("aba").
    let s = "abacdfgdcaba";
    check(s, longest_palindrome(s), 3);
}

#[test]
fn all_same_char() {
    assert_eq!(longest_palindrome("zzzzzz"), "zzzzzz");
}

#[test]
fn returned_slice_points_into_input() {
    let s = String::from("xxracecaryy");
    let got = longest_palindrome(&s);
    assert_eq!(got, "racecar");
    assert_eq!(offset_in(&s, got), 2);
    // The empty answer still borrows from (the start or end of) the input.
    let empty = String::new();
    assert_eq!(offset_in(&empty, longest_palindrome(&empty)), 0);
}

#[test]
fn result_lives_as_long_as_the_input() {
    // The result may outlive everything except the string it came from.
    fn longest_of_line(line: &str) -> &str {
        let trimmed = line.trim();
        longest_palindrome(trimmed)
    }
    let text = String::from("  level up  ");
    let got = longest_of_line(&text);
    assert_eq!(got, "level");
    assert_eq!(offset_in(&text, got), 2);
}

#[test]
fn non_ascii_palindromes_by_char() {
    // Trap: comparing bytes. "é" is [0xC3, 0xA9], so "héh" and "éé" are not
    // palindromes byte-wise, and slicing a single byte of 'é' panics.
    assert_eq!(longest_palindrome("xhéhy"), "héh");
    assert_eq!(longest_palindrome("zééq"), "éé");
    assert_eq!(longest_palindrome("日本語本日"), "日本語本日");
    assert_eq!(longest_palindrome("ab🦀ba!"), "ab🦀ba");
}

#[test]
fn non_ascii_without_long_palindrome() {
    // Every answer is one char; byte-level code would return half a char.
    for s in ["é", "éè", "日本語", "🦀🐍"] {
        check(s, longest_palindrome(s), 1);
    }
}

#[test]
fn random_small_match_brute_force() {
    let mut rng = Rng(0x5555_AAAA_1234_9876);
    let alphabets: [&[char]; 4] = [&['a', 'b'], &['a', 'b', 'c'], &['a', 'B', '1'], &['é', 'e', '日']];
    for _ in 0..400 {
        let n = rng.below(26);
        let alpha = alphabets[rng.below(alphabets.len())];
        let s: String = (0..n).map(|_| alpha[rng.below(alpha.len())]).collect();
        check(&s, longest_palindrome(&s), brute_max_len(&s));
    }
}

#[test]
fn large_all_same() {
    let s = "a".repeat(1000);
    assert_eq!(longest_palindrome(&s), s);
    let s = "é".repeat(1000);
    assert_eq!(longest_palindrome(&s), s);
}

#[test]
fn large_embedded_palindrome() {
    // "abc" repeated has no palindrome longer than 1; embed a 400-char
    // palindrome made of x/y in the middle. It is the unique longest answer.
    let mut rng = Rng(42);
    let half: String = (0..200).map(|_| if rng.below(2) == 0 { 'x' } else { 'y' }).collect();
    let pal: String = half.chars().chain(half.chars().rev()).collect();
    let s = format!("{}{}{}", "abc".repeat(100), pal, "abc".repeat(100));
    assert_eq!(s.len(), 1000);
    let got = longest_palindrome(&s);
    assert_eq!(got, pal);
    assert_eq!(offset_in(&s, got), 300);
}

#[test]
fn large_random_matches_dp_oracle() {
    let mut rng = Rng(2024);
    for alpha in [['a', 'b'], ['ä', 'ö']] {
        let s: String = (0..1000).map(|_| alpha[rng.below(2)]).collect();
        check(&s, longest_palindrome(&s), dp_max_len(&s));
    }
}

#[test]
fn large_inputs_with_long_runs() {
    // Three long runs of 'a' split by "bc" and "de": no palindrome crosses a
    // separator, so the longest is the longest run. Testing every substring
    // costs over 10^7 char comparisons per input, as a substring that spans
    // runs keeps matching until one end reaches a separator. Trying the
    // longest substrings first still tests every one longer than a run.
    // Expanding around each center is ~10^5 comparisons per input.
    const ROUNDS: usize = 10;
    let n = 1000;
    let mut rng = Rng(0x7A11_D5ED_0016_0003);
    for round in 0..ROUNDS {
        let r1 = 300 + rng.below(60);
        let r2 = 300 + rng.below(60);
        let r3 = n - 4 - r1 - r2;
        let s = format!("{}bc{}de{}", "a".repeat(r1), "a".repeat(r2), "a".repeat(r3));
        let want = r1.max(r2).max(r3);
        check(&s, longest_palindrome(&s), want);
        assert_eq!(s.len(), n, "round {round}: runs {r1}, {r2}, {r3}");
    }
}
