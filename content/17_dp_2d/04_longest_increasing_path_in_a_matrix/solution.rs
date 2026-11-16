// Longest Increasing Path in a Matrix — reference solution.
// Process cells from largest to smallest value: best[cell] = 1 + max best of its strictly
// larger neighbours, which are already final. Iterative, so no deep recursion.
// Time O(rc log rc), space O(rc).

use std::cmp::Reverse;

pub fn longest_increasing_path<T: Ord>(matrix: &[Vec<T>]) -> usize {
    let rows = matrix.len();
    let cols = matrix.first().map_or(0, Vec::len);
    let mut cells: Vec<(usize, usize)> = (0..rows)
        .flat_map(|r| (0..cols).map(move |c| (r, c)))
        .collect();
    cells.sort_unstable_by_key(|&(r, c)| Reverse(&matrix[r][c]));

    // best[r * cols + c] = longest increasing path starting at (r, c)
    let mut best = vec![0usize; rows * cols];
    for (r, c) in cells {
        let here = &matrix[r][c];
        let longest_next = neighbours(r, c, rows, cols)
            .filter(|&(nr, nc)| matrix[nr][nc] > *here)
            .map(|(nr, nc)| best[nr * cols + nc])
            .max()
            .unwrap_or(0);
        best[r * cols + c] = longest_next + 1;
    }
    best.into_iter().max().unwrap_or(0)
}

/// The in-bounds cells directly above, below, left and right of (r, c).
fn neighbours(r: usize, c: usize, rows: usize, cols: usize) -> impl Iterator<Item = (usize, usize)> {
    let up = r.checked_sub(1).map(|r| (r, c));
    let left = c.checked_sub(1).map(|c| (r, c));
    [up, Some((r + 1, c)), left, Some((r, c + 1))]
        .into_iter()
        .flatten()
        .filter(move |&(r, c)| r < rows && c < cols)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
