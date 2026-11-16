## Description

Given a signed 32-bit integer `x`, return `x` with its decimal digits reversed
(the sign stays in front). If the reversed value doesn't fit in an `i32`,
return `None`.

**Don't widen to `i64` / `i128`** (or go through strings and parse a wider
type) to detect overflow — detect it in `i32` arithmetic. The tests can't
check this, but it's the point of the exercise.

## Signature

```rust
pub fn reverse(x: i32) -> Option<i32>
```

The classic version returns `0` on overflow, but `0` is also the honest answer
for `x = 0` — a sentinel that collides with a real value. `Option<i32>` keeps
"doesn't fit" separate from every possible result, exactly like the standard
library's `checked_*` methods.

## Examples

```rust
assert_eq!(reverse(123), Some(321));
assert_eq!(reverse(-123), Some(-321));
assert_eq!(reverse(120), Some(21));            // trailing zeros vanish
assert_eq!(reverse(0), Some(0));
assert_eq!(reverse(1_534_236_469), None);      // 9_646_324_351 > i32::MAX
assert_eq!(reverse(i32::MIN), None);           // -8_463_847_412 < i32::MIN
```

## Constraints

- `i32::MIN <= x <= i32::MAX`

## Rust notes

- `i32::checked_mul` and `i32::checked_add` return `None` instead of
  overflowing, and `?` inside a function returning `Option` short-circuits on
  that `None` — the overflow check becomes one line.
- `%` and `/` on a negative `x` give a negative digit and round towards zero
  (`-123 % 10 == -3`, `-123 / 10 == -12`), so the same loop handles negative
  numbers with no special case.
- Don't reach for `abs()` first: `i32::MIN.abs()` overflows and panics in a
  debug build (`unsigned_abs()` doesn't, but then you're juggling two types).
- `overflowing_mul` / `wrapping_mul` exist too, but they compute a wrong value
  and leave the detection to you; `checked_*` says what you mean.
