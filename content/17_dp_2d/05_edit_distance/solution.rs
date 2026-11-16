// Edit Distance — reference solution.
// Levenshtein DP over char prefixes with one rolling row; `diag` holds d[i-1][j-1] before it
// is overwritten. Time O(m * n), space O(n).

pub fn edit_distance(from: &str, to: &str) -> usize {
    let to: Vec<char> = to.chars().collect();
    // row[j] = distance from the prefix of `from` seen so far to to[..j]
    let mut row: Vec<usize> = (0..=to.len()).collect();
    for (i, a) in from.chars().enumerate() {
        let mut diag = row[0];
        row[0] = i + 1;
        for (j, &b) in to.iter().enumerate() {
            let above = row[j + 1];
            row[j + 1] = if a == b {
                diag
            } else {
                1 + diag.min(above).min(row[j])
            };
            diag = above;
        }
    }
    row[to.len()]
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
