## Description

Given a list of closed intervals `(start, end)` in no particular order, merge
all overlapping intervals and return the resulting non-overlapping intervals,
which together cover exactly the same points as the input. Intervals that
merely **touch** (`(1, 4)` and `(4, 5)`) count as overlapping.

Return the merged intervals **sorted by start**. An empty input gives an empty
output.

## Signature

```rust
pub fn merge_intervals(intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)>
```

The function takes the `Vec` by value: it sorts and compacts that very buffer
and hands it back, so no second allocation is needed. A caller who still needs
the original can pass `intervals.clone()` — the choice is theirs.

## Examples

```rust
// (1, 3) and (2, 6) overlap.
assert_eq!(
    merge_intervals(vec![(1, 3), (2, 6), (8, 10), (15, 18)]),
    [(1, 6), (8, 10), (15, 18)]
);
// Touching intervals are merged.
assert_eq!(merge_intervals(vec![(1, 4), (4, 5)]), [(1, 5)]);
// The input isn't sorted.
assert_eq!(merge_intervals(vec![(4, 7), (1, 4)]), [(1, 7)]);
assert_eq!(merge_intervals(vec![]), []);
```

## Constraints

- `0 <= intervals.len() <= 10^4`
- `-10^4 <= start <= end <= 10^4`

## Rust notes

- `sort_unstable_by_key(|&(start, _)| start)` — or plain `sort_unstable()`,
  since tuples compare lexicographically.
- `Vec::dedup_by(|next, kept| …)` walks neighbours and drops `next` when the
  closure returns `true` — and `kept` is a `&mut`, so the closure can widen it.
  That's merging in place.
- Equivalently, build a new `Vec` and coalesce into `merged.last_mut()`: a
  `match` with a guard (`Some(last) if start <= last.1 => …`) reads well.
