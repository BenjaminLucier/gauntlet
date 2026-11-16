// Tests for Rotate Image — shared by template.rs, solution.rs and your attempt.
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

/// Obviously-correct oracle: build a fresh rotated copy.
fn brute_rotate<T: Clone>(m: &[Vec<T>]) -> Vec<Vec<T>> {
    let n = m.len();
    (0..n)
        .map(|r| (0..n).map(|c| m[n - 1 - c][r].clone()).collect())
        .collect()
}

fn check(input: &[Vec<i32>], expected: &[Vec<i32>]) {
    let mut m = input.to_vec();
    rotate(&mut m);
    assert_eq!(m, expected, "input = {input:?}");
}

fn seq_matrix(n: usize) -> Vec<Vec<i32>> {
    (0..n)
        .map(|r| (0..n).map(|c| (r * n + c) as i32).collect())
        .collect()
}

#[test]
fn example_1() {
    check(
        &[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        &[vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]],
    );
}

#[test]
fn example_2() {
    check(
        &[
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ],
        &[
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ],
    );
}

#[test]
fn example_3_str_elements() {
    let mut m = vec![vec!["a", "b"], vec!["c", "d"]];
    rotate(&mut m);
    assert_eq!(m, [["c", "a"], ["d", "b"]]);
}

#[test]
fn empty_matrix_is_a_no_op() {
    let mut m: Vec<Vec<i32>> = Vec::new();
    rotate(&mut m);
    assert!(m.is_empty());
}

#[test]
fn single_element() {
    check(&[vec![42]], &[vec![42]]);
}

#[test]
fn two_by_two() {
    check(&[vec![1, 2], vec![3, 4]], &[vec![3, 1], vec![4, 2]]);
}

#[test]
fn not_counterclockwise() {
    // Counter-clockwise would give [[2, 4], [1, 3]].
    let mut m = vec![vec![1, 2], vec![3, 4]];
    rotate(&mut m);
    assert_ne!(m, [[2, 4], [1, 3]], "rotated the wrong way");
    assert_eq!(m, [[3, 1], [4, 2]]);
}

#[test]
fn transpose_only_is_wrong() {
    let input = seq_matrix(3);
    check(&input, &brute_rotate(&input));
}

#[test]
fn odd_size_center_stays() {
    let input = seq_matrix(5);
    let mut m = input.clone();
    rotate(&mut m);
    assert_eq!(m[2][2], input[2][2]);
    assert_eq!(m, brute_rotate(&input));
}

#[test]
fn extreme_values_and_duplicates() {
    check(
        &[vec![-1000, 1000, 0], vec![0, 0, 0], vec![1000, -1000, -1000]],
        &[vec![1000, 0, -1000], vec![-1000, 0, 1000], vec![-1000, 0, 0]],
    );
    check(
        &[vec![i32::MIN, i32::MAX], vec![0, -1]],
        &[vec![0, i32::MIN], vec![-1, i32::MAX]],
    );
}

#[test]
fn four_rotations_is_identity() {
    for n in 1..=7 {
        let input = seq_matrix(n);
        let mut m = input.clone();
        for _ in 0..4 {
            rotate(&mut m);
        }
        assert_eq!(m, input, "n = {n}");
    }
}

#[test]
fn two_rotations_is_180() {
    let input = seq_matrix(6);
    let mut m = input.clone();
    rotate(&mut m);
    rotate(&mut m);
    let expected: Vec<Vec<i32>> = input
        .iter()
        .rev()
        .map(|row| row.iter().rev().copied().collect())
        .collect();
    assert_eq!(m, expected);
}

#[test]
fn all_sizes_sequential() {
    for n in 1..=20 {
        let input = seq_matrix(n);
        check(&input, &brute_rotate(&input));
    }
}

#[test]
fn generic_over_strings() {
    // Owned, non-Copy elements: the rotation has to move them, not copy them.
    let input: Vec<Vec<String>> = (0..4)
        .map(|r| (0..4).map(|c| format!("{r}{c}")).collect())
        .collect();
    let mut m = input.clone();
    rotate(&mut m);
    assert_eq!(m, brute_rotate(&input));
}

#[test]
fn generic_over_bytes_and_tuples() {
    let mut pixels = vec![b"ab".to_vec(), b"cd".to_vec()];
    rotate(&mut pixels);
    assert_eq!(pixels, [b"ca".to_vec(), b"db".to_vec()]);

    let mut points = vec![vec![(0, 0), (0, 1)], vec![(1, 0), (1, 1)]];
    rotate(&mut points);
    assert_eq!(points, [[(1, 0), (0, 0)], [(1, 1), (0, 1)]]);
}

#[test]
fn works_without_clone() {
    // Deliberately neither Clone nor Copy.
    #[derive(Debug, PartialEq)]
    struct Pixel(u32);
    let mut m: Vec<Vec<Pixel>> = (0..3)
        .map(|r| (0..3).map(|c| Pixel(r * 3 + c)).collect())
        .collect();
    rotate(&mut m);
    let got: Vec<Vec<u32>> = m.iter().map(|row| row.iter().map(|p| p.0).collect()).collect();
    assert_eq!(got, [[6, 3, 0], [7, 4, 1], [8, 5, 2]]);
}

#[test]
fn randomized_vs_brute_force() {
    let mut rng = Rng(0x1234_5678_9abc_def1);
    for _ in 0..400 {
        let n = rng.range(1, 20) as usize;
        let input: Vec<Vec<i32>> = (0..n)
            .map(|_| (0..n).map(|_| rng.range(-1000, 1000) as i32).collect())
            .collect();
        check(&input, &brute_rotate(&input));
    }
}

#[test]
fn large_many_rotations() {
    let input = seq_matrix(20);
    let mut m = input.clone();
    let mut expected = input.clone();
    for i in 0..400 {
        rotate(&mut m);
        expected = brute_rotate(&expected);
        if i % 50 == 0 {
            assert_eq!(m, expected, "after {} rotations", i + 1);
        }
    }
    assert_eq!(m, input);
}
