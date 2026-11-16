## Description

An ascending slice of **distinct** items may have been rotated at an unknown
pivot `k` (`0 <= k < n`), turning `[a0, a1, ..., a(n-1)]` into
`[ak, ..., a(n-1), a0, ..., a(k-1)]`. For example `[0, 1, 2, 4, 5, 6, 7]`
rotated at pivot 3 becomes `[4, 5, 6, 7, 0, 1, 2]`.

Given the rotated slice and a `target`, return the index of `target`, or `None`
if it is absent. Your algorithm must run in `O(log n)` time.

## Signature

```rust
pub fn search_rotated<T: Ord>(nums: &[T], target: &T) -> Option<usize>
```

Like a plain binary search, this only compares items, so it is generic over
`T: Ord` and borrows both the slice and the target. An index is a `usize`, and
"absent" is `None`.

## Examples

```rust
assert_eq!(search_rotated(&[4, 5, 6, 7, 0, 1, 2], &0), Some(4));
assert_eq!(search_rotated(&[4, 5, 6, 7, 0, 1, 2], &3), None);
assert_eq!(search_rotated(&[1], &0), None);
assert_eq!(search_rotated(&['x', 'y', 'z', 'a', 'b'], &'a'), Some(3));
assert_eq!(search_rotated::<i32>(&[], &5), None);
```

## Constraints

- `0 <= nums.len() <= 10^5`
- All items are distinct; `nums` is an ascending slice, possibly rotated.

## Rust notes

- A half-open window `[lo, hi)` sidesteps the `mid - 1` underflow that a closed
  window hits at `mid == 0` with `usize` indices.
- The target is a `&T` and slice elements are `T`: compare with `nums[i] < *target`
  or `&nums[i] < target` — both work, just be consistent.
- `(a..b).contains(&x)` reads nicely for half-open range checks, and works on
  references to any `PartialOrd` type.
