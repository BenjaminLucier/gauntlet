## Description

Given an unsorted slice of integers `nums`, return the length of the longest
run of **consecutive integers** (`x, x+1, x+2, …`) whose values all appear in
`nums`. The values may appear anywhere in the slice, in any order, and
duplicates count once.

Aim for `O(n)` expected time.

## Signature

```rust
pub fn longest_consecutive(nums: &[i32]) -> usize
```

The slice is only read, so it's borrowed. The answer is a length, so it's a
`usize` — and it can be `0` (for an empty slice) without any special case.

## Examples

```rust
assert_eq!(longest_consecutive(&[100, 4, 200, 1, 3, 2]), 4);           // 1, 2, 3, 4
assert_eq!(longest_consecutive(&[0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);   // 0 through 8
assert_eq!(longest_consecutive(&[1, 0, 1, 2]), 3);                      // duplicates count once
assert_eq!(longest_consecutive(&[]), 0);
assert_eq!(longest_consecutive(&[i32::MAX - 1, i32::MAX]), 2);          // no overflow at the top
```

## Constraints

- `0 <= nums.len() <= 10^5`
- `i32::MIN <= nums[i] <= i32::MAX` — the whole `i32` range

## Rust notes

- `x + 1` overflows (and panics, with overflow checks on) when
  `x == i32::MAX`, and `x - 1` when `x == i32::MIN`. `checked_add` /
  `checked_sub` return `Option`, which slots neatly into a lookup.
- `nums.iter().copied().collect::<HashSet<i32>>()` builds the set in one line.
- Iterate over the *set*, not the slice: duplicates in the slice would repeat
  work.
- `Iterator::max()` returns an `Option`; `unwrap_or(0)` handles the empty case.
