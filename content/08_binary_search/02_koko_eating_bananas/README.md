## Description

There are `n` piles of bananas; pile `i` holds `piles[i]` bananas. Koko has `h`
hours before the guards return.

She picks an integer eating speed `k` (bananas per hour). Each hour she chooses
one pile and eats `k` bananas from it; if the pile has fewer than `k` left she
finishes it and does nothing else that hour. So a pile of `p` bananas takes
`ceil(p / k)` hours.

Return the **minimum** speed `k >= 1` that lets her finish every pile within
`h` hours, or `None` if no speed is fast enough — every pile takes at least one
hour, so that happens exactly when `h < n`. With no piles at all, speed `1`
will do.

## Signature

```rust
pub fn min_eating_speed(piles: &[u32], h: u64) -> Option<u32>
```

Pile sizes and speeds are never negative, so they are unsigned. `h` is a `u64`
because the hours needed can add up to far more than a `u32` holds (at speed
`1` it is the total number of bananas). An impossible request returns `None`
instead of a sentinel.

## Examples

```rust
// At k = 4 she needs 1 + 2 + 2 + 3 = 8 hours; at k = 3 she would need 10.
assert_eq!(min_eating_speed(&[3, 6, 7, 11], 8), Some(4));
assert_eq!(min_eating_speed(&[30, 11, 23, 4, 20], 5), Some(30));
assert_eq!(min_eating_speed(&[30, 11, 23, 4, 20], 6), Some(23));
assert_eq!(min_eating_speed(&[30, 11, 23, 4, 20], 4), None); // 5 piles, 4 hours
assert_eq!(min_eating_speed(&[], 0), Some(1));
```

## Constraints

- `0 <= piles.len() <= 10^4`
- `1 <= piles[i] <= u32::MAX`
- `0 <= h <= u64::MAX`

## Rust notes

- `u32::div_ceil` computes `ceil(p / k)` directly. The classic
  `(p + k - 1) / k` overflows when `p` is near `u32::MAX` — and with overflow
  checks on, that's a panic, not a wrong answer.
- Sum the hours as `u64`: `.map(|&p| u64::from(p.div_ceil(k))).sum::<u64>()`.
- `iter().max()` returns `Option<&u32>`: an empty slice has no maximum, so
  decide what the search range is in that case.
