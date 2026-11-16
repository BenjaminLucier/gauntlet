## Description

Twenty-three small functions on the integer types. In a debug build — and in
these tests, which run with overflow checks on — `i32::MAX + 1` panics; in a
release build it silently wraps. Either way it's a bug unless you said what you
wanted. Rust lets you say it: *checked*, *wrapping*, *saturating* or
*overflowing* arithmetic, plus Euclidean division, exact integer roots and
logarithms, and lossless conversions between widths.

Every function says in its doc comment which method it's after.

## Restrictions

- **No widening to dodge overflow** — no doing the maths in `i128`/`u128` and
  checking afterwards. Use the method that states the policy.
- **No `as` casts** outside the "What `as` does" group, whose whole point is to
  pin down what `as` does.

## The drills

| Group | Functions | Methods |
|---|---|---|
| Checked | `checked_total`, `checked_factorial`, `checked_power`, `safe_abs`, `safe_div` | [`checked_add`], [`checked_mul`], [`checked_pow`], [`checked_abs`], [`checked_div`], [`try_fold`] |
| Wrapping | `djb2`, `negate_wrapping` | [`wrapping_mul`], [`wrapping_add`], [`wrapping_neg`] |
| Saturating | `apply_damage`, `saturating_product` | [`saturating_add_signed`], [`saturating_mul`] |
| Overflowing | `add_with_carry`, `mul_overflows` | [`overflowing_add`], [`overflowing_mul`] |
| Differences and magnitudes | `distance`, `magnitude` | [`abs_diff`], [`unsigned_abs`] |
| Euclidean division | `euclid_div_rem`, `wrap_index` | [`div_euclid`], [`rem_euclid`] |
| Roots and logarithms | `int_sqrt`, `bit_width`, `digit_count` | [`isqrt`], [`checked_ilog2`], [`checked_ilog10`] |
| Converting between widths | `to_byte`, `narrow_all` | [`TryFrom`], [`TryFromIntError`] |
| What `as` does | `low_byte`, `reinterpret`, `float_to_byte` | [numeric casts][as] |

[`checked_add`]: https://doc.rust-lang.org/std/primitive.u32.html#method.checked_add
[`checked_mul`]: https://doc.rust-lang.org/std/primitive.u64.html#method.checked_mul
[`checked_pow`]: https://doc.rust-lang.org/std/primitive.i64.html#method.checked_pow
[`checked_abs`]: https://doc.rust-lang.org/std/primitive.i32.html#method.checked_abs
[`checked_div`]: https://doc.rust-lang.org/std/primitive.i32.html#method.checked_div
[`try_fold`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_fold
[`wrapping_mul`]: https://doc.rust-lang.org/std/primitive.u32.html#method.wrapping_mul
[`wrapping_add`]: https://doc.rust-lang.org/std/primitive.u32.html#method.wrapping_add
[`wrapping_neg`]: https://doc.rust-lang.org/std/primitive.i32.html#method.wrapping_neg
[`saturating_add_signed`]: https://doc.rust-lang.org/std/primitive.u8.html#method.saturating_add_signed
[`saturating_mul`]: https://doc.rust-lang.org/std/primitive.i32.html#method.saturating_mul
[`overflowing_add`]: https://doc.rust-lang.org/std/primitive.u64.html#method.overflowing_add
[`overflowing_mul`]: https://doc.rust-lang.org/std/primitive.i32.html#method.overflowing_mul
[`abs_diff`]: https://doc.rust-lang.org/std/primitive.i32.html#method.abs_diff
[`unsigned_abs`]: https://doc.rust-lang.org/std/primitive.i64.html#method.unsigned_abs
[`div_euclid`]: https://doc.rust-lang.org/std/primitive.i32.html#method.div_euclid
[`rem_euclid`]: https://doc.rust-lang.org/std/primitive.i32.html#method.rem_euclid
[`isqrt`]: https://doc.rust-lang.org/std/primitive.u64.html#method.isqrt
[`checked_ilog2`]: https://doc.rust-lang.org/std/primitive.u32.html#method.checked_ilog2
[`checked_ilog10`]: https://doc.rust-lang.org/std/primitive.u64.html#method.checked_ilog10
[`TryFrom`]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
[`TryFromIntError`]: https://doc.rust-lang.org/std/num/struct.TryFromIntError.html
[as]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast

## Examples

```rust
assert_eq!(checked_total(&[u32::MAX, 1]), None);
assert_eq!(checked_factorial(21), None);            // 21! > u64::MAX
assert_eq!(safe_abs(i32::MIN), None);               // |MIN| = MAX + 1
assert_eq!(safe_div(i32::MIN, -1), None);           // same problem, via division

assert_eq!(apply_damage(10, -20), 0);               // clamped, not wrapped
assert_eq!(add_with_carry(u64::MAX, 2), (1, true));
assert_eq!(distance(i32::MIN, i32::MAX), u32::MAX);

assert_eq!(euclid_div_rem(-7, 2), (-4, 1));         // whereas -7 / 2 == -3, -7 % 2 == -1
assert_eq!(wrap_index(-1, 5), 4);

assert_eq!(int_sqrt(u64::MAX), 4_294_967_295);      // f64 sqrt says 4_294_967_296
assert_eq!(bit_width(0), 0);
assert_eq!(to_byte(300), None);
assert_eq!(low_byte(300), 44);                      // `as` keeps the low 8 bits
```

## Rust notes

- `cargo build --release` turns overflow checks off by default, so code that
  "works" in release may be wrapping silently. Saying `wrapping_*` when you
  mean it documents the intent and works the same in both profiles.
- Signed ranges are asymmetric: `i32::MIN` is `-2³¹` but `i32::MAX` is
  `2³¹ - 1`. `abs`, unary `-` and `/ -1` all overflow on `MIN`.
- `/` and `%` truncate toward zero, so `%` can be negative. For "index modulo
  length" or "day of week", you want `rem_euclid`.
- Integer-to-integer `as` never fails: narrowing keeps the low bits,
  same-width reinterprets the bits. Float-to-integer `as` saturates and maps
  NaN to 0. Prefer `TryFrom` when a value might not fit, and `From` when it
  always does (`u32::from(b)` for a `u8`).
