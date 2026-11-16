## Description

You start at index `0` of a slice `jumps`. From index `i` you may jump to any
index in `i + 1 ..= i + jumps[i]` (staying inside the slice).

Return the **minimum number of jumps** needed to reach the last index, or
`None` if the last index can't be reached at all. A single-element (or empty)
slice needs `Some(0)` jumps.

## Signature

```rust
pub fn min_jumps(jumps: &[usize]) -> Option<usize>
```

Jump lengths and the jump count are non-negative quantities measured against
indices, so both are `usize`. "Unreachable" is `None` rather than a sentinel
like `-1`.

## Examples

```rust
// Jump 1 step to index 1, then 3 steps to the last index.
assert_eq!(min_jumps(&[2, 3, 1, 1, 4]), Some(2));
assert_eq!(min_jumps(&[2, 3, 0, 1, 4]), Some(2));
assert_eq!(min_jumps(&[7]), Some(0));          // already there
// Every path gets stuck on index 3.
assert_eq!(min_jumps(&[3, 2, 1, 0, 4]), None);
```

## Constraints

- `0 <= jumps.len() <= 10^4`
- `0 <= jumps[i]` — any `usize`, including `usize::MAX`

## Rust notes

- `saturating_add` keeps `i + jumps[i]` from overflowing when a jump is huge.
- `bool::then_some` turns "did we make it?" into the `Option` you return.
- `jumps.len().saturating_sub(1)` gives the last index without a special case
  for the empty slice (`len() - 1` would panic).
- `iter().enumerate().take(n)` walks indices and values together and stops
  early — the last index never needs to jump.
