// Tests for Pow(x, n) — shared by template.rs, solution.rs and your attempt.
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

    fn range(&mut self, lo: i64, hi: i64) -> i64 {
        lo + (self.next() % (hi - lo + 1) as u64) as i64
    }

    /// Uniform float in [0, 1).
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
}

fn close(got: f64, expected: f64, rel: f64) -> bool {
    (got - expected).abs() <= rel * expected.abs().max(1.0)
}

fn check(x: f64, n: i32, expected: f64) {
    let got = pow(x, n);
    assert!(
        close(got, expected, 1e-5),
        "pow({x}, {n}) = {got}, expected {expected}"
    );
}

/// Naive repeated multiplication — only for small |n|.
fn brute_pow(x: f64, n: i32) -> f64 {
    let r: f64 = (0..n.unsigned_abs()).map(|_| x).product();
    if n < 0 { r.recip() } else { r }
}

#[test]
fn example_1() {
    check(2.0, 10, 1024.0);
}

#[test]
fn example_2() {
    check(2.1, 3, 9.261);
}

#[test]
fn example_3() {
    check(2.0, -2, 0.25);
}

#[test]
fn exponent_zero_is_one() {
    for x in [2.0, -3.5, 0.0001, 99.9, -99.9, 1.0] {
        check(x, 0, 1.0);
    }
}

#[test]
fn exponent_one_and_minus_one() {
    check(7.5, 1, 7.5);
    check(-7.5, 1, -7.5);
    check(4.0, -1, 0.25);
    check(-0.5, -1, -2.0);
}

#[test]
fn zero_base_positive_exponent() {
    check(0.0, 1, 0.0);
    check(0.0, 7, 0.0);
    check(0.0, 1_000_001, 0.0);
}

#[test]
fn zero_base_negative_exponent_is_infinite() {
    // Outside the constraints, but the IEEE answer is well defined.
    assert_eq!(pow(0.0, -1), f64::INFINITY);
    assert_eq!(pow(0.0, -2), f64::INFINITY);
}

#[test]
fn negative_base_sign() {
    check(-2.0, 3, -8.0);
    check(-2.0, 4, 16.0);
    check(-2.0, -3, -0.125);
    check(-1.0, 1_000_001, -1.0);
    check(-1.0, 1_000_000, 1.0);
}

#[test]
fn fractional_base() {
    check(0.5, 10, 1.0 / 1024.0);
    check(0.5, -10, 1024.0);
    check(1.5, 4, 5.0625);
}

#[test]
fn n_is_i32_min_trap() {
    // `-n` / `n.abs()` overflow here and panic in a debug build. One call:
    // multiplying |n| times is 2^31 multiplications here.
    check(-1.0, i32::MIN, 1.0); // even exponent
}

#[test]
fn large_exponents() {
    // 16 exponents within 1.6 * 10^6 of +-5 * 10^6, odd and even, both signs:
    // multiplying |n| times is ~8 * 10^7 multiplications in all; squaring needs
    // ~50 per call. Bases are within 4e-8 of 1 or -1, so every power stays
    // below 10^4.
    for i in 0..16 {
        let n = if i % 2 == 0 { 5_000_000 - i * 100_003 } else { -5_000_000 + i * 100_003 };
        let magnitude = 1.0 + (f64::from(i) - 7.5) * 5e-9;
        let x = if i % 3 == 0 { -magnitude } else { magnitude };
        let sign = if x < 0.0 && n % 2 != 0 { -1.0 } else { 1.0 };
        check(x, n, sign * (f64::from(n) * (magnitude - 1.0).ln_1p()).exp());
    }
}

#[test]
fn powers_of_two_exact() {
    for n in -60..=13 {
        let expected = if n >= 0 {
            (1u64 << n) as f64
        } else {
            1.0 / (1u64 << -n) as f64
        };
        assert_eq!(pow(2.0, n), expected, "n = {n}");
    }
}

#[test]
fn randomized_vs_brute_force() {
    let mut rng = Rng(0xabcd_ef01_2345_6789);
    let mut tested = 0;
    while tested < 1000 {
        let x = (rng.unit() * 2.0 - 1.0) * 5.0;
        let n = rng.range(-40, 40) as i32;
        if x == 0.0 && n <= 0 {
            continue;
        }
        let expected = brute_pow(x, n);
        if !expected.is_finite() || expected.abs() > 1e4 {
            continue;
        }
        check(x, n, expected);
        tested += 1;
    }
}

#[test]
fn randomized_large_exponents_near_one() {
    let mut rng = Rng(0x1357_9bdf_2468_ace0);
    for _ in 0..500 {
        let x = 1.0 + (rng.unit() * 2.0 - 1.0) * 1e-6;
        let n = rng.range(-200_000, 200_000) as i32;
        let expected = (f64::from(n) * x.ln()).exp();
        if !expected.is_finite() || expected.abs() > 1e4 {
            continue;
        }
        let got = pow(x, n);
        assert!(
            close(got, expected, 1e-4),
            "pow({x}, {n}) = {got}, expected {expected}"
        );
    }
}
