// Tests for Sum of Two Integers — shared by template.rs, solution.rs and your attempt.
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

    fn next_i32(&mut self) -> i32 {
        (self.next() >> 32) as u32 as i32
    }
}

/// Oracle: the test file may add (the solution may not).
fn check(a: i32, b: i32) {
    assert_eq!(add(a, b), a.wrapping_add(b), "a = {a}, b = {b}");
}

#[test]
fn example_1() {
    assert_eq!(add(1, 2), 3);
}

#[test]
fn example_2() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn example_3() {
    assert_eq!(add(-5, 3), -2);
}

#[test]
fn zeros() {
    assert_eq!(add(0, 0), 0);
    assert_eq!(add(0, 7), 7);
    assert_eq!(add(-7, 0), -7);
}

#[test]
fn opposites_cancel() {
    assert_eq!(add(-1, 1), 0);
    assert_eq!(add(1000, -1000), 0);
    assert_eq!(add(-123, 123), 0);
}

#[test]
fn both_negative() {
    assert_eq!(add(-1, -1), -2);
    assert_eq!(add(-1000, -1000), -2000);
    assert_eq!(add(-3, -5), -8);
}

#[test]
fn mixed_signs() {
    assert_eq!(add(-5, 3), -2);
    assert_eq!(add(5, -3), 2);
    assert_eq!(add(-1, 1000), 999);
}

#[test]
fn constraint_bounds() {
    assert_eq!(add(1000, 1000), 2000);
    assert_eq!(add(-1000, 1000), 0);
    assert_eq!(add(1000, -999), 1);
}

#[test]
fn long_carry_chain() {
    // 0b0111...1 + 1: the carry ripples through every bit.
    assert_eq!(add(1023, 1), 1024);
    assert_eq!(add(0x3FFF_FFFF, 1), 0x4000_0000);
    // -1 is all ones; adding 1 carries out past bit 31.
    assert_eq!(add(-1, 1), 0);
}

#[test]
fn i32_extremes_without_overflow() {
    assert_eq!(add(i32::MAX, 0), i32::MAX);
    assert_eq!(add(i32::MIN, 0), i32::MIN);
    assert_eq!(add(i32::MIN, i32::MAX), -1);
    assert_eq!(add(i32::MAX, -1), i32::MAX - 1);
    assert_eq!(add(i32::MIN, 1), i32::MIN + 1);
    assert_eq!(add(i32::MAX - 5, 5), i32::MAX);
    assert_eq!(add(i32::MIN + 5, -5), i32::MIN);
}

#[test]
fn overflow_wraps_instead_of_panicking() {
    assert_eq!(add(i32::MAX, 1), i32::MIN);
    assert_eq!(add(i32::MIN, -1), i32::MAX);
    assert_eq!(add(i32::MAX, i32::MAX), -2);
    assert_eq!(add(i32::MIN, i32::MIN), 0);
    assert_eq!(add(0x4000_0000, 0x4000_0000), i32::MIN);
}

#[test]
fn commutative() {
    let mut rng = Rng(0x00c0_ffee_babe);
    for _ in 0..10_000 {
        let (a, b) = (rng.next_i32(), rng.next_i32());
        assert_eq!(add(a, b), add(b, a), "a = {a}, b = {b}");
    }
}

#[test]
fn exhaustive_small_range() {
    for a in -150..=150 {
        for b in -150..=150 {
            check(a, b);
        }
    }
}

#[test]
fn exhaustive_constraint_grid() {
    // Every pair on a grid covering the classic [-1000, 1000] range.
    for a in (-1000..=1000).step_by(7) {
        for b in (-1000..=1000).step_by(11) {
            check(a, b);
        }
    }
}

#[test]
fn randomized_in_constraints() {
    let mut rng = Rng(0x5a5a_a5a5_1234_8765);
    for _ in 0..50_000 {
        check(rng.range(-1000, 1000) as i32, rng.range(-1000, 1000) as i32);
    }
}

#[test]
fn randomized_full_i32_non_overflowing() {
    let mut rng = Rng(0x0fed_cba9_8765_4321);
    let mut tested = 0;
    while tested < 50_000 {
        let (a, b) = (rng.next_i32(), rng.next_i32());
        if a.checked_add(b).is_none() {
            continue;
        }
        check(a, b);
        tested += 1;
    }
}

#[test]
fn randomized_full_i32_overflowing() {
    let mut rng = Rng(0x7777_1234_4321_7777);
    let mut tested = 0;
    while tested < 50_000 {
        let (a, b) = (rng.next_i32(), rng.next_i32());
        if a.checked_add(b).is_some() {
            continue;
        }
        check(a, b);
        tested += 1;
    }
}
