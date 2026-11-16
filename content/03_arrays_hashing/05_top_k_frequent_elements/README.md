## Description

Given a slice of items and a count `k`, return the `k` distinct values that
occur most often. The result may be in **any order**.

Inputs are chosen so the answer is unique: there is never a tie across the
`k`-th place. If `k` is at least the number of distinct values, return every
distinct value; if `k == 0` (or the slice is empty), return nothing.

The function is generic: it counts anything hashable — numbers, words, tuples.

## Signature

```rust
pub fn top_k_frequent<T: Hash + Eq + Clone>(items: &[T], k: usize) -> Vec<T>
```

The items are borrowed; the result holds `k` owned copies of the winning
values, which is why `T: Clone` is required (cloning `k` values is cheap next
to counting `n`). `k` is a count, so it's a `usize`.

## Examples

```rust
let mut top = top_k_frequent(&[1, 1, 1, 2, 2, 3], 2);
top.sort_unstable();                       // any order is accepted
assert_eq!(top, vec![1, 2]);               // 1 occurs 3 times, 2 twice, 3 once

assert_eq!(top_k_frequent(&[1], 1), vec![1]);
assert_eq!(top_k_frequent(&["to", "be", "or", "not", "to", "be", "to"], 1), vec!["to"]);
assert_eq!(top_k_frequent(&[7, 7, 8], 5).len(), 2);   // only 2 distinct values
assert!(top_k_frequent::<i32>(&[], 3).is_empty());
```

## Constraints

- `0 <= items.len() <= 10^5`
- `0 <= k`
- The answer is unique (no tie at the `k`-th place).

## Rust notes

- Count with `HashMap<&T, usize>` — keys can be references into `items`, so
  nothing is cloned until the very end.
- `*counts.entry(item).or_insert(0) += 1` is the standard counting idiom.
- `vec![Vec::new(); n + 1]` builds a vector of empty buckets.
- An iterator chain like `.rev().flatten().take(k).cloned().collect()` reads
  the buckets from the most frequent down and stops after `k` values.
- `Reverse` from `std::cmp` turns `BinaryHeap` into a min-heap if you go the
  heap route.
