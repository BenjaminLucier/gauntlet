// Tests for Decode Ways — shared by template.rs, solution.rs and your attempt.
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

/// Exponential brute force: try every way to cut off a valid 1- or 2-digit
/// piece, by parsing the piece and checking it maps to a letter.
fn brute_count(s: &str) -> u64 {
    if s.is_empty() {
        return 1;
    }
    (1..=2.min(s.len()))
        .filter(|&len| {
            let piece = &s[..len];
            !piece.starts_with('0')
                && (1..=26).contains(&piece.parse::<u32>().expect("digits only"))
        })
        .map(|len| brute_count(&s[len..]))
        .sum()
}

/// fib[k] with fib[1] = fib[2] = 1, in u128.
fn fibonacci(len: usize) -> Vec<u128> {
    let mut fib = vec![0u128, 1, 1];
    while fib.len() < len {
        let k = fib.len();
        fib.push(fib[k - 1] + fib[k - 2]);
    }
    fib
}

#[test]
fn example_1() {
    assert_eq!(num_decodings("12"), 2);
}

#[test]
fn example_2() {
    assert_eq!(num_decodings("226"), 3);
}

#[test]
fn example_3() {
    assert_eq!(num_decodings("06"), 0);
}

#[test]
fn empty_has_one_decoding() {
    assert_eq!(num_decodings(""), 1);
}

#[test]
fn single_digits() {
    assert_eq!(num_decodings("0"), 0);
    assert_eq!(num_decodings("1"), 1);
    assert_eq!(num_decodings("9"), 1);
}

#[test]
fn ten_and_twenty_only_one_way() {
    // Trap: counting "1 0" as a decoding.
    assert_eq!(num_decodings("10"), 1);
    assert_eq!(num_decodings("20"), 1);
}

#[test]
fn zero_after_large_digit_is_dead() {
    // "30", "90": the 0 can't stand alone and "30" > 26.
    assert_eq!(num_decodings("30"), 0);
    assert_eq!(num_decodings("190"), 0);
    assert_eq!(num_decodings("1090"), 0);
}

#[test]
fn double_zero_is_dead() {
    assert_eq!(num_decodings("100"), 0);
    assert_eq!(num_decodings("1001"), 0);
}

#[test]
fn boundary_26_and_27() {
    // Trap: off-by-one on the upper bound.
    assert_eq!(num_decodings("26"), 2);
    assert_eq!(num_decodings("27"), 1);
    assert_eq!(num_decodings("19"), 2);
}

#[test]
fn leading_zero_anywhere_gives_zero() {
    assert_eq!(num_decodings("012"), 0);
    assert_eq!(num_decodings("00"), 0);
    // "01" is not "1": a pair starting with 0 is never valid.
    assert_eq!(num_decodings("101"), 1);
}

#[test]
fn zero_steals_previous_digit() {
    // "2101": 2 10 1 only — the 0 forces "10", so "21" can't be used.
    assert_eq!(num_decodings("2101"), 1);
    assert_eq!(num_decodings("11106"), 2);
}

#[test]
fn random_small_match_brute_force() {
    let mut rng = Rng(0x00C0_FFEE_1234_5678);
    // Weighted towards 0/1/2 so that zeros and two-digit pieces are common.
    let digits = b"0011122234567899";
    for _ in 0..500 {
        let n = rng.below(19);
        let s: String = (0..n).map(|_| char::from(digits[rng.below(digits.len())])).collect();
        assert_eq!(num_decodings(&s), brute_count(&s), "s = {s:?}");
    }
}

#[test]
fn all_ones_follow_fibonacci() {
    // k ones decode in fib(k + 1) ways.
    let fib = fibonacci(95);
    for k in 0..=92 {
        assert_eq!(u128::from(num_decodings(&"1".repeat(k))), fib[k + 1], "k = {k}");
    }
}

#[test]
fn large_all_ones_past_i32() {
    // 45 ones -> fib(46) = 1836311903 ways: past an i32 accumulator.
    assert_eq!(num_decodings(&"1".repeat(45)), 1_836_311_903);
    assert_eq!(num_decodings(&"1".repeat(46)), 2_971_215_073);
}

#[test]
fn long_ambiguous_input_is_not_exponential() {
    // Every adjacent pair is a valid letter, so unmemoized recursion makes
    // ~fib(93) = 1.2 * 10^19 calls on each of these 92-digit strings.
    let want = 12_200_160_415_121_876_738;
    for s in ["1".repeat(92), "2".repeat(92), "12".repeat(46), "21".repeat(46)] {
        assert_eq!(num_decodings(&s), want, "s = {s}");
    }
}

#[test]
fn overflow_past_i64() {
    // 92 ones -> fib(93) > i64::MAX, but still fits in u64.
    assert_eq!(num_decodings(&"1".repeat(92)), 12_200_160_415_121_876_738);
    // A trailing "0" forces "10": as many ways as 91 ones, though the prefix
    // of 92 ones (fib(93)) is computed on the way.
    assert_eq!(num_decodings(&format!("{}0", "1".repeat(92))), 7_540_113_804_746_346_429);
}

#[test]
fn large_max_length_with_separators() {
    // 40 ones then "7": "17" is valid, giving fib(42) = 267914296. Then "7"
    // cannot pair with the following "1", "10"*29 is forced (1 way), and the
    // final "9" cannot pair with the preceding "0".
    let s = format!("{}7{}9", "1".repeat(40), "10".repeat(29));
    assert_eq!(s.len(), 100);
    assert_eq!(num_decodings(&s), 267_914_296);
}

#[test]
fn large_forced_chain() {
    // "10" * 50: every 0 forces its 1, exactly one decoding.
    assert_eq!(num_decodings(&"10".repeat(50)), 1);
    // A trailing "0" is fine after a 2 (forced "20") but kills every
    // decoding after a 3.
    assert_eq!(num_decodings(&format!("{}20", "3".repeat(98))), 1);
    assert_eq!(num_decodings(&format!("{}30", "3".repeat(98))), 0);
}

#[test]
fn large_mixed_segments_multiply() {
    // Independent segments multiply: "11110" decodes like "111" (3 ways) and
    // "2220" like "22" (2 ways); "02" can't join them.
    assert_eq!(num_decodings("111102220"), 3 * 2);
    // "113" has 3 decodings and "31" never pairs, so 33 copies give 3^33.
    let s = "113".repeat(33);
    assert_eq!(num_decodings(&s), 3u64.pow(33));
}
