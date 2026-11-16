## Description

Compute `x` raised to the integer power `n`.

Don't use `f64::powi` / `f64::powf` — the point is to implement the
exponentiation yourself. Results are compared with a small relative tolerance.

## Signature

```rust
pub fn pow(x: f64, n: i32) -> f64
```

The exponent is a signed `i32` because negative powers are allowed
(`x^-n = 1 / x^n`); the full `i32` range is valid, including `i32::MIN`. The
result follows normal IEEE float behaviour: `0.0` raised to a negative power is
infinity, just like `1.0 / 0.0`.

## Examples

```rust
assert_eq!(pow(2.0, 10), 1024.0);
assert!((pow(2.1, 3) - 9.261).abs() < 1e-9);
assert_eq!(pow(2.0, -2), 0.25);          // 1 / 2² = 1 / 4
assert_eq!(pow(1.0, i32::MIN), 1.0);    // the most negative exponent works too
```

## Constraints

- `-100.0 < x < 100.0`
- `i32::MIN <= n <= i32::MAX`
- Either `x` is not zero or `n > 0`
- `-10^4 <= x^n <= 10^4`

## Rust notes

- `-n` and `n.abs()` overflow when `n == i32::MIN` (there is no `+2^31` in an
  `i32`) and panic in a debug build. `n.unsigned_abs()` returns the magnitude
  as a `u32`, which always fits.
- On the unsigned exponent, `e & 1` tests the lowest bit and `e >>= 1` halves
  it — shifts on unsigned types are the natural tool here.
- Float multiplication never panics; overflow becomes `inf` and underflow
  becomes `0.0`, so no checked arithmetic is needed on the base.
