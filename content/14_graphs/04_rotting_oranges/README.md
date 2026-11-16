## Description

Each cell of an `m x n` crate is empty, holds a fresh orange, or holds a rotten
orange. Every minute, each fresh orange that is 4-directionally adjacent to a
rotten orange becomes rotten.

Return the minimum number of minutes until no fresh orange remains, or `None`
if some fresh orange can never rot.

## Signature

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Fresh,
    Rotten,
}

pub fn oranges_rotting(grid: &[Vec<Cell>]) -> Option<usize>
```

A three-variant enum instead of the classic `0` / `1` / `2` integers: the
names say what the cells are, and a `match` must cover all three. The grid is
borrowed — the caller's crate is a description of the starting state, so
simulate on your own copy or track rot in a separate buffer. "Impossible" is
`None` rather than `-1`; minutes are a count, so `usize`.

## Examples

```rust
use Cell::{Empty as E, Fresh as F, Rotten as R};

assert_eq!(oranges_rotting(&[vec![R, F, F], vec![F, F, E], vec![E, F, F]]), Some(4));
// The bottom-left orange has no neighbour that can ever rot it.
assert_eq!(oranges_rotting(&[vec![R, F, F], vec![E, F, F], vec![F, E, F]]), None);
// Nothing fresh at minute 0: zero minutes, even with nothing rotten either.
assert_eq!(oranges_rotting(&[vec![E, R]]), Some(0));
assert_eq!(oranges_rotting(&[vec![E]]), Some(0));
```

## Constraints

- `0 <= m, n <= 100`; all rows have the same length

## Rust notes

- `VecDeque` holds the BFS frontier. To advance one minute at a time, either
  drain exactly `queue.len()` entries per round or store the minute in each
  queue entry.
- `grid.to_vec()` gives you an owned, mutable copy to simulate on — `Cell` is
  `Copy`, so that's a cheap memcpy of each row.
- Neighbour coordinates are `usize`: `wrapping_sub(1)` plus one bounds check
  (or `checked_sub`) handles the top and left edges.
- `(fresh == 0).then_some(minutes)` turns the final check into an `Option`.
