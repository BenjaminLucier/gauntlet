## Description

Given a slice of items that **may contain duplicates**, return every possible
subset (the power set) without repeating any subset. Two subsets are the same
if they contain each value the same number of times — with `[1, 2, 2]`,
choosing either `2` gives the single subset `[1, 2]`.

Subsets may be returned in **any order**; items inside a subset may be in any
order.

## Signature

```rust
pub fn subsets_with_dup<T: Ord + Clone>(items: &[T]) -> Vec<Vec<T>>
```

Unlike plain subsets, this one has to know when two items are equal, and the
natural way to group equal items is to sort — hence `T: Ord`. `Clone` is needed
because each subset owns its items. The input is borrowed; sort a private copy.

## Examples

```rust
let mut all = subsets_with_dup(&[1, 2, 2]);
all.sort();
assert_eq!(all, vec![vec![], vec![1], vec![1, 2], vec![1, 2, 2], vec![2], vec![2, 2]]);
assert_eq!(subsets_with_dup(&[0]).len(), 2);                 // [] and [0]
assert_eq!(subsets_with_dup(&[5, 5, 5]).len(), 4);           // 0, 1, 2 or 3 fives
assert_eq!(subsets_with_dup::<i32>(&[]), vec![vec![]]);
assert_eq!(subsets_with_dup(&["b", "a", "b"]).len(), 6);
```

## Constraints

- `0 <= items.len() <= 10`

## Rust notes

- `let mut sorted = items.to_vec(); sorted.sort_unstable();` — equal items end
  up adjacent, and the caller's slice is untouched.
- Backtracking with a `&mut Vec<T>` path: `push(x.clone())`, recurse, `pop()`.
- Recursing on subslices (`&sorted[i + 1..]`) keeps index arithmetic out of the
  way; `i > 0 && sorted[i] == sorted[i - 1]` is then "same value already tried
  at this depth".
- `slice::chunk_by` (stable since 1.77) groups runs of equal items, which fits
  the "take this value 0..=c times" formulation.
