## Description

You are given the heights of `n` vertical lines: line `i` goes from `(i, 0)`
up to `(i, heights[i])`.

Choose two lines that, together with the x-axis, form a container. Its
capacity is `(j - i) * min(heights[i], heights[j])`. Return the **maximum**
amount of water any such container can hold. (The container can't be tilted,
and the lines in between don't matter.) With fewer than two lines there is no
container, and the answer is `0`.

## Signature

```rust
pub fn max_area(heights: &[u32]) -> u64
```

Heights can't be negative, so they're `u32`. The area is width × height,
which can reach about `10^5 * 4.3 * 10^9 ≈ 4.3 * 10^14` — far beyond `u32`,
so the result is widened to `u64`.

## Examples

```rust
assert_eq!(max_area(&[1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);   // lines 1 and 8: 7 * min(8, 7)
assert_eq!(max_area(&[1, 1]), 1);
assert_eq!(max_area(&[4]), 0);                             // one line holds nothing
assert_eq!(max_area(&[u32::MAX, 0, u32::MAX]), 2 * u64::from(u32::MAX));
```

## Constraints

- `0 <= heights.len() <= 10^5`
- `0 <= heights[i] <= u32::MAX`

## Rust notes

- Convert *before* multiplying: `width as u64 * u64::from(h)`. Multiplying
  in `u32` (or `usize` on a 32-bit target) and converting afterwards is too
  late — the overflow has already panicked.
- `u64::from(x)` is a lossless conversion that the compiler checks; prefer it
  to `as` when widening.
- `heights.len() - 1` underflows on an empty slice; guard first or use an
  exclusive right bound.
- `Ord::min` / `Ord::max` (`a.min(b)`) keep the update lines short.
