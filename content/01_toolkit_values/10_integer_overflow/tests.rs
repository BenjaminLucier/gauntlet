// Tests for Integer Overflow — shared by template.rs, solution.rs and your attempt.
// These tests are compiled with overflow checks on: plain `+`, `*`, `-x` or
// `abs()` that overflow will panic instead of returning the expected value.
use super::*;

// ── Checked ──

#[test]
fn checked_total_detects_overflow() {
    assert_eq!(checked_total(&[1, 2, 3]), Some(6));
    assert_eq!(checked_total(&[]), Some(0));
    assert_eq!(checked_total(&[u32::MAX, 0]), Some(u32::MAX));
    assert_eq!(checked_total(&[u32::MAX, 1]), None);
    assert_eq!(checked_total(&[u32::MAX / 2 + 1, u32::MAX / 2 + 1]), None);
}

#[test]
fn checked_factorial_stops_at_21() {
    assert_eq!(checked_factorial(0), Some(1));
    assert_eq!(checked_factorial(5), Some(120));
    assert_eq!(checked_factorial(20), Some(2_432_902_008_176_640_000));
    assert_eq!(checked_factorial(21), None);
    assert_eq!(checked_factorial(u32::MAX), None, "must stop early, not loop for ages");
}

#[test]
fn checked_power_edges() {
    assert_eq!(checked_power(3, 39), Some(4_052_555_153_018_976_267));
    assert_eq!(checked_power(3, 40), None);
    assert_eq!(checked_power(-2, 63), Some(i64::MIN), "fits: the negative side is one larger");
    assert_eq!(checked_power(2, 63), None);
    assert_eq!(checked_power(0, 0), Some(1));
    assert_eq!(checked_power(-1, u32::MAX), Some(-1));
}

#[test]
fn safe_abs_rejects_min() {
    assert_eq!(safe_abs(-5), Some(5));
    assert_eq!(safe_abs(i32::MAX), Some(i32::MAX));
    assert_eq!(safe_abs(i32::MIN + 1), Some(i32::MAX));
    assert_eq!(safe_abs(i32::MIN), None);
}

#[test]
fn safe_div_both_failure_modes() {
    assert_eq!(safe_div(7, 2), Some(3));
    assert_eq!(safe_div(-7, 2), Some(-3), "truncates toward zero");
    assert_eq!(safe_div(1, 0), None);
    assert_eq!(safe_div(i32::MIN, -1), None, "the answer would be i32::MAX + 1");
    assert_eq!(safe_div(i32::MIN, 1), Some(i32::MIN));
}

// ── Wrapping ──

#[test]
fn djb2_wraps() {
    assert_eq!(djb2(""), 5381);
    assert_eq!(djb2("a"), 177_670);
    assert_eq!(djb2("hello world"), 894_552_257);
    assert_eq!(djb2("héllo"), 144_619_072, "hashes UTF-8 bytes");
}

#[test]
fn negate_wrapping_min_is_fixed_point() {
    assert_eq!(negate_wrapping(5), -5);
    assert_eq!(negate_wrapping(0), 0);
    assert_eq!(negate_wrapping(i32::MAX), i32::MIN + 1);
    assert_eq!(negate_wrapping(i32::MIN), i32::MIN);
}

// ── Saturating ──

#[test]
fn apply_damage_clamps() {
    assert_eq!(apply_damage(100, -1), 99);
    assert_eq!(apply_damage(10, -20), 0);
    assert_eq!(apply_damage(250, 10), 255);
    assert_eq!(apply_damage(0, i8::MIN), 0);
    assert_eq!(apply_damage(255, i8::MAX), 255);
}

#[test]
fn saturating_product_clamps_each_step() {
    assert_eq!(saturating_product(&[2, 3, -4]), -24);
    assert_eq!(saturating_product(&[]), 1);
    assert_eq!(saturating_product(&[1 << 20, 1 << 20]), i32::MAX);
    assert_eq!(saturating_product(&[i32::MIN, 2]), i32::MIN);
    // MAX * 2 saturates to MAX first; then MAX * -1 is exact.
    assert_eq!(saturating_product(&[i32::MAX, 2, -1]), -i32::MAX);
}

// ── Overflowing ──

#[test]
fn add_with_carry_reports_the_carry() {
    assert_eq!(add_with_carry(1, 2), (3, false));
    assert_eq!(add_with_carry(u64::MAX, 2), (1, true));
    assert_eq!(add_with_carry(u64::MAX, u64::MAX), (u64::MAX - 1, true));
}

#[test]
fn mul_overflows_at_the_boundary() {
    assert!(!mul_overflows(46_340, 46_340));
    assert!(mul_overflows(46_341, 46_341));
    assert!(mul_overflows(-1, i32::MIN));
    assert!(!mul_overflows(-1, i32::MAX));
    assert!(!mul_overflows(0, i32::MIN));
}

// ── Differences and magnitudes ──

#[test]
fn distance_spans_the_full_range() {
    assert_eq!(distance(3, 10), 7);
    assert_eq!(distance(10, 3), 7);
    assert_eq!(distance(-5, 5), 10);
    assert_eq!(distance(i32::MIN, i32::MAX), u32::MAX);
}

#[test]
fn magnitude_of_min() {
    assert_eq!(magnitude(-42), 42);
    assert_eq!(magnitude(0), 0);
    assert_eq!(magnitude(i64::MIN), 9_223_372_036_854_775_808);
}

// ── Euclidean division ──

#[test]
fn euclid_div_rem_on_negatives() {
    assert_eq!(euclid_div_rem(7, 2), (3, 1));
    assert_eq!(euclid_div_rem(-7, 2), (-4, 1));
    assert_eq!((-7 / 2, -7 % 2), (-3, -1), "for contrast: / and % truncate");
    assert_eq!(euclid_div_rem(7, -2), (-3, 1));
    assert_eq!(euclid_div_rem(-7, -2), (4, 1));
    assert_eq!(euclid_div_rem(-6, 3), (-2, 0));
}

#[test]
fn wrap_index_wraps_both_ways() {
    assert_eq!(wrap_index(-1, 5), 4);
    assert_eq!(wrap_index(7, 5), 2);
    assert_eq!(wrap_index(-6, 5), 4);
    assert_eq!(wrap_index(0, 1), 0);
    assert_eq!(wrap_index(i64::MIN, 3), 1);
    assert_eq!(wrap_index(i64::MAX, 10), 7);
}

// ── Roots and logarithms ──

#[test]
fn int_sqrt_is_exact() {
    assert_eq!(int_sqrt(0), 0);
    assert_eq!(int_sqrt(15), 3);
    assert_eq!(int_sqrt(16), 4);
    assert_eq!(int_sqrt((1 << 32) - 1), 65_535);
}

#[test]
fn int_sqrt_beats_floating_point() {
    // (u64::MAX as f64).sqrt() rounds up to 2^32, which is one too many.
    assert_eq!(int_sqrt(u64::MAX), 4_294_967_295);
    let big = 999_999_999_u64;
    assert_eq!(int_sqrt(big * big - 1), big - 1);
}

#[test]
fn bit_width_handles_zero() {
    assert_eq!(bit_width(0), 0);
    assert_eq!(bit_width(1), 1);
    assert_eq!(bit_width(255), 8);
    assert_eq!(bit_width(256), 9);
    assert_eq!(bit_width(u32::MAX), 32);
}

#[test]
fn digit_count_handles_zero() {
    assert_eq!(digit_count(0), 1);
    assert_eq!(digit_count(9), 1);
    assert_eq!(digit_count(10), 2);
    assert_eq!(digit_count(999_999), 6);
    assert_eq!(digit_count(u64::MAX), 20);
}

// ── Converting between widths ──

#[test]
fn to_byte_range_checks() {
    assert_eq!(to_byte(0), Some(0));
    assert_eq!(to_byte(255), Some(255));
    assert_eq!(to_byte(256), None);
    assert_eq!(to_byte(300), None);
    assert_eq!(to_byte(-1), None);
}

#[test]
fn narrow_all_fails_on_first_misfit() {
    assert_eq!(narrow_all(&[1, -2, i64::from(i32::MAX)]), Ok(vec![1, -2, i32::MAX]));
    assert!(narrow_all(&[1, i64::from(i32::MAX) + 1]).is_err());
    assert!(narrow_all(&[i64::from(i32::MIN) - 1]).is_err());
    assert_eq!(narrow_all(&[]), Ok(vec![]));
}

// ── What `as` does ──

#[test]
fn low_byte_truncates() {
    assert_eq!(low_byte(300), 44, "300 = 0x12C keeps 0x2C");
    assert_eq!(low_byte(256), 0);
    assert_eq!(low_byte(-1), 255, "-1 is all ones");
    assert_eq!(low_byte(255), 255);
}

#[test]
fn reinterpret_is_twos_complement() {
    assert_eq!(reinterpret(127), 127);
    assert_eq!(reinterpret(128), -128);
    assert_eq!(reinterpret(200), -56);
    assert_eq!(reinterpret(255), -1);
}

#[test]
fn float_to_byte_saturates() {
    assert_eq!(float_to_byte(3.99), 3, "rounds toward zero");
    assert_eq!(float_to_byte(300.7), 255);
    assert_eq!(float_to_byte(-5.0), 0);
    assert_eq!(float_to_byte(f64::NAN), 0);
    assert_eq!(float_to_byte(f64::INFINITY), 255);
}
