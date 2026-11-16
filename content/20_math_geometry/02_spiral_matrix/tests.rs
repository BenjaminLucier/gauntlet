// Tests for Spiral Matrix — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Tiny deterministic PRNG (xorshift64) so random tests are reproducible.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn range(&mut self, lo: i64, hi: i64) -> i64 {
        lo + (self.next() % (hi - lo + 1) as u64) as i64
    }
}

/// Simulation with a visited grid: turn right whenever the next cell is
/// outside the matrix or already visited.
fn brute_spiral<T: Clone>(m: &[Vec<T>]) -> Vec<T> {
    let rows = m.len() as i64;
    let cols = m.first().map_or(0, Vec::len) as i64;
    let mut seen = vec![vec![false; cols as usize]; rows as usize];
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let (mut r, mut c, mut d) = (0i64, 0i64, 0usize);
    let mut out = Vec::new();
    for _ in 0..rows * cols {
        out.push(m[r as usize][c as usize].clone());
        seen[r as usize][c as usize] = true;
        let (nr, nc) = (r + dirs[d].0, c + dirs[d].1);
        if nr < 0 || nr >= rows || nc < 0 || nc >= cols || seen[nr as usize][nc as usize] {
            d = (d + 1) % 4;
        }
        r += dirs[d].0;
        c += dirs[d].1;
    }
    out
}

fn grid(rows: usize, cols: usize) -> Vec<Vec<i32>> {
    (0..rows)
        .map(|r| (0..cols).map(|c| (r * cols + c + 1) as i32).collect())
        .collect()
}

fn check(matrix: &[Vec<i32>], expected: &[i32]) {
    assert_eq!(spiral_order(matrix), expected, "matrix = {matrix:?}");
}

#[test]
fn example_1() {
    check(&grid(3, 3), &[1, 2, 3, 6, 9, 8, 7, 4, 5]);
}

#[test]
fn example_2() {
    check(&grid(3, 4), &[1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);
}

#[test]
fn example_3_chars() {
    let m = vec![vec!['a', 'b'], vec!['d', 'c']];
    assert_eq!(spiral_order(&m), ['a', 'b', 'c', 'd']);
}

#[test]
fn empty_matrix() {
    assert!(spiral_order::<i32>(&[]).is_empty());
}

#[test]
fn rows_without_columns() {
    // Three rows of width 0: nothing to visit, and no underflow on `right - 1`.
    let m: Vec<Vec<i32>> = vec![vec![]; 3];
    assert!(spiral_order(&m).is_empty());
}

#[test]
fn single_element() {
    check(&[vec![-7]], &[-7]);
}

#[test]
fn single_row() {
    check(&[vec![1, 2, 3, 4, 5]], &[1, 2, 3, 4, 5]);
}

#[test]
fn single_column() {
    check(&[vec![1], vec![2], vec![3], vec![4]], &[1, 2, 3, 4]);
}

#[test]
fn two_by_two() {
    check(&grid(2, 2), &[1, 2, 4, 3]);
}

#[test]
fn middle_single_row_not_duplicated() {
    // 3x5: the last loop leaves a single middle row [7, 8, 9].
    check(&grid(3, 5), &[1, 2, 3, 4, 5, 10, 15, 14, 13, 12, 11, 6, 7, 8, 9]);
}

#[test]
fn middle_single_column_not_duplicated() {
    // 5x3: the last loop leaves a single middle column [5, 8, 11].
    check(&grid(5, 3), &[1, 2, 3, 6, 9, 12, 15, 14, 13, 10, 7, 4, 5, 8, 11]);
}

#[test]
fn wide_two_rows() {
    check(&grid(2, 4), &[1, 2, 3, 4, 8, 7, 6, 5]);
}

#[test]
fn tall_two_columns() {
    check(&grid(4, 2), &[1, 2, 4, 6, 8, 7, 5, 3]);
}

#[test]
fn four_by_four() {
    check(
        &grid(4, 4),
        &[1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10],
    );
}

#[test]
fn negatives_and_duplicates() {
    check(
        &[vec![-100, 100, 0], vec![0, 0, -100]],
        &[-100, 100, 0, -100, 0, 0],
    );
}

#[test]
fn generic_over_strings() {
    // Owned, non-Copy elements are cloned out; the matrix is left intact.
    let m: Vec<Vec<String>> = (0..3)
        .map(|r| (0..4).map(|c| format!("{r}{c}")).collect())
        .collect();
    let before = m.clone();
    assert_eq!(spiral_order(&m), brute_spiral(&m));
    assert_eq!(m, before);
}

#[test]
fn all_shapes_vs_brute_force() {
    for rows in 1..=10 {
        for cols in 1..=10 {
            let m = grid(rows, cols);
            check(&m, &brute_spiral(&m));
        }
    }
}

#[test]
fn randomized_vs_brute_force() {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    for _ in 0..500 {
        let rows = rng.range(1, 10) as usize;
        let cols = rng.range(1, 10) as usize;
        let m: Vec<Vec<i32>> = (0..rows)
            .map(|_| (0..cols).map(|_| rng.range(-100, 100) as i32).collect())
            .collect();
        check(&m, &brute_spiral(&m));
    }
}

#[test]
fn largest_visits_each_exactly_once() {
    let m = grid(10, 10);
    let out = spiral_order(&m);
    assert_eq!(out.len(), 100);
    let mut sorted = out.clone();
    sorted.sort_unstable();
    assert_eq!(sorted, (1..=100).collect::<Vec<i32>>());
    assert_eq!(out, brute_spiral(&m));
}

#[test]
fn large_wide_and_tall() {
    // Bigger than the constraints: long thin shapes and uneven rectangles.
    for (rows, cols) in [(300, 300), (1, 50_000), (50_000, 1), (2, 20_000), (199, 301)] {
        let m = grid(rows, cols);
        let out = spiral_order(&m);
        assert_eq!(out, brute_spiral(&m), "{rows}x{cols}");
    }
}
