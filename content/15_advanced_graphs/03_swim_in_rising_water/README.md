## Description

You are given an `n x n` grid where `grid[r][c]` is the elevation of that cell.
Every value from `0` to `n² - 1` appears exactly once.

Rain starts falling, and at time `t` the water everywhere is at depth `t`. You
can swim from a cell to a 4-directionally adjacent cell only if **both** cells
have elevation at most `t`. Swimming takes no time at all.

Starting at the top-left cell `(0, 0)`, return the least time `t` at which you
can reach the bottom-right cell `(n - 1, n - 1)`.

## Signature

```rust
pub fn swim_in_water(grid: &[Vec<u32>]) -> u32
```

The grid is only read, so it is borrowed as a slice of rows. Elevations and
times are never negative, so both are `u32`.

## Examples

```rust
// Until t = 3 the target cell (elevation 3) is above water.
assert_eq!(swim_in_water(&[vec![0, 2], vec![1, 3]]), 3);

// Follow the top row and the 5, cross 16 and go down the right side; the
// highest cell on that route is 16, and no route avoids something as high.
let grid = [
    vec![0, 1, 2, 3, 4],
    vec![24, 23, 22, 21, 5],
    vec![12, 13, 14, 15, 16],
    vec![11, 17, 18, 19, 20],
    vec![10, 9, 8, 7, 6],
];
assert_eq!(swim_in_water(&grid), 16);

// You can't leave the start until it is under water.
assert_eq!(swim_in_water(&[vec![3, 0], vec![1, 2]]), 3);
```

## Constraints

- `1 <= n <= 50`
- `grid` is square: `grid.len() == n` and every row has length `n`
- `0 <= grid[r][c] < n²`, all values distinct

## Rust notes

- `BinaryHeap` is a max-heap; wrap entries in `std::cmp::Reverse` —
  `Reverse((elevation, row, col))` — to pop the lowest cell first. Tuples
  compare lexicographically, so the elevation must come first.
- Neighbour coordinates are `usize`: `r - 1` at row 0 panics. Either
  `r.checked_sub(1)`, or `r.wrapping_sub(1)` followed by a single `< n` bounds
  check (the wrapped value is huge, so it fails the check).
- Mark a cell as seen when you *push* it, not when you pop it, or the heap can
  fill with duplicates.
- `u32::max` / `Ord::max` keeps a running maximum without an `if`.
