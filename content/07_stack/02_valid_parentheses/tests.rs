// Tests for Valid Parentheses — shared by template.rs, solution.rs and your attempt.
use super::*;

fn check(s: &str, expected: bool) {
    assert_eq!(is_valid(s), expected, "s = {s:?}");
}

/// Oracle: repeatedly delete adjacent matching pairs until nothing changes.
fn brute_is_valid(s: &str) -> bool {
    let mut cur = s.to_string();
    loop {
        let next = cur.replace("()", "").replace("[]", "").replace("{}", "");
        if next == cur {
            return cur.is_empty();
        }
        cur = next;
    }
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
    check("()", true);
}

#[test]
fn example_2() {
    check("()[]{}", true);
}

#[test]
fn example_3() {
    check("(]", false);
}

#[test]
fn example_4() {
    check("([])", true);
}

#[test]
fn example_5_wrong_order() {
    check("([)]", false);
}

#[test]
fn empty_string_is_valid() {
    check("", true);
}

#[test]
fn single_opener() {
    check("(", false);
    check("[", false);
    check("{", false);
}

#[test]
fn single_closer_on_empty_stack() {
    check(")", false);
    check("]", false);
    check("}", false);
}

#[test]
fn closer_first_then_balanced() {
    check(")(", false);
    check("]()", false);
}

#[test]
fn leftover_openers_at_end() {
    check("(()", false);
    check("{[]", false);
    check("()(", false);
}

#[test]
fn extra_closer_at_end() {
    check("())", false);
    check("[]]", false);
}

#[test]
fn counts_balanced_but_mismatched_trap() {
    check("{(})", false);
    check("[{]}", false);
    check("(][)", false);
}

#[test]
fn deeply_nested_mixed() {
    check("{[()()]}([]){}", true);
    check("{[(())]}", true);
    check("{[(()]}", false);
}

#[test]
fn every_pair_mismatch() {
    for s in ["(]", "(}", "[)", "[}", "{)", "{]"] {
        check(s, false);
    }
}

#[test]
fn accepts_borrowed_slices_of_owned_strings() {
    let owned = String::from("xx([]{})yy");
    assert!(is_valid(&owned[2..8]));
    assert!(!is_valid(&owned[2..7]));
}

#[test]
fn randomized_vs_brute() {
    let chars = b"()[]{}";
    let mut rng = Rng(0x2101_dead_beef);
    for _ in 0..600 {
        let len = 1 + rng.below(12) as usize;
        let s: String = (0..len).map(|_| chars[rng.below(6) as usize] as char).collect();
        assert_eq!(is_valid(&s), brute_is_valid(&s), "s = {s:?}");
    }
}

#[test]
fn randomized_valid_strings() {
    // Build guaranteed-valid strings so the "true" branch gets real coverage.
    let pairs = [('(', ')'), ('[', ']'), ('{', '}')];
    let mut rng = Rng(0x2102_cafe_f00d);
    for _ in 0..300 {
        let mut s = String::new();
        let mut stack = Vec::new();
        let target = 2 + rng.below(20) as usize;
        while s.len() < target || !stack.is_empty() {
            if s.len() < target && (stack.is_empty() || rng.below(2) == 0) {
                let (o, c) = pairs[rng.below(3) as usize];
                s.push(o);
                stack.push(c);
            } else if let Some(c) = stack.pop() {
                s.push(c);
            }
        }
        assert!(is_valid(&s), "s = {s:?}");
        assert!(brute_is_valid(&s), "oracle disagrees on {s:?}");
    }
}

#[test]
fn large_nested_valid() {
    let s = "({[".repeat(1666) + &"]})".repeat(1666);
    assert!(is_valid(&s));
}

#[test]
fn large_mismatch_at_very_end() {
    let mut s = "[".repeat(5000) + &"]".repeat(4999);
    s.push(')');
    assert!(!is_valid(&s));
}

#[test]
fn deeply_nested_max_length_strings() {
    // 16 strings of 10^4 brackets nested 5000 deep, with random bracket
    // types. Deleting matched pairs until nothing changes only gets the
    // innermost one or two per pass: about 5000 passes over 10^4 bytes each,
    // where one pass with a stack reads each byte once.
    let pairs = [(b'(', b')'), (b'[', b']'), (b'{', b'}')];
    let depth = 5_000;
    let mut rng = Rng(0x2103_9e37_79b9);
    for round in 0..16 {
        let kinds: Vec<usize> = (0..depth).map(|_| rng.below(3) as usize).collect();
        let mut s: Vec<u8> = kinds.iter().map(|&k| pairs[k].0).collect();
        s.extend(kinds.iter().rev().map(|&k| pairs[k].1));
        // Every other string has its last closer swapped for a wrong one.
        let valid = round % 2 == 0;
        if !valid {
            let last = s.len() - 1;
            s[last] = pairs[(kinds[0] + 1) % 3].1;
        }
        let s = String::from_utf8(s).expect("ASCII");
        assert_eq!(
            is_valid(&s),
            valid,
            "s.len() = {}, round = {round}",
            s.len()
        );
    }
}
