// Unique Paths — reference solution.
// Rolling 1-D DP: each new row is the running sum of the row above. `checked_add`
// plus `?` reports overflow as None. Time O(rows * cols), space O(cols).

pub fn unique_paths(rows: usize, cols: usize) -> Option<u64> {
    if rows == 0 || cols == 0 {
        return Some(0);
    }
    let mut row = vec![1u64; cols];
    for _ in 1..rows {
        let mut from_left = 0u64;
        for cell in &mut row {
            // paths here = paths from above (old *cell) + paths from the left
            from_left = from_left.checked_add(*cell)?;
            *cell = from_left;
        }
    }
    row.last().copied()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
