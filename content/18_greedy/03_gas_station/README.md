## Description

There are `n` gas stations on a circular route. Station `i` has `gas[i]` units
of fuel, and driving from station `i` to station `(i + 1) % n` burns `cost[i]`
units. Your car has an unlimited tank and starts **empty** at a station of your
choice.

Return the index of a starting station from which you can drive around the
whole circuit once (clockwise, back to the start), or `None` if no start works.
With no stations at all there is nowhere to start, so the answer is `None`.

In the classic version of this puzzle the valid start is unique; here, if
several starts work, any one of them is accepted.

## Signature

```rust
pub fn can_complete_circuit(gas: &[u32], cost: &[u32]) -> Option<usize>
```

Fuel amounts can't be negative, so they are `u32`; the answer is an index, so
it is a `usize` — wrapped in `Option` because there may be no valid start.
The two slices must have the same length (the function may panic otherwise).
The *difference* `gas[i] - cost[i]` can be negative, and a running total of
`10^5` of them can exceed `i32`: widen to `i64` before subtracting.

## Examples

```rust
// Start at 3 with 4 fuel. Tank after each leg: 3 → 6 → 4 → 3 → 0.
assert_eq!(can_complete_circuit(&[1, 2, 3, 4, 5], &[3, 4, 5, 1, 2]), Some(3));
// Total gas 9 is less than total cost 10.
assert_eq!(can_complete_circuit(&[2, 3, 4], &[3, 4, 3]), None);
assert_eq!(can_complete_circuit(&[4], &[4]), Some(0)); // arrive with exactly 0
assert_eq!(can_complete_circuit(&[], &[]), None);
```

## Constraints

- `gas.len() == cost.len()`
- `0 <= gas.len() <= 10^5`
- `0 <= gas[i], cost[i] <= 10^4` in the tests (any `u32` must not overflow)

## Rust notes

- `u32 - u32` panics if the result would be negative. Convert first with
  `i64::from(x)` (lossless, unlike `as`), then subtract.
- `gas.iter().zip(cost)` walks the two slices in lockstep; add `enumerate()`
  for the station index.
- `bool::then_some(start)` builds the `Option` from the final feasibility check.
