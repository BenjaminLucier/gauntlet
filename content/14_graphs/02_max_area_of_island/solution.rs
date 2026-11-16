// Max Area of Island — reference solution.
// Flood-fill each unvisited island with an explicit stack, marking cells when
// they are pushed and counting them; keep the maximum. Time O(m*n), space O(m*n).

pub fn max_area_of_island(grid: &[&str]) -> usize {
    let rows: Vec<&[u8]> = grid.iter().map(|row| row.as_bytes()).collect();
    let m = rows.len();
    let n = rows.first().map_or(0, |row| row.len());
    let mut seen = vec![false; m * n];
    let mut stack = Vec::new();
    let mut best = 0;
    for r in 0..m {
        for c in 0..n {
            if rows[r][c] != b'1' || seen[r * n + c] {
                continue;
            }
            seen[r * n + c] = true;
            stack.push((r, c));
            let mut area = 0;
            while let Some((i, j)) = stack.pop() {
                area += 1;
                // wrapping_sub turns "-1" into usize::MAX, which the bounds check rejects.
                let neighbours = [
                    (i.wrapping_sub(1), j),
                    (i + 1, j),
                    (i, j.wrapping_sub(1)),
                    (i, j + 1),
                ];
                for (a, b) in neighbours {
                    if a < m && b < n && rows[a][b] == b'1' && !seen[a * n + b] {
                        seen[a * n + b] = true;
                        stack.push((a, b));
                    }
                }
            }
            best = best.max(area);
        }
    }
    best
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
