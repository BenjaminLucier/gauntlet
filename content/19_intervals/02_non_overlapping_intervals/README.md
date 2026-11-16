## Description

Given a list of intervals `(start, end)` with `start < end`, return the
**minimum number of intervals to remove** so that the remaining intervals
don't overlap.

Intervals that only **touch** (`(1, 2)` and `(2, 3)`) do **not** overlap. An
empty list needs no removals.

## Signature

```rust
pub fn min_removals(intervals: Vec<(i32, i32)>) -> usize
```

The answer is a count, so it is a `usize`. The function sorts its input, so it
takes the `Vec` by value and sorts that buffer in place; callers who need the
original keep a clone.

## Examples

```rust
// Remove (1, 3); the rest only touch.
assert_eq!(min_removals(vec![(1, 2), (2, 3), (3, 4), (1, 3)]), 1);
assert_eq!(min_removals(vec![(1, 2), (1, 2), (1, 2)]), 2);
assert_eq!(min_removals(vec![(1, 2), (2, 3)]), 0);
assert_eq!(min_removals(vec![]), 0);
```

## Constraints

- `0 <= intervals.len() <= 10^5`
- `start < end`, any `i32` values (the classic bound is `±5 * 10^4`)

## Rust notes

- `sort_unstable_by_key(|&(_, end)| end)` sorts by one tuple field without
  writing a comparator.
- `Iterator::fold` can carry a small tuple of state — here a count and "where
  the last kept interval ends" — instead of two `let mut`s and a loop.
- Starting the "last end" at `i32::MIN` is a safe sentinel *for a comparison*;
  just never do arithmetic on it.
