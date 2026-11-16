## Description

You are given a slice `nums` and a window size `k`. A window of `k`
consecutive elements slides from the very left of the slice to the very
right, one position at a time.

Return the maximum of each window, in order. There are `nums.len() - k + 1`
windows when `k <= nums.len()`, and none (an empty result) when `k` is larger
than the slice.

The function is generic: it works for any totally ordered, copyable item —
integers, `char`s, tuples, …

## Signature

```rust
pub fn max_sliding_window<T: Ord + Copy>(nums: &[T], k: NonZeroUsize) -> Vec<T>
```

A window of size zero has no maximum, so `k` is a `NonZeroUsize`: the type
rules out the bad case, and the function doesn't need a panic or an error for
it. (`std::slice::windows` takes a plain `usize` and panics on `0` instead.)
`T: Copy` lets the result hold the maximum values themselves.

## Examples

```rust
use std::num::NonZeroUsize;
let k = |n| NonZeroUsize::new(n).expect("window size is non-zero");

assert_eq!(max_sliding_window(&[1, 3, -1, -3, 5, 3, 6, 7], k(3)), vec![3, 3, 5, 5, 6, 7]);
//   [1  3  -1] -3  5  3  6  7   -> 3
//    1 [3  -1  -3] 5  3  6  7   -> 3
//    1  3 [-1  -3  5] 3  6  7   -> 5
//    1  3  -1 [-3  5  3] 6  7   -> 5
//    1  3  -1  -3 [5  3  6] 7   -> 6
//    1  3  -1  -3  5 [3  6  7]  -> 7
assert_eq!(max_sliding_window(&[1], k(1)), vec![1]);
assert_eq!(max_sliding_window(&['r', 'u', 's', 't'], k(2)), vec!['u', 'u', 't']);
assert!(max_sliding_window(&[1, 2], k(3)).is_empty());   // window bigger than the slice
```

## Constraints

- `0 <= nums.len() <= 10^5`
- `1 <= k` (any size; larger than `nums.len()` gives no windows)

## Rust notes

- `std::collections::VecDeque` gives `O(1)` push and pop at both ends,
  plus `front()` / `back()` that return `Option<&T>`.
- `while deque.back().is_some_and(|&j| …)` pops while a condition holds
  without a separate emptiness check.
- `k.get()` turns a `NonZeroUsize` back into a `usize`.
- `Vec::with_capacity(n.saturating_sub(k - 1))` sizes the output without
  underflowing when `k > n`.
- `k` can be as large as `usize::MAX`, so `j + k` can overflow; compare
  distances (`i - j >= k`) instead.
- `nums.windows(k)` with `iter().max()` is a correct `O(n * k)` baseline.
