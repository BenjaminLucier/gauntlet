## Description

Given a collection of positive integers `candidates` (which **may contain
duplicates**) and a `target`, return every unique combination of candidates
that sums to `target`. Each element of `candidates` may be used **at most
once** — but equal values at different positions are separate elements.

The result must not contain the same combination twice: with
`candidates = [1, 1, 2]` and `target = 3`, `[1, 2]` appears only once even
though either `1` could be used. Return combinations in **any order**;
numbers inside a combination may be in any order.

## Signature

```rust
pub fn combination_sum_ii(candidates: &[u32], target: u32) -> Vec<Vec<u32>>
```

All values are positive, so they're `u32`. The candidates are borrowed: the
caller keeps their slice untouched, and the function sorts its own copy.

## Examples

```rust
let sorted = |mut v: Vec<Vec<u32>>| { v.iter_mut().for_each(|c| c.sort()); v.sort(); v };
assert_eq!(
    sorted(combination_sum_ii(&[10, 1, 2, 7, 6, 1, 5], 8)),
    vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
);
assert_eq!(sorted(combination_sum_ii(&[2, 5, 2, 1, 2], 5)), vec![vec![1, 2, 2], vec![5]]);
assert_eq!(combination_sum_ii(&[1, 1, 2], 3).len(), 1);   // [1, 2] only once
assert!(combination_sum_ii(&[2], 4).is_empty());          // the single 2 can't be reused
```

## Constraints

- `1 <= candidates.len() <= 100`
- `1 <= candidates[i] <= 50`
- `1 <= target <= 30`

## Rust notes

- Backtrack with a `&mut Vec<u32>` path: `push`, recurse, `pop`.
- Compare before subtracting: `remaining - c` on `u32` panics on underflow.
- Recursing on a subslice (`&sorted[i + 1..]`) instead of passing a start index
  makes "each element at most once" structural.
- Don't dedupe afterwards with a `HashSet<Vec<u32>>` — the duplicate branches
  are the expensive part, and the set doesn't stop you exploring them.
