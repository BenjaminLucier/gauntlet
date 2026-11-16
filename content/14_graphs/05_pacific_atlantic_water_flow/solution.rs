// Pacific Atlantic Water Flow — reference solution.
// Reverse BFS uphill from each ocean's border cells; the answer is the
// intersection of the two reachable sets. Time O(m*n), space O(m*n).

use std::collections::VecDeque;

/// Marks every cell reachable from `seeds` by walking to neighbours that are
/// at least as high (i.e. every cell that drains into those seeds).
fn climb(
    heights: &[Vec<u32>],
    seeds: impl IntoIterator<Item = (usize, usize)>,
) -> Vec<Vec<bool>> {
    let m = heights.len();
    let n = heights[0].len();
    let mut seen = vec![vec![false; n]; m];
    let mut queue = VecDeque::new();
    for (r, c) in seeds {
        if !seen[r][c] {
            seen[r][c] = true;
            queue.push_back((r, c));
        }
    }
    while let Some((r, c)) = queue.pop_front() {
        // wrapping_sub turns "row -1" into usize::MAX, which the bounds check rejects.
        for (a, b) in [
            (r.wrapping_sub(1), c),
            (r + 1, c),
            (r, c.wrapping_sub(1)),
            (r, c + 1),
        ] {
            if a < m && b < n && !seen[a][b] && heights[a][b] >= heights[r][c] {
                seen[a][b] = true;
                queue.push_back((a, b));
            }
        }
    }
    seen
}

pub fn pacific_atlantic(heights: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let m = heights.len();
    let n = heights.first().map_or(0, Vec::len);
    if m == 0 || n == 0 {
        return Vec::new();
    }
    let pacific = climb(heights, (0..m).map(|r| (r, 0)).chain((0..n).map(|c| (0, c))));
    let atlantic = climb(
        heights,
        (0..m).map(|r| (r, n - 1)).chain((0..n).map(|c| (m - 1, c))),
    );
    (0..m)
        .flat_map(|r| (0..n).map(move |c| (r, c)))
        .filter(|&(r, c)| pacific[r][c] && atlantic[r][c])
        .collect()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
