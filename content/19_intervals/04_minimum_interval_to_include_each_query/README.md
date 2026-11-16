## Description

You are given `intervals`, where each `(left, right)` is the inclusive range
`left..=right`, whose **size** is `right - left + 1`. You are also given a
slice of `queries`.

For each query `q`, find the size of the **smallest** interval that contains
`q` (`left <= q <= right`), or `None` if no interval contains it. Return the
answers in the same order as `queries`.

## Signature

```rust
pub fn min_interval_sizes(intervals: Vec<(i32, i32)>, queries: &[i32]) -> Vec<Option<u32>>
```

A size is a positive count, so it is a `u32` (`i32::abs_diff` produces one
directly), and "no interval contains this query" is `None` rather than `-1`.
The intervals get sorted, so they are taken by value; the queries must keep
their order and are only read, so they are borrowed.

## Examples

```rust
// q = 2 and q = 3: (2, 4) has size 3; q = 4: (4, 4) has size 1;
// q = 5: (3, 6) has size 4.
assert_eq!(
    min_interval_sizes(vec![(1, 4), (2, 4), (3, 6), (4, 4)], &[2, 3, 4, 5]),
    [Some(3), Some(3), Some(1), Some(4)]
);
// No interval contains 19.
assert_eq!(
    min_interval_sizes(vec![(2, 3), (2, 5), (1, 8), (20, 25)], &[2, 19, 5, 22]),
    [Some(2), None, Some(4), Some(6)]
);
assert_eq!(min_interval_sizes(vec![], &[1, 2]), [None, None]);
```

## Constraints

- `0 <= intervals.len() <= 10^5`
- `0 <= queries.len() <= 10^5`
- `1 <= left <= right <= 10^7`
- `1 <= queries[j] <= 10^7`

## Rust notes

- Sort query *indices* rather than the queries themselves:
  `order.sort_unstable_by_key(|&i| queries[i])` lets you answer in sorted
  order and write each answer to its original slot.
- `BinaryHeap<Reverse<(u32, i32)>>` is a min-heap ordered by size first, then
  by right end — tuples compare lexicographically.
- `Peekable::next_if` consumes the next sorted interval only while it starts
  at or before the current query.
- `vec![None; n]` builds the answer vector; `heap.peek().map(…)` turns "heap
  top, if any" into the `Option` answer.
