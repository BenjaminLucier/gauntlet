// Swim in Rising Water — reference solution.
// Minimax Dijkstra: repeatedly pop the lowest reachable cell from a min-heap,
// tracking the highest elevation popped so far. Time O(n² log n), space O(n²).

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn swim_in_water(grid: &[Vec<u32>]) -> u32 {
    let n = grid.len();
    let mut seen = vec![vec![false; n]; n];
    seen[0][0] = true;
    let mut heap = BinaryHeap::from([Reverse((grid[0][0], 0, 0))]);
    let mut level = 0;
    while let Some(Reverse((height, r, c))) = heap.pop() {
        level = level.max(height);
        if (r, c) == (n - 1, n - 1) {
            return level;
        }
        // wrapping_sub turns "row -1" into usize::MAX, which the bounds check rejects.
        let neighbours = [(r.wrapping_sub(1), c), (r + 1, c), (r, c.wrapping_sub(1)), (r, c + 1)];
        for (a, b) in neighbours {
            if a < n && b < n && !seen[a][b] {
                seen[a][b] = true;
                heap.push(Reverse((grid[a][b], a, b)));
            }
        }
    }
    unreachable!("every cell of the grid is reachable eventually")
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
