// Rotate Image — reference solution.
// Transpose in place (split_at_mut gives two disjoint rows to swap between),
// then reverse each row: together a 90° clockwise turn. Time O(n^2), space O(1).

use std::mem;

pub fn rotate<T>(matrix: &mut [Vec<T>]) {
    for r in 0..matrix.len() {
        let (upper, lower) = matrix.split_at_mut(r + 1);
        let row = &mut upper[r];
        for (c, other) in (r + 1..).zip(lower.iter_mut()) {
            mem::swap(&mut row[c], &mut other[r]);
        }
    }
    for row in matrix.iter_mut() {
        row.reverse();
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
