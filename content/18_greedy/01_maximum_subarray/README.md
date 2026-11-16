## Description

Given a slice of integers, find the **non-empty** contiguous subarray with the
largest sum and return that sum.

An empty slice has no non-empty subarray, so there is no answer: return `None`.

## Signature

```rust
pub fn max_subarray_sum(nums: &[i32]) -> Option<i32>
```

The slice is only read, so it is borrowed. The result is an `Option` because the
empty slice has no answer — just like `Iterator::max` on an empty iterator.
Under the constraints the largest possible sum is `10^5 · 10^4 = 10^9`, which
still fits in an `i32`.

## Examples

```rust
// The subarray [4, -1, 2, 1] sums to 6.
assert_eq!(max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), Some(6));
assert_eq!(max_subarray_sum(&[1]), Some(1));
// The whole slice is the best subarray.
assert_eq!(max_subarray_sum(&[5, 4, -1, 7, 8]), Some(23));
// All negative: the answer is the largest element, not 0.
assert_eq!(max_subarray_sum(&[-3, -1, -2]), Some(-1));
assert_eq!(max_subarray_sum(&[]), None);
```

## Constraints

- `0 <= nums.len() <= 10^5`
- `-10^4 <= nums[i] <= 10^4`

## Rust notes

- `split_first` hands you the first element and the rest as a slice, and
  combined with `?` it turns "empty slice" into `None` in one line.
- `Iterator::scan` carries running state from item to item and yields a value
  for each; `Iterator::fold` (or `max`) can then reduce those values.
- `i32::max` is a function you can pass straight to `fold`:
  `.fold(first, i32::max)`.
- Integer overflow panics in these tests. The constraints keep every sum within
  `i32`, but initialising an accumulator to `i32::MIN` and then adding to it
  will not.
