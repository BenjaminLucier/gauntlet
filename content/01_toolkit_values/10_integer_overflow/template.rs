// Integer Overflow — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no widening to a bigger type to dodge overflow (no i128/u128 detours);
// no `as` casts outside the "What `as` does" group.
#![allow(unused_imports, unused_variables, dead_code)]

use std::num::TryFromIntError;

// ── Checked: `None` on overflow ─────────────────────────────────────────────

/// The sum, or `None` if it overflows `u32`. (`checked_add`, `try_fold`)
pub fn checked_total(xs: &[u32]) -> Option<u32> {
    todo!()
}

/// `n!`, or `None` if it doesn't fit in a `u64`. (`checked_mul`)
pub fn checked_factorial(n: u32) -> Option<u64> {
    todo!()
}

/// `base` to the power `exp`, or `None` on overflow. (`checked_pow`)
pub fn checked_power(base: i64, exp: u32) -> Option<i64> {
    todo!()
}

/// `|x|`, or `None` when it isn't representable (`i32::MIN`). (`checked_abs`)
pub fn safe_abs(x: i32) -> Option<i32> {
    todo!()
}

/// `a / b`, or `None` for both ways integer division can fail: dividing by
/// zero, and `i32::MIN / -1`. (`checked_div`)
pub fn safe_div(a: i32, b: i32) -> Option<i32> {
    todo!()
}

// ── Wrapping: modular arithmetic on purpose ─────────────────────────────────

/// The djb2 hash of the bytes of `s`: start at 5381, then `h = h * 33 + byte`
/// for each byte, all modulo 2³². (`wrapping_mul`, `wrapping_add`)
pub fn djb2(s: &str) -> u32 {
    todo!()
}

/// `-x` in two's complement, where `-i32::MIN` wraps back to `i32::MIN`.
/// (`wrapping_neg`)
pub fn negate_wrapping(x: i32) -> i32 {
    todo!()
}

// ── Saturating: clamp at the edges ──────────────────────────────────────────

/// Apply a signed change to a `u8` health bar, clamped to `0..=255`.
/// (`saturating_add_signed`)
pub fn apply_damage(hp: u8, delta: i8) -> u8 {
    todo!()
}

/// The product, clamped to `i32::MIN..=i32::MAX` at every step.
/// (`saturating_mul`)
pub fn saturating_product(xs: &[i32]) -> i32 {
    todo!()
}

// ── Overflowing: the result and a flag ──────────────────────────────────────

/// The wrapped sum and whether it carried out of 64 bits. (`overflowing_add`)
pub fn add_with_carry(a: u64, b: u64) -> (u64, bool) {
    todo!()
}

/// Would `a * b` overflow an `i32`? (`overflowing_mul`)
pub fn mul_overflows(a: i32, b: i32) -> bool {
    todo!()
}

// ── Differences and magnitudes ──────────────────────────────────────────────

/// `|a - b|`, for any two `i32`s — even `i32::MIN` and `i32::MAX`.
/// (`abs_diff`)
pub fn distance(a: i32, b: i32) -> u32 {
    todo!()
}

/// `|x|` as an unsigned value — defined for every `i64`, `i64::MIN` included.
/// (`unsigned_abs`)
pub fn magnitude(x: i64) -> u64 {
    todo!()
}

// ── Euclidean division ──────────────────────────────────────────────────────

/// Quotient and remainder with a remainder that is never negative
/// (`a == q * b + r`, `0 <= r < |b|`), unlike `/` and `%`, which truncate
/// toward zero. (`div_euclid`, `rem_euclid`)
pub fn euclid_div_rem(a: i32, b: i32) -> (i32, i32) {
    todo!()
}

/// Map any index — negative or past the end — into `0..len` by wrapping
/// around, so `-1` is the last slot. `len > 0`. (`rem_euclid`, `TryFrom`)
pub fn wrap_index(i: i64, len: usize) -> usize {
    todo!()
}

// ── Roots and logarithms ────────────────────────────────────────────────────

/// `⌊√n⌋`, exactly, with no floating point. (`isqrt`)
pub fn int_sqrt(n: u64) -> u64 {
    todo!()
}

/// How many bits it takes to write `n` in binary; 0 for 0.
/// (`checked_ilog2` — `ilog2` panics on 0)
pub fn bit_width(n: u32) -> u32 {
    todo!()
}

/// How many decimal digits `n` has; 0 has one digit. (`checked_ilog10`)
pub fn digit_count(n: u64) -> u32 {
    todo!()
}

// ── Converting between widths ───────────────────────────────────────────────

/// `x` as a `u8`, or `None` if it's out of range. (`u8::try_from`)
pub fn to_byte(x: i32) -> Option<u8> {
    todo!()
}

/// Narrow every value to `i32`, failing on the first that doesn't fit.
/// (`i32::try_from`, collecting into a `Result`)
pub fn narrow_all(xs: &[i64]) -> Result<Vec<i32>, TryFromIntError> {
    todo!()
}

// ── What `as` does ──────────────────────────────────────────────────────────

/// `x` narrowed to a `u8`: integer-to-integer `as` never fails — it keeps the
/// low 8 bits. (`as`)
pub fn low_byte(x: i32) -> u8 {
    todo!()
}

/// `x` as an `i8`: same width, so the bits are reinterpreted (two's
/// complement). (`as`)
pub fn reinterpret(x: u8) -> i8 {
    todo!()
}

/// `x` converted to a `u8`: float-to-integer `as` rounds toward zero and
/// *saturates*; NaN becomes 0. (`as`)
pub fn float_to_byte(x: f64) -> u8 {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
