## Description

Given a slice of non-negative integers `nums`, return `true` if its elements
can be split into **two groups with equal sums** — every element goes into
exactly one group — and `false` otherwise.

A group may be empty, so an empty slice (two empty groups) and a slice of
zeros both split evenly, while a single positive element never does.

## Signature

```rust
pub fn can_partition(nums: &[u32]) -> bool
```

The values are never negative, so `u32`; the slice is only read, so it is
borrowed.

## Examples

```rust
assert!(can_partition(&[1, 5, 11, 5]));   // [1, 5, 5] and [11] both sum to 11
assert!(!can_partition(&[1, 2, 3, 5]));   // the total, 11, is odd
assert!(!can_partition(&[1, 5]));         // total 6, but no subset sums to 3
assert!(!can_partition(&[2]));            // one group would be empty, the other not
assert!(can_partition(&[]));              // two empty groups
```

## Constraints

- `0 <= nums.len() <= 200`
- `0 <= nums[i] <= 100`

## Rust notes

- A `Vec<bool>` of reachable sums is the classic 1-D table. Iterate sums
  **downwards** — `(x..=target).rev()` — so each element is used at most once;
  upwards would let `x` be reused.
- Sum in a type wide enough for the total and convenient for indexing:
  `nums.iter().map(|&x| x as usize).sum::<usize>()`. The turbofish on `sum` is
  usually needed, because many types implement `Sum`.
- `usize::is_multiple_of(2)` reads better than `% 2 == 0` (and clippy prefers it).
- Returning early once the target becomes reachable is cheap and often saves
  most of the work.
