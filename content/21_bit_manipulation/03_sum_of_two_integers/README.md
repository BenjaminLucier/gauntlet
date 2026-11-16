## Description

Given two integers `a` and `b`, return their sum **without using the `+` or `-`
operators** (and without `wrapping_add`, `checked_add`, `overflowing_add`,
`Sum`, or any other library function that adds for you). Use bit operations.

If the true sum doesn't fit in an `i32`, the result **wraps around** in two's
complement, exactly like `a.wrapping_add(b)`: `i32::MAX` plus `1` is
`i32::MIN`. The function must not panic for any pair of inputs.

The tests can't detect which operators you used — that part is on your honour.

## Signature

```rust
pub fn add(a: i32, b: i32) -> i32
```

The classic version only promises inputs whose sum fits. Here every pair of
`i32`s is valid and the overflow behaviour is stated up front, so the function
is total: wrapping is what the bit-level adder naturally does, and naming that
semantics is the Rust way (compare `wrapping_add` and `checked_add`).

## Examples

```rust
assert_eq!(add(1, 2), 3);
assert_eq!(add(2, 3), 5);
assert_eq!(add(-5, 3), -2);
assert_eq!(add(i32::MAX, 1), i32::MIN);   // wraps, like i32::wrapping_add
```

## Constraints

- `i32::MIN <= a, b <= i32::MAX` (the classic version uses `-1000..=1000`)

## Rust notes

- `a as u32` / `x as i32` reinterpret the same 32 bits between signed and
  unsigned without changing them (also spelled `a.cast_unsigned()` and
  `x.cast_signed()`). Working on `u32` sidesteps sign questions entirely.
- `<<` on a `u32` simply drops bits shifted past bit 31 — it never panics for
  shift amounts below 32 — so the carry out of the top bit disappears, which
  is exactly two's-complement wrapping.
- `+` on `i32` panics on overflow in a debug build (and in the gauntlet). That
  is why `add(i32::MAX, 1)` has to come out of bit operations, not arithmetic.
