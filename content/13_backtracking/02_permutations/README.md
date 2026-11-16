## Description

Given a slice of **distinct** items, return all possible orderings
(permutations) of them. The permutations may be returned in **any order**, but
each one is itself an ordered list.

The function is generic over the item type.

## Signature

```rust
pub fn permutations<T: Clone>(items: &[T]) -> Vec<Vec<T>>
```

The input is borrowed; every permutation in the output owns its items, hence
`T: Clone`. Nothing else is required — items are never compared, since they're
distinct by assumption.

## Examples

```rust
let mut all = permutations(&[1, 2, 3]);
all.sort();
assert_eq!(
    all,
    vec![vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3], vec![2, 3, 1], vec![3, 1, 2], vec![3, 2, 1]]
);
assert_eq!(permutations(&[1]), vec![vec![1]]);
assert_eq!(permutations::<u8>(&[]), vec![vec![]]);   // 0! = 1: one empty ordering
assert_eq!(permutations(&['a', 'b']).len(), 2);
```

## Constraints

- `0 <= items.len() <= 6`
- All items are distinct.

## Rust notes

- `items.to_vec()` gives you a private, mutable working copy; `slice::swap`
  then permutes it in place without any extra bookkeeping.
- The "used" approach needs a `Vec<bool>` plus a `&mut Vec<T>` path —
  `push`, recurse, `pop`, and reset the flag.
- Record a permutation with `.clone()` (or `.to_vec()` on a slice) — you need
  a snapshot, not the buffer you're still mutating.
- `Vec::with_capacity` is cheap insurance when you know the output size
  (`(1..=n).product()`).
