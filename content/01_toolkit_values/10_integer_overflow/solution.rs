// Integer Overflow — reference solution.
// Pick the overflow policy per call site: checked (Option), wrapping (mod 2^n),
// saturating (clamp) or overflowing (value + flag). Every function is O(1) or O(n).

use std::num::TryFromIntError;

// ── Checked: `None` on overflow ─────────────────────────────────────────────

pub fn checked_total(xs: &[u32]) -> Option<u32> {
    xs.iter().try_fold(0u32, |acc, &x| acc.checked_add(x))
}

pub fn checked_factorial(n: u32) -> Option<u64> {
    (1..=u64::from(n)).try_fold(1u64, u64::checked_mul)
}

pub fn checked_power(base: i64, exp: u32) -> Option<i64> {
    base.checked_pow(exp)
}

pub fn safe_abs(x: i32) -> Option<i32> {
    x.checked_abs()
}

pub fn safe_div(a: i32, b: i32) -> Option<i32> {
    a.checked_div(b)
}

// ── Wrapping: modular arithmetic on purpose ─────────────────────────────────

pub fn djb2(s: &str) -> u32 {
    s.bytes()
        .fold(5381u32, |h, b| h.wrapping_mul(33).wrapping_add(u32::from(b)))
}

pub fn negate_wrapping(x: i32) -> i32 {
    x.wrapping_neg()
}

// ── Saturating: clamp at the edges ──────────────────────────────────────────

pub fn apply_damage(hp: u8, delta: i8) -> u8 {
    hp.saturating_add_signed(delta)
}

pub fn saturating_product(xs: &[i32]) -> i32 {
    xs.iter().fold(1i32, |acc, &x| acc.saturating_mul(x))
}

// ── Overflowing: the result and a flag ──────────────────────────────────────

pub fn add_with_carry(a: u64, b: u64) -> (u64, bool) {
    a.overflowing_add(b)
}

pub fn mul_overflows(a: i32, b: i32) -> bool {
    a.overflowing_mul(b).1
}

// ── Differences and magnitudes ──────────────────────────────────────────────

pub fn distance(a: i32, b: i32) -> u32 {
    a.abs_diff(b)
}

pub fn magnitude(x: i64) -> u64 {
    x.unsigned_abs()
}

// ── Euclidean division ──────────────────────────────────────────────────────

pub fn euclid_div_rem(a: i32, b: i32) -> (i32, i32) {
    (a.div_euclid(b), a.rem_euclid(b))
}

pub fn wrap_index(i: i64, len: usize) -> usize {
    let len = i64::try_from(len).expect("a slice length fits in i64");
    usize::try_from(i.rem_euclid(len)).expect("rem_euclid is in 0..len")
}

// ── Roots and logarithms ────────────────────────────────────────────────────

pub fn int_sqrt(n: u64) -> u64 {
    n.isqrt()
}

pub fn bit_width(n: u32) -> u32 {
    n.checked_ilog2().map_or(0, |l| l + 1)
}

pub fn digit_count(n: u64) -> u32 {
    n.checked_ilog10().map_or(1, |l| l + 1)
}

// ── Converting between widths ───────────────────────────────────────────────

pub fn to_byte(x: i32) -> Option<u8> {
    u8::try_from(x).ok()
}

pub fn narrow_all(xs: &[i64]) -> Result<Vec<i32>, TryFromIntError> {
    xs.iter().map(|&x| i32::try_from(x)).collect()
}

// ── What `as` does ──────────────────────────────────────────────────────────

pub fn low_byte(x: i32) -> u8 {
    x as u8
}

pub fn reinterpret(x: u8) -> i8 {
    x as i8
}

pub fn float_to_byte(x: f64) -> u8 {
    x as u8
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
