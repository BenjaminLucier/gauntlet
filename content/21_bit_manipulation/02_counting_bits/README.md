## Description

Given `n`, return a vector `ans` of length `n + 1` such that `ans[i]` is the
number of `1` bits in the binary representation of `i`, for every
`0 <= i <= n`.

Can you do it in a single `O(n)` pass, without counting the bits of each number
from scratch (and without `count_ones()`)?

## Signature

```rust
pub fn count_bits(n: usize) -> Vec<u32>
```

`n` determines the length of the output, so it is a `usize`, and the `i` in
`ans[i]` is an index anyway. Bit counts can't be negative; `u32` matches what
`count_ones()` returns.

## Examples

```rust
assert_eq!(count_bits(2), [0, 1, 1]);              // 0, 1, 10
assert_eq!(count_bits(5), [0, 1, 1, 2, 1, 2]);     // 0, 1, 10, 11, 100, 101
assert_eq!(count_bits(0), [0]);
```

## Constraints

- `0 <= n <= 10^5`

## Rust notes

- `vec![0; n + 1]` allocates the whole answer up front; then each entry is
  filled from an earlier one.
- Bit operations work on `usize` like on any unsigned integer: `i >> 1`,
  `i & 1`, `i & (i - 1)`.
- `(i & 1) as u32` converts the low bit for the sum; for a lossless widening
  between integer types, prefer `u32::from(..)` where it's available.
