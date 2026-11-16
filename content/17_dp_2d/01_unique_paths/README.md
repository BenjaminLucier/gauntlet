## Description

A robot stands in the top-left cell of a grid with `rows` rows and `cols`
columns. On each move it can go either **one cell right** or **one cell down**.
It wants to reach the bottom-right cell.

Return the number of distinct paths the robot can take, or `None` if that
number is too large to fit in a `u64`. A grid with no cells (`rows == 0` or
`cols == 0`) has no paths at all.

## Signature

```rust
pub fn unique_paths(rows: usize, cols: usize) -> Option<u64>
```

Grid dimensions are sizes, so they are `usize`. The number of paths grows
exponentially (a `35 x 35` grid already has more than `u64::MAX`), so instead of
quietly wrapping or panicking the function reports overflow as `None`.

## Examples

```rust
assert_eq!(unique_paths(3, 7), Some(28));
// Right -> Down -> Down, Down -> Down -> Right, Down -> Right -> Down
assert_eq!(unique_paths(3, 2), Some(3));
assert_eq!(unique_paths(1, 1), Some(1));          // already there
assert_eq!(unique_paths(0, 5), Some(0));          // no grid, no paths
assert_eq!(unique_paths(100, 100), None);         // C(198, 99) > u64::MAX
```

## Constraints

- `0 <= rows, cols`
- `rows * cols <= 10^6`

## Rust notes

- `u64::checked_add` returns `None` on overflow, and `?` propagates that `None`
  straight out of a function returning `Option`.
- Path counts only grow towards the bottom-right corner, so if any cell
  overflows, the answer does too — you may stop at the first overflow.
- `vec![1u64; cols]` builds the first row; iterating `&mut row` with a running
  total avoids index arithmetic entirely.
- Under `overflow-checks` (as in the tests) a plain `+` that overflows panics
  instead of returning `None`.
