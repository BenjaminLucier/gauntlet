## Description

Given two slices `a` (length `m`) and `b` (length `n`), each sorted in
non-decreasing order, return the **median** of all `m + n` values combined, or
`None` if both are empty.

The median is the middle value of the combined sorted sequence; when the total
count is even, it is the average of the two middle values (so it may be a
non-integer).

Aim for `O(log(min(m, n)))` time — merging the slices (`O(m + n)`) is the
stepping stone, not the goal.

## Signature

```rust
pub fn median_of_sorted(a: &[i32], b: &[i32]) -> Option<f64>
```

Both inputs are only read, so they are borrowed. Two empty slices have no
median, which is `None` rather than a `NaN` or a panic. The result is an `f64`
because the average of two integers can end in `.5`.

## Examples

```rust
assert_eq!(median_of_sorted(&[1, 3], &[2]), Some(2.0));        // merged: [1, 2, 3]
assert_eq!(median_of_sorted(&[1, 2], &[3, 4]), Some(2.5));     // (2 + 3) / 2
assert_eq!(median_of_sorted(&[], &[7]), Some(7.0));
assert_eq!(median_of_sorted(&[], &[]), None);                  // nothing to take the middle of
assert_eq!(median_of_sorted(&[i32::MAX], &[i32::MAX]), Some(i32::MAX as f64));
```

## Constraints

- `0 <= m, n <= 10^5`
- Values are any `i32`, each slice sorted non-decreasing.

## Rust notes

- The sum of the two middle values can overflow `i32` (`i32::MAX + i32::MAX`):
  widen with `i64::from` before adding, then convert to `f64` and divide.
- `slice.get(k)` returns `None` past the end, and `k.checked_sub(1)` returns
  `None` at the start — handy for treating missing neighbours as -∞ / +∞
  without separate `if`s.
- Swapping the two slice references so the first is the shorter one is a single
  `let (a, b) = if … { (a, b) } else { (b, a) };`.
- With `usize` indices, moving a bound to `i - 1` must only happen when `i > 0`.
