## Description

Given the heights of `n` bars in an elevation map where every bar has width
`1`, compute how many units of water are trapped after it rains.

The water above bar `i` rises to `min(highest bar to the left, highest bar to
the right)` (each including bar `i` itself), minus `heights[i]`. An empty map
traps nothing.

## Signature

```rust
pub fn trap(heights: &[u32]) -> u64
```

Heights are never negative, so they're `u32`. The total can reach roughly
`2 * 10^4 * 4.3 * 10^9 ≈ 8.6 * 10^13`, so it's accumulated in a `u64`.

## Examples

```rust
assert_eq!(trap(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
assert_eq!(trap(&[4, 2, 0, 3, 2, 5]), 9);
assert_eq!(trap(&[3, 0, 3]), 3);
assert_eq!(trap(&[1, 2, 3]), 0);                          // water runs off
assert_eq!(trap(&[]), 0);
assert_eq!(trap(&[u32::MAX, 0, 0, u32::MAX]), 2 * u64::from(u32::MAX));
```

## Constraints

- `0 <= heights.len() <= 2 * 10^4`
- `0 <= heights[i] <= u32::MAX`

## Rust notes

- With unsigned types, `a - b` panics if `b > a`. Arrange the computation so
  you only ever subtract a height from a maximum that already includes it
  (or reach for `saturating_sub` if you really mean "at least zero").
- Widen each bar's water to `u64` (`u64::from(...)`) before adding it to the
  total.
- `heights.len() - 1` underflows on an empty slice — guard first, or keep an
  exclusive right bound.
- The `O(n)`-space version reads nicely with iterators: `scan` for running
  maxima, `zip` to combine the two directions.
