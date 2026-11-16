## Description

Given a slice of items, return the length of its longest **strictly
increasing subsequence**. A subsequence keeps the original order but may skip
elements; it does not need to be contiguous. An empty slice has an empty
longest subsequence, of length `0`.

The function is generic: anything totally ordered works — integers, strings,
tuples (compared lexicographically), your own `Ord` types.

## Signature

```rust
pub fn length_of_lis<T: Ord>(items: &[T]) -> usize
```

The algorithm only ever compares items, so `T: Ord` is all it needs — no
`Copy`, no `Clone`. The input is borrowed, and the result is a length, so
`usize`.

## Examples

```rust
assert_eq!(length_of_lis(&[10, 9, 2, 5, 3, 7, 101, 18]), 4); // e.g. 2, 3, 7, 101
assert_eq!(length_of_lis(&[0, 1, 0, 3, 2, 3]), 4);
assert_eq!(length_of_lis(&[7, 7, 7, 7]), 1);                 // equal isn't increasing
assert_eq!(length_of_lis(&["pear", "apple", "fig", "kiwi"]), 3); // apple, fig, kiwi
assert_eq!(length_of_lis::<i32>(&[]), 0);
```

## Constraints

- `0 <= items.len() <= 10^5`

## Rust notes

- `slice::partition_point(|t| t < x)` is a binary search for the first
  position whose element is **not** less than `x` — exactly the "lower bound"
  that patience sorting needs, with no `Ok`/`Err` to untangle as with
  `binary_search` (which is also ambiguous among equal elements).
- Your working vector can hold references, `Vec<&T>`, so items never need to be
  cloned; comparisons between `&T`s use `T`'s ordering.
- Strictness lives in the comparison: `t < x` versus `t <= x` is the
  difference between "increasing" and "non-decreasing".
- `Vec::with_capacity(items.len())` avoids regrowth, though the vector rarely
  gets that long.
