// Tests for Minimum Window Substring — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Does `w` contain every character of `t` with multiplicity?
fn covers(w: &str, t: &str) -> bool {
    let mut cnt = [0i64; 128];
    for b in w.bytes() {
        cnt[usize::from(b)] += 1;
    }
    for b in t.bytes() {
        cnt[usize::from(b)] -= 1;
        if cnt[usize::from(b)] < 0 {
            return false;
        }
    }
    true
}

/// O(m^2) oracle: the leftmost shortest covering window, as a byte range.
fn brute_min_window(s: &str, t: &str) -> Option<(usize, usize)> {
    let mut best: Option<(usize, usize)> = None;
    for i in 0..=s.len() {
        for j in i..=s.len() {
            if best.is_some_and(|(l, r)| j - i >= r - l) {
                break;
            }
            if covers(&s[i..j], t) {
                best = Some((i, j));
                break;
            }
        }
    }
    best
}

/// Checks the exact window *and* that it's a slice of `s` at that position.
fn check(s: &str, t: &str, expected: Option<(usize, usize)>) {
    let got = min_window(s, t);
    let ctx = if s.len() + t.len() <= 200 {
        format!("s = {s:?}, t = {t:?}, got = {got:?}")
    } else {
        format!(
            "s.len() = {}, t.len() = {}, got.len() = {:?}",
            s.len(),
            t.len(),
            got.map(str::len)
        )
    };
    match (got, expected) {
        (None, None) => {}
        (Some(w), Some((l, r))) => {
            assert_eq!(w.len(), r - l, "wrong window length; {ctx}");
            assert!(covers(w, t), "result does not contain all of t; {ctx}");
            assert!(
                std::ptr::eq(w.as_ptr(), s[l..].as_ptr()),
                "expected the window starting at byte {l} of s (leftmost shortest); {ctx}"
            );
        }
        _ => panic!("expected {expected:?} as a byte range; {ctx}"),
    }
}

/// For readable unique-answer cases.
fn check_str(s: &str, t: &str, expected: Option<&str>) {
    assert_eq!(min_window(s, t), expected, "s = {s:?}, t = {t:?}");
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
    check_str("ADOBECODEBANC", "ABC", Some("BANC"));
}

#[test]
fn example_2() {
    check_str("a", "a", Some("a"));
}

#[test]
fn example_3() {
    check_str("a", "aa", None);
}

#[test]
fn example_case_sensitive() {
    check_str("aAbBa", "Ab", Some("Ab"));
}

#[test]
fn example_leftmost() {
    check("xyzyx", "y", Some((1, 2)));
}

#[test]
fn empty_t_gives_empty_window() {
    check_str("abc", "", Some(""));
    check_str("", "", Some(""));
    check("abc", "", Some((0, 0)));
}

#[test]
fn empty_s() {
    check_str("", "a", None);
}

#[test]
fn t_longer_than_s() {
    check_str("ab", "abc", None);
}

#[test]
fn char_missing() {
    check_str("abcdef", "z", None);
}

#[test]
fn whole_string() {
    check_str("abc", "cba", Some("abc"));
}

#[test]
fn duplicates_in_t() {
    // Trap: treating t as a set — "ab" would wrongly satisfy "aab".
    check_str("abcaxb", "aab", Some("abca"));
    check_str("aaflslflsldkalskaaa", "aaa", Some("aaa"));
}

#[test]
fn case_sensitive() {
    // Trap: lowercasing or using a 26-letter table.
    check_str("aAbBa", "Ab", Some("Ab"));
    check_str("abc", "A", None);
    check_str("zZ", "Z", Some("Z"));
}

#[test]
fn shrink_from_left() {
    // Trap: not shrinking after the window first becomes valid.
    check_str("aaaaaaab", "ab", Some("ab"));
}

#[test]
fn answer_at_start() {
    check_str("bacxxxxxxxb", "abc", Some("bac"));
}

#[test]
fn answer_at_end() {
    check_str("xxxxxxxbca", "abc", Some("bca"));
}

#[test]
fn ties_resolve_to_leftmost() {
    check("abxab", "ab", Some((0, 2)));
    check("baab", "ab", Some((0, 2)));
}

#[test]
fn result_borrows_from_s_not_t() {
    let s = String::from("xxABCxx");
    let window = {
        let t = String::from("CBA"); // dropped before `window` is used
        min_window(&s, &t)
    };
    assert_eq!(window, Some("ABC"));
    let w = window.expect("checked above");
    assert!(std::ptr::eq(w.as_ptr(), s[2..].as_ptr()), "not a slice of s");
}

#[test]
fn randomized_against_brute() {
    let mut rng = Rng(0x1F83_D9AB_FB41_BD6B);
    for _ in 0..400 {
        let alpha: &[u8] = if rng.below(2) == 0 { b"abc" } else { b"aAbB" };
        let m = rng.below(15) as usize;
        let n = rng.below(5) as usize;
        let s: String = (0..m)
            .map(|_| char::from(alpha[rng.below(alpha.len() as u64) as usize]))
            .collect();
        let t: String = (0..n)
            .map(|_| char::from(alpha[rng.below(alpha.len() as u64) as usize]))
            .collect();
        check(&s, &t, brute_min_window(&s, &t));
    }
}

#[test]
fn late_answers() {
    // Strings where every start has to look to the very end.
    let n = 2_000;
    // The only 'Z' is at the end and the nearest 'Y' before it (index
    // n - 1002) is 1001 chars back.
    let mut s: String = (0..n - 1000).map(|i| if i % 2 == 0 { 'Y' } else { 'a' }).collect();
    s.push_str(&"b".repeat(999));
    s.push('Z');
    check(&s, "YZ", Some((n - 1002, n)));
    // No 'z' anywhere.
    let s: String = (0..n).map(|i| char::from(b'a' + (i % 25) as u8)).collect();
    check(&s, "az", None);
    // No 'b' at all.
    check(&"a".repeat(n), "ab", None);
    // 'z' only first and 'Z' only last: the whole string.
    let mut s: String = "z".to_string();
    s.extend((1..n - 1).map(|i| char::from(b'A' + (i % 25) as u8)));
    s.push('Z');
    check(&s, "Zz", Some((0, n)));
    // As the first, with t's letters in the other order.
    let mut s: String = (0..n - 1000).map(|i| if i % 3 == 0 { 'q' } else { 'R' }).collect();
    s.push_str(&"r".repeat(999));
    s.push('P');
    let q = s.rfind('q').expect("s has a 'q'");
    check(&s, "Pq", Some((q, n)));
}

#[test]
fn large_long_t_with_answer_at_the_end() {
    // t holds k of each of 51 letters, in a scrambled order, then one 'Z'; s
    // cycles through those 51 letters and ends in the only 'Z'. The answer is
    // the last |t| ~ n / 2 chars. Growing a window from every start is
    // ~n^2 / 2 = 9 * 10^8 steps here, and checking coverage by walking t at
    // every step is ~n^2 / 4 = 4.4 * 10^8; one sliding pass is ~2n.
    let n = 42_000;
    let k = n / 102;
    let mut letters: Vec<char> = ('a'..='z').chain('A'..='Y').collect();
    let mut rng = Rng(0x3956_C25B_59F1_11F1);
    for i in (1..letters.len()).rev() {
        letters.swap(i, rng.below(i as u64 + 1) as usize);
    }
    let mut t: String = (0..51 * k).map(|i| letters[(i * 7) % 51]).collect();
    t.push('Z');
    let mut s: String = (0..n - 1).map(|i| letters[i % 51]).collect();
    s.push('Z');
    assert_eq!(min_window(&s, &t).map(str::len), Some(t.len()), "s.len() = {n}, t.len() = {}", t.len());
}

#[test]
fn long_t() {
    // t uses 52 letters with multiplicity; s is a cyclic arrangement of them,
    // so the answer is the first length-|t| window (|t| = 520, |s| = 10^4).
    let letters: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
    let t: String = (0..520).map(|i| letters[i % 52]).collect();
    let s: String = (0..10_000).map(|i| letters[(i * 7) % 52]).collect();
    // gcd(7, 52) = 1, so every 52-block of s is a permutation of all 52 letters
    // and any 520-window holds each letter exactly 10 times.
    check(&s, &t, Some((0, 520)));
}

#[test]
fn impossible_by_one_char() {
    let s = "a".repeat(2_000);
    let t = format!("{}b", "a".repeat(1_999));
    check_str(&s, &t, None);
}
