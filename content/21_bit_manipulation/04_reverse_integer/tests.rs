// Tests for Reverse Integer — shared by template.rs, solution.rs and your attempt.
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

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

/// Oracle: reverse through a string and an i64 (the oracle is allowed to cheat).
fn brute_reverse(x: i32) -> Option<i32> {
    let digits: String = i64::from(x).abs().to_string().chars().rev().collect();
    let v: i64 = digits.parse().expect("reversed digits are a valid number");
    i32::try_from(if x < 0 { -v } else { v }).ok()
}

fn check(x: i32, expected: Option<i32>) {
    assert_eq!(reverse(x), expected, "x = {x}");
}

#[test]
fn example_1() {
    check(123, Some(321));
}

#[test]
fn example_2() {
    check(-123, Some(-321));
}

#[test]
fn example_3() {
    check(120, Some(21));
}

#[test]
fn zero_is_some_zero() {
    // The classic "0 means overflow" convention can't tell these apart.
    check(0, Some(0));
    check(1_534_236_469, None);
}

#[test]
fn single_digits() {
    for d in -9..=9 {
        check(d, Some(d));
    }
}

#[test]
fn trailing_zeros() {
    check(1000, Some(1));
    check(-1000, Some(-1));
    check(1_000_000_000, Some(1));
    check(-2_000_000_000, Some(-2));
    check(1020, Some(201));
}

#[test]
fn i32_max_overflows() {
    // 7463847412 > i32::MAX
    check(i32::MAX, None);
}

#[test]
fn i32_min_overflows() {
    // abs(i32::MIN) itself overflows; reversed value -8463847412 is out of range.
    check(i32::MIN, None);
}

#[test]
fn classic_overflow_case() {
    check(1_534_236_469, None); // 9646324351
    check(-1_534_236_469, None);
}

#[test]
fn just_fits() {
    check(1_463_847_412, Some(2_147_483_641));
    check(-1_463_847_412, Some(-2_147_483_641));
    check(1_463_847_413, None); // 3147483641
    check(-2_147_483_412, Some(-2_143_847_412));
    check(2_147_483_412, Some(2_143_847_412));
}

#[test]
fn negative_side_is_one_wider() {
    // i32::MIN itself can't be produced (its reversal -8463847412 isn't an
    // input), but negative results may go one step further than positive ones.
    check(-1_463_847_412, Some(-2_147_483_641));
    check(-8_463_847, Some(-7_483_648));
    check(-1_563_847_412, None); // -2147483651
}

#[test]
fn ten_digit_overflow_by_last_digit() {
    check(1_000_000_003, None); // 3000000001
    check(1_000_000_002, Some(2_000_000_001));
    check(-1_000_000_003, None);
    check(-1_000_000_002, Some(-2_000_000_001));
    check(1_563_847_412, None); // 2147483651 — only the last digit pushes it over
    check(-1_563_847_412, None); // -2147483651
    check(1_463_847_422, None); // 2247483641
    check(1_473_847_412, None); // 2147483741 > MAX
}

#[test]
fn palindromes() {
    check(12321, Some(12321));
    check(-1_234_554_321, Some(-1_234_554_321));
}

#[test]
fn near_extremes_vs_brute_force() {
    for x in (i32::MAX - 5000)..=i32::MAX {
        check(x, brute_reverse(x));
    }
    for x in i32::MIN..=(i32::MIN + 5000) {
        check(x, brute_reverse(x));
    }
}

#[test]
fn small_range_vs_brute_force() {
    for x in -20_000..=20_000 {
        check(x, brute_reverse(x));
    }
}

#[test]
fn randomized_vs_brute_force() {
    let mut rng = Rng(0x1357_2468_ace0_bdf1);
    for _ in 0..100_000 {
        let x = (rng.next() >> 32) as u32 as i32;
        check(x, brute_reverse(x));
    }
}

#[test]
fn randomized_near_overflow_boundary() {
    // Ten-digit numbers whose reversal is close to ±2147483647.
    let mut rng = Rng(0x0bee_f0be_ef0b_eef0);
    for _ in 0..20_000 {
        let tail = rng.below(100_000) as i64; // perturb the low-order reversed digits
        let r = 2_147_400_000i64 + tail * if rng.below(2) == 0 { 1 } else { 2 };
        let x_str: String = r.to_string().chars().rev().collect();
        let mut x: i64 = x_str.parse().expect("digits parse as a number");
        if rng.below(2) == 0 {
            x = -x;
        }
        let Ok(x) = i32::try_from(x) else { continue };
        check(x, brute_reverse(x));
    }
}

#[test]
fn round_trip_when_no_trailing_zero() {
    // Reversing twice gives back the input whenever both steps fit.
    let mut rng = Rng(0x2468_ace0_1357_9bdf);
    for _ in 0..20_000 {
        let x = (rng.next() >> 33) as i32;
        if x % 10 == 0 {
            continue;
        }
        if let Some(r) = reverse(x) {
            assert_eq!(reverse(r), Some(x), "x = {x}, reversed = {r}");
        } else {
            assert_eq!(brute_reverse(x), None, "x = {x}");
        }
    }
}
