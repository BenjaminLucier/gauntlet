## Description

You are given a slice in which every value appears **exactly twice**, except
for one value that appears **exactly once**. Return that single value.

Aim for linear time and constant extra space.

The function is generic: it works for any type that supports `^` (bitwise
XOR) and can be copied — every integer type, `bool`, or your own bitmask types.
For an empty slice there is no single value, and the function returns `None`.

## Signature

```rust
pub fn single_number<T: BitXor<Output = T> + Copy>(nums: &[T]) -> Option<T>
```

The slice is only read, so it is borrowed. `T: Copy` lets the function take
values out of the borrowed slice. `Option` covers the empty slice without
inventing a "zero" value for an arbitrary `T`. If the input breaks the
"everything else appears twice" promise, the result is unspecified (but the
function must not panic).

## Examples

```rust
assert_eq!(single_number(&[2, 2, 1]), Some(1));
assert_eq!(single_number(&[4, 1, 2, 1, 2]), Some(4));
assert_eq!(single_number(&[1]), Some(1));
assert_eq!(single_number(&[7u8, 3, 7]), Some(3));    // any integer type
assert_eq!(single_number::<i32>(&[]), None);
```

## Constraints

- `0 <= nums.len() <= 3 * 10^4`
- Every element appears twice except for exactly one, which appears once
  (when the slice is non-empty)

## Rust notes

- `std::ops::BitXor<Output = T>` is the trait behind `a ^ b`; the
  `Output = T` part says the result has the same type, so you can keep
  folding.
- `Iterator::reduce` is `fold` without an initial value: it returns `None` for
  an empty iterator, which is exactly the `Option` this signature needs — and
  it means you never need a generic "zero".
- `iter().copied()` turns an iterator of `&T` into one of `T` for `Copy` types.
