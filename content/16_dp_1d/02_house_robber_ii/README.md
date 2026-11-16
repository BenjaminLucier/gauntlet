## Description

A row of houses each holds some cash, and robbing two adjacent houses trips
the alarm. Here the houses are arranged in a **circle**: the first and last
house are neighbours. `houses[i]` is the cash in house `i`, and
robbing any two adjacent houses (including the first and last) trips the
alarm. Return the most cash you can collect.

A single house has no neighbour but itself, so it can be robbed; an empty
circle yields `0`.

## Signature

```rust
pub fn rob_circle(houses: &[u32]) -> u64
```

Cash is never negative (`u32`) and only read (borrowed). Sums of up to `10^5`
amounts of up to `u32::MAX` need a `u64`.

## Examples

```rust
assert_eq!(rob_circle(&[2, 3, 2]), 3);    // houses 0 and 2 are neighbours on the circle
assert_eq!(rob_circle(&[1, 2, 3, 1]), 4); // houses 0 and 2: 1 + 3
assert_eq!(rob_circle(&[1, 2, 3]), 3);
assert_eq!(rob_circle(&[5]), 5);          // a lone house is fair game
assert_eq!(rob_circle(&[]), 0);
```

## Constraints

- `0 <= houses.len() <= 10^5`
- `0 <= houses[i] <= u32::MAX`

## Rust notes

- Write the linear robber as a helper over `&[u32]`, then call it on
  sub-slices — slicing is free, no copying.
- Slice patterns make the special cases explicit:
  `match houses { [] => …, [only] => …, [_, rest @ ..] => … }`. Likewise
  `split_first` / `split_last` hand you "everything but one end" as a slice.
- Widen with `u64::from` before adding: two `u32`s can overflow a `u32`.
