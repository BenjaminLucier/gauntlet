// Spiral Matrix — reference solution.
// Shrink four half-open boundaries after walking each side, re-checking them
// before the two return trips. Time O(m*n), space O(1) besides the output.

pub fn spiral_order<T: Clone>(matrix: &[Vec<T>]) -> Vec<T> {
    let rows = matrix.len();
    let cols = matrix.first().map_or(0, Vec::len);
    let mut out = Vec::with_capacity(rows * cols);
    let (mut top, mut bottom, mut left, mut right) = (0, rows, 0, cols);
    while top < bottom && left < right {
        out.extend(matrix[top][left..right].iter().cloned());
        top += 1;
        out.extend(matrix[top..bottom].iter().map(|row| row[right - 1].clone()));
        right -= 1;
        if top < bottom {
            out.extend(matrix[bottom - 1][left..right].iter().rev().cloned());
            bottom -= 1;
        }
        if left < right {
            out.extend(matrix[top..bottom].iter().rev().map(|row| row[left].clone()));
            left += 1;
        }
    }
    out
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
