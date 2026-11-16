// Tests for Longest Substring Without Repeating Characters — shared by template.rs, solution.rs and your attempt.
use super::*;

fn check(s: &str, expected: usize) {
    let got = length_of_longest_substring(s);
    if s.len() <= 100 {
        assert_eq!(got, expected, "s = {s:?}");
    } else {
        assert_eq!(got, expected, "s.len() = {}", s.len());
    }
}

/// O(n²) oracle: extend from every start until a repeat.
fn brute_longest(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let mut best = 0;
    for i in 0..chars.len() {
        let mut seen = Vec::new();
        for &c in &chars[i..] {
            if seen.contains(&c) {
                break;
            }
            seen.push(c);
        }
        best = best.max(seen.len());
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
    check("abcabcbb", 3);
}

#[test]
fn example_2() {
    check("bbbbb", 1);
}

#[test]
fn example_3() {
    check("pwwkew", 3);
}

#[test]
fn example_unicode() {
    check("日本日本語", 3);
}

#[test]
fn empty() {
    check("", 0);
}

#[test]
fn single_space() {
    check(" ", 1);
}

#[test]
fn all_distinct() {
    check("abcdef", 6);
}

#[test]
fn left_must_not_move_backwards() {
    // Trap: jumping left to last['a'] + 1 = 1 at the final 'a' (window "bba").
    check("abba", 2);
}

#[test]
fn restart_too_far() {
    // Trap: restarting the window *at* the repeat instead of just after the
    // earlier copy — "dvdf" -> "vdf".
    check("dvdf", 3);
}

#[test]
fn best_at_end() {
    check("aab", 2);
    check("aaaaabcdef", 6);
}

#[test]
fn spaces_digits_symbols() {
    check("a b c!", 4); // "b c!"
    check("12 3!1", 5);
    check("!@#$%^&*()", 10);
}

#[test]
fn case_sensitive() {
    check("aAbB", 4);
}

#[test]
fn counts_chars_not_bytes() {
    // Trap: "日本語" is 9 bytes but 3 characters.
    check("日本語", 3);
    check("🦀", 1);
}

#[test]
fn multi_byte_repeats() {
    // Trap: a byte-based window sees C3 A9 C3 A9 … and answers 2.
    check("ééé", 1);
    check("éèé", 2);
    // 'é' (C3 A9) and '©' (C2 A9) share a byte but are different chars.
    check("é©é", 2);
}

#[test]
fn randomized_against_brute() {
    let alphabet: Vec<char> = "abcdA1 !é日🦀".chars().collect();
    let mut rng = Rng(0xA54F_F53A_5F1D_36F1);
    for _ in 0..500 {
        let n = rng.below(20) as usize;
        let k = 1 + rng.below(alphabet.len() as u64);
        let s: String = (0..n).map(|_| alphabet[rng.below(k) as usize]).collect();
        check(&s, brute_longest(&s));
    }
}

#[test]
fn large_all_printable_cycling() {
    // 2*10^4 chars cycling through all 95 printable ASCII; answer 95.
    let s: String = (0..20_000u32)
        .map(|i| char::from(b' ' + (i % 95) as u8))
        .collect();
    check(&s, 95);
}

#[test]
fn large_single_char() {
    check(&"z".repeat(50_000), 1);
}

#[test]
fn many_distinct_unicode() {
    // 1000 distinct CJK characters (3000 bytes), then a repeat of the first.
    let mut s: String = (0x4E00u32..0x4E00 + 1_000)
        .map(|u| char::from_u32(u).expect("valid scalar"))
        .collect();
    s.push('\u{4E00}');
    check(&s, 1_000);
}

#[test]
fn large_shuffled_alphabet_twice() {
    // `d` holds the first 2500 chars (all of ASCII and 2372 2-byte chars:
    // 4872 bytes). The input is a shuffle of `d` written twice, so the answer
    // is d.len() and every char of the second copy repeats one inside the
    // window. Extending a fresh window from every start is about
    // d.len()^2 = 6 * 10^6 steps here; one sliding pass is 2 * d.len().
    let mut d: Vec<char> = (0u32..2500).filter_map(char::from_u32).collect();
    assert_eq!(d.len(), 2500);
    let mut rng = Rng(0x7137_4491_23EF_65CD);
    for i in (1..d.len()).rev() {
        d.swap(i, rng.below(i as u64 + 1) as usize);
    }
    let s: String = d.iter().chain(&d).collect();
    assert_eq!(length_of_longest_substring(&s), d.len(), "s.len() = {}", s.len());
}
