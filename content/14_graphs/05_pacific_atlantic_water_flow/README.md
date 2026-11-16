## Description

An `m x n` island is described by `heights`, where `heights[r][c]` is the
height of the cell at row `r`, column `c`. The **Pacific Ocean** touches the
island's top and left edges; the **Atlantic Ocean** touches its bottom and
right edges.

Rain water flows from a cell to a 4-directionally adjacent cell whose height is
**less than or equal to** its own. Water in any cell on an edge can flow
directly into the ocean bordering that edge.

Return the coordinates `(row, col)` of every cell from which water can reach
**both** oceans, each exactly once. The order of the result does not matter.
An empty map has no such cells.

## Signature

```rust
pub fn pacific_atlantic(heights: &[Vec<u32>]) -> Vec<(usize, usize)>
```

The map is only read, so it is borrowed. Heights can't be negative, so they
are `u32`; coordinates are indices, so they are `(usize, usize)` pairs rather
than two-element vectors.

## Examples

```rust
let heights = [
    vec![1, 2, 2, 3, 5],
    vec![3, 2, 3, 4, 4],
    vec![2, 4, 5, 3, 1],
    vec![6, 7, 1, 4, 5],
    vec![5, 1, 1, 2, 4],
];
let mut cells = pacific_atlantic(&heights);
cells.sort();
// e.g. from (2, 2) (height 5) water flows up to the Pacific (5 -> 3 -> 2)
// and right to the Atlantic (5 -> 3 -> 1).
assert_eq!(cells, [(0, 4), (1, 3), (1, 4), (2, 2), (3, 0), (3, 1), (4, 0)]);

assert_eq!(pacific_atlantic(&[vec![1]]), [(0, 0)]); // touches every edge
assert_eq!(pacific_atlantic(&[]), []);
```

## Constraints

- `0 <= m, n <= 200` (all rows have the same length)
- `0 <= heights[r][c] <= 10^5`

## Rust notes

- A `VecDeque` (or a plain `Vec` used as a stack) gives an iterative search; a
  recursive DFS over 40,000 cells risks overflowing the thread stack.
- Neighbour coordinates are `usize`: `r - 1` at `r == 0` panics. Use
  `r.checked_sub(1)`, or `r.wrapping_sub(1)` followed by a `< m` bounds check
  (the wrapped value is huge, so the check rejects it).
- A helper that takes an iterator of seed cells (`impl IntoIterator<Item =
  (usize, usize)>`) lets you run the same search for both oceans.
- `iter().zip()` over two `Vec<Vec<bool>>` grids, or `flat_map` over row and
  column ranges, builds the result without index juggling.
