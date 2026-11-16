// Tests for Regular Expression Matching — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Shorthand for patterns that are known to be valid.
fn m(s: &str, p: &str) -> bool {
    is_match(s, p).expect("test pattern is valid")
}

fn nothing_to_repeat(at: usize) -> Result<bool, PatternError> {
    Err(PatternError::NothingToRepeat { at })
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

/// Plain recursive backtracking matcher over chars (exponential, fine for tiny inputs).
/// Assumes a valid pattern.
fn brute_match(s: &[char], p: &[char]) -> bool {
    let Some(&atom) = p.first() else {
        return s.is_empty();
    };
    let first = s.first().is_some_and(|&c| atom == '.' || atom == c);
    if p.get(1) == Some(&'*') {
        brute_match(s, &p[2..]) || (first && brute_match(&s[1..], p))
    } else {
        first && brute_match(&s[1..], &p[1..])
    }
}

fn brute(s: &str, p: &str) -> bool {
    let (s, p): (Vec<char>, Vec<char>) = (s.chars().collect(), p.chars().collect());
    brute_match(&s, &p)
}

/// Byte offset of the first `*` with nothing to repeat, if any.
fn brute_first_error(p: &str) -> Option<usize> {
    let mut prev: Option<char> = None;
    for (at, ch) in p.char_indices() {
        if ch == '*' && matches!(prev, None | Some('*')) {
            return Some(at);
        }
        prev = Some(ch);
    }
    None
}

#[test]
fn example_1() {
    assert_eq!(is_match("aa", "a"), Ok(false));
}

#[test]
fn example_2() {
    assert_eq!(is_match("aa", "a*"), Ok(true));
}

#[test]
fn example_3() {
    assert_eq!(is_match("ab", ".*"), Ok(true));
}

#[test]
fn example_4_and_5() {
    assert_eq!(is_match("aab", "c*a*b"), Ok(true));
    assert_eq!(is_match("héllo", "h.l*o"), Ok(true));
}

#[test]
fn example_errors() {
    assert_eq!(is_match("a", "*a"), nothing_to_repeat(0));
    assert_eq!(is_match("a", "a**"), nothing_to_repeat(2));
}

#[test]
fn classic_cases() {
    assert!(m("aab", "c*a*b"));
    assert!(!m("mississippi", "mis*is*p*."));
    assert!(m("mississippi", "mis*is*ip*."));
}

#[test]
fn empty_inputs() {
    assert!(m("", ""));
    assert!(m("", "a*"));
    assert!(m("", "a*b*.*"));
    assert!(!m("", "a"));
    assert!(!m("", "."));
    assert!(!m("a", ""));
}

#[test]
fn must_match_whole_string() {
    assert!(!m("abc", "ab"));
    assert!(!m("abc", "bc"));
    assert!(!m("ab", "."));
    assert!(m("abc", "abc"));
}

#[test]
fn star_can_mean_zero() {
    assert!(m("b", "a*b"));
    assert!(m("a", "ab*"));
    assert!(m("a", "ab*c*d*"));
}

#[test]
fn star_binds_to_one_element() {
    // "ab*" is "a" followed by any number of b's, not "(ab)*".
    assert!(!m("abab", "ab*"));
    assert!(m("abbb", "ab*"));
}

#[test]
fn greedy_star_must_backtrack() {
    // A star that eats as much as possible and never gives back fails these.
    assert!(m("aaa", "a*a"));
    assert!(m("aaa", "ab*a*c*a"));
    assert!(m("bbbba", ".*a*a"));
    assert!(m("ab", ".*.."));
    assert!(!m("ab", ".*c"));
}

#[test]
fn dot_star_matches_anything() {
    assert!(m("anything", ".*"));
    assert!(m("abcdef", "a.*f"));
    assert!(!m("abcdef", "a.*e"));
}

#[test]
fn star_after_dot_inside_pattern() {
    assert!(m("abcd", "a.*d"));
    assert!(m("ad", "a.*d"));
    assert!(!m("abcde", "a.*d"));
}

#[test]
fn malformed_patterns_are_errors() {
    assert_eq!(is_match("", "*"), nothing_to_repeat(0));
    assert_eq!(is_match("", "**"), nothing_to_repeat(0));
    assert_eq!(is_match("aa", "a***"), nothing_to_repeat(2));
    assert_eq!(is_match("abc", "a*b**c"), nothing_to_repeat(4));
    assert_eq!(is_match("x", ".**"), nothing_to_repeat(2));
}

#[test]
fn error_even_if_match_fails_early() {
    // A matcher that stops at the first mismatch never sees the bad star.
    assert_eq!(is_match("b", "a**"), nothing_to_repeat(2));
    assert_eq!(is_match("", "x.y**"), nothing_to_repeat(4));
    assert_eq!(is_match("zzzz", "a*b*c**"), nothing_to_repeat(6));
}

#[test]
fn error_offset_is_in_bytes() {
    // 'é' is two bytes, '🦀' four: the offset is where you would slice the pattern.
    assert_eq!(is_match("é", "é**"), nothing_to_repeat(3));
    assert_eq!(is_match("", "🦀**"), nothing_to_repeat(5));
    let p = "日本*語**";
    let Err(PatternError::NothingToRepeat { at }) = is_match("", p) else {
        panic!("pattern {p:?} should be rejected");
    };
    assert_eq!(&p[at..], "*");
}

#[test]
fn error_is_a_std_error() {
    let err = is_match("a", "*a").expect_err("leading star is malformed");
    let boxed: Box<dyn std::error::Error> = Box::new(err);
    assert!(!boxed.to_string().is_empty());
    // Usable with `?` in a function returning Box<dyn Error>.
    fn check(p: &str) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(is_match("aa", p)?)
    }
    assert!(check("a*").expect("valid"));
    assert!(check("**").is_err());
}

#[test]
fn dot_matches_one_char_not_one_byte() {
    assert!(m("é", "."));
    assert!(!m("é", ".."));
    assert!(m("日本", ".."));
    assert!(!m("日本", "..."));
    assert!(m("🦀🦀🦀", "🦀*"));
    assert!(m("ééé", "é*"));
    assert!(!m("éè", "é*"));
    assert!(m("naïve", "na.ve"));
}

#[test]
fn literal_non_letters() {
    // Only '.' and '*' are special.
    assert!(m("a+b", "a+b"));
    assert!(m("a?", "a?"));
    assert!(!m("aab", "a+b"));
    assert!(m("  ", " *"));
}

#[test]
fn random_vs_brute_force() {
    let mut rng = Rng(0x5EED_004E_6EC0_DE01);
    let atoms = ['a', 'b', '.'];
    for _ in 0..1000 {
        let ls = rng.below(8) as usize;
        let s: String = (0..ls).map(|_| if rng.below(2) == 0 { 'a' } else { 'b' }).collect();
        let tokens = rng.below(6) as usize;
        let mut p = String::new();
        for _ in 0..tokens {
            p.push(atoms[rng.below(3) as usize]);
            if rng.below(2) == 0 {
                p.push('*');
            }
        }
        assert_eq!(is_match(&s, &p), Ok(brute(&s, &p)), "s = {s:?}, p = {p:?}");
    }
}

#[test]
fn random_unicode_vs_brute_force() {
    let mut rng = Rng(0x1BAD_F00D_2BAD_F00D);
    let chars = ['é', 'e', '🦀'];
    for _ in 0..500 {
        let ls = rng.below(7) as usize;
        let s: String = (0..ls).map(|_| chars[rng.below(3) as usize]).collect();
        let mut p = String::new();
        for _ in 0..rng.below(6) {
            p.push(if rng.below(4) == 0 { '.' } else { chars[rng.below(3) as usize] });
            if rng.below(2) == 0 {
                p.push('*');
            }
        }
        assert_eq!(is_match(&s, &p), Ok(brute(&s, &p)), "s = {s:?}, p = {p:?}");
    }
}

#[test]
fn random_malformed_patterns() {
    let mut rng = Rng(0xE77E_0000_1111_2222);
    let alphabet = ['a', '.', '*', 'é'];
    for _ in 0..500 {
        let len = rng.below(8) as usize;
        let p: String = (0..len).map(|_| alphabet[rng.below(4) as usize]).collect();
        let s = "a".repeat(rng.below(4) as usize);
        let expected = match brute_first_error(&p) {
            Some(at) => nothing_to_repeat(at),
            None => Ok(brute(&s, &p)),
        };
        assert_eq!(is_match(&s, &p), expected, "s = {s:?}, p = {p:?}");
    }
}

#[test]
fn large_backtracking_blowup_many_stars() {
    // Unmemoized backtracking tries every way to split 30 a's among 14 stars.
    let s = "a".repeat(30);
    let p = format!("{}c", "a*".repeat(14));
    assert!(!m(&s, &p));
    let s2 = format!("{}b", "a".repeat(24));
    assert!(!m(&s2, "a*a*a*a*a*a*a*a*c"));
}

#[test]
fn large_backtracking_blowup_dot_stars() {
    let s = "a".repeat(30);
    let p = format!("{}b", ".*".repeat(14));
    assert!(!m(&s, &p));
}

#[test]
fn large_true_cases() {
    let s = "a".repeat(30);
    assert!(m(&s, &"a*".repeat(15)));
    assert!(m(&s, &format!("{}a", "a*".repeat(14))));
    assert!(m(&format!("{}b", "a".repeat(29)), &format!("{}b", ".*a*".repeat(7))));
}

#[test]
fn large_non_ascii_blowup() {
    let s = "é".repeat(30);
    let p = format!("{}ü", "é*".repeat(14));
    assert!(!m(&s, &p));
}
