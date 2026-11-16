// Rotting Oranges — reference solution.
// Multi-source BFS from all rotten oranges on a copy of the grid, one level per
// minute, counting down the fresh oranges. Time O(m*n), space O(m*n).

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Fresh,
    Rotten,
}

pub fn oranges_rotting(grid: &[Vec<Cell>]) -> Option<usize> {
    let mut grid = grid.to_vec();
    let m = grid.len();
    let n = grid.first().map_or(0, Vec::len);
    let mut queue = VecDeque::new();
    let mut fresh = 0usize;
    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            match cell {
                Cell::Fresh => fresh += 1,
                Cell::Rotten => queue.push_back((r, c)),
                Cell::Empty => {}
            }
        }
    }
    let mut minutes = 0;
    while fresh > 0 && !queue.is_empty() {
        minutes += 1;
        for _ in 0..queue.len() {
            let (r, c) = queue.pop_front().expect("loop runs queue.len() times");
            let neighbours = [
                (r.wrapping_sub(1), c),
                (r + 1, c),
                (r, c.wrapping_sub(1)),
                (r, c + 1),
            ];
            for (a, b) in neighbours {
                if a < m && b < n && grid[a][b] == Cell::Fresh {
                    grid[a][b] = Cell::Rotten;
                    fresh -= 1;
                    queue.push_back((a, b));
                }
            }
        }
    }
    (fresh == 0).then_some(minutes)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
