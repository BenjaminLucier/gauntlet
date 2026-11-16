// Tests for Pacific Atlantic Water Flow — shared by template.rs, solution.rs and your attempt.
use super::*;

fn sorted(mut v: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    v.sort_unstable();
    v
}

fn check(heights: &[Vec<u32>], expected: Vec<(usize, usize)>) {
    let got = sorted(pacific_atlantic(heights));
    assert_eq!(got, sorted(expected), "heights = {heights:?}");
}

/// Tiny deterministic PRNG (xorshift64) so random tests are reproducible.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

/// From every cell, follow water downhill and see which edges it reaches.
fn brute_pacific_atlantic(h: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let m = h.len();
    let n = h[0].len();
    let mut out = Vec::new();
    for sr in 0..m {
        for sc in 0..n {
            let mut seen = vec![vec![false; n]; m];
            let mut stack = vec![(sr, sc)];
            seen[sr][sc] = true;
            let (mut pac, mut atl) = (false, false);
            while let Some((r, c)) = stack.pop() {
                pac |= r == 0 || c == 0;
                atl |= r == m - 1 || c == n - 1;
                let mut nbs = Vec::new();
                if r > 0 {
                    nbs.push((r - 1, c));
                }
                if c > 0 {
                    nbs.push((r, c - 1));
                }
                if r + 1 < m {
                    nbs.push((r + 1, c));
                }
                if c + 1 < n {
                    nbs.push((r, c + 1));
                }
                for (a, b) in nbs {
                    if !seen[a][b] && h[a][b] <= h[r][c] {
                        seen[a][b] = true;
                        stack.push((a, b));
                    }
                }
            }
            if pac && atl {
                out.push((sr, sc));
            }
        }
    }
    out
}

fn random_heights(rng: &mut Rng, m: usize, n: usize, range: u64) -> Vec<Vec<u32>> {
    (0..m)
        .map(|_| (0..n).map(|_| rng.below(range) as u32).collect())
        .collect()
}

#[test]
fn example_1() {
    let heights = [
        vec![1, 2, 2, 3, 5],
        vec![3, 2, 3, 4, 4],
        vec![2, 4, 5, 3, 1],
        vec![6, 7, 1, 4, 5],
        vec![5, 1, 1, 2, 4],
    ];
    let expected = vec![(0, 4), (1, 3), (1, 4), (2, 2), (3, 0), (3, 1), (4, 0)];
    check(&heights, expected);
}

#[test]
fn example_2() {
    check(&[vec![1]], vec![(0, 0)]);
}

#[test]
fn empty_map_has_no_cells() {
    assert_eq!(pacific_atlantic(&[]), []);
    assert_eq!(pacific_atlantic(&[vec![], vec![]]), []);
}

#[test]
fn single_row_everything_touches_both() {
    check(&[vec![5, 1, 9, 0]], vec![(0, 0), (0, 1), (0, 2), (0, 3)]);
}

#[test]
fn single_column_everything_touches_both() {
    check(&[vec![3], vec![0], vec![7]], vec![(0, 0), (1, 0), (2, 0)]);
}

#[test]
fn flat_plateau_equal_heights_flow() {
    // Flow is allowed between equal heights (<=, not <).
    let heights = vec![vec![4; 3]; 3];
    let expected: Vec<(usize, usize)> =
        (0..3).flat_map(|r| (0..3).map(move |c| (r, c))).collect();
    check(&heights, expected);
}

#[test]
fn central_pit_cannot_drain() {
    let heights = [vec![5, 5, 5], vec![5, 1, 5], vec![5, 5, 5]];
    let expected = vec![
        (0, 0),
        (0, 1),
        (0, 2),
        (1, 0),
        (1, 2),
        (2, 0),
        (2, 1),
        (2, 2),
    ];
    check(&heights, expected);
}

#[test]
fn slope_toward_pacific() {
    // Heights grow toward the bottom-right: only the Atlantic edges reach both.
    let heights = [vec![0, 1, 2], vec![1, 2, 3], vec![2, 3, 4]];
    let expected = vec![(0, 2), (1, 2), (2, 0), (2, 1), (2, 2)];
    check(&heights, expected);
}

#[test]
fn corners_always_included() {
    // Top-right and bottom-left corners touch both oceans.
    let heights = [vec![9, 0, 0], vec![0, 9, 0], vec![0, 0, 9]];
    let got = pacific_atlantic(&heights);
    assert!(got.contains(&(0, 2)) && got.contains(&(2, 0)), "got {got:?}");
    check(&heights, brute_pacific_atlantic(&heights));
}

#[test]
fn no_duplicates_in_output() {
    // Border cells are seeded from both a row and a column; report each once.
    check(&[vec![1, 1], vec![1, 1]], vec![(0, 0), (0, 1), (1, 0), (1, 1)]);
}

#[test]
fn zero_heights_and_maximum_heights() {
    check(&[vec![0, 0], vec![0, 0]], vec![(0, 0), (0, 1), (1, 0), (1, 1)]);
    let heights = [vec![100_000, 0], vec![0, 100_000]];
    check(&heights, brute_pacific_atlantic(&heights));
}

#[test]
fn randomized_vs_per_cell_search() {
    let mut rng = Rng(0xFEED_FACE_1234_5678);
    for _ in 0..300 {
        let m = 1 + rng.below(7) as usize;
        let n = 1 + rng.below(7) as usize;
        let range = 1 + rng.below(6);
        let heights = random_heights(&mut rng, m, n, range);
        check(&heights, brute_pacific_atlantic(&heights));
    }
}

#[test]
fn medium_random_vs_per_cell_search() {
    let mut rng = Rng(42);
    for _ in 0..3 {
        let heights = random_heights(&mut rng, 25, 25, 100_001);
        check(&heights, brute_pacific_atlantic(&heights));
    }
}

#[test]
fn medium_flat() {
    let heights = vec![vec![100_000; 50]; 50];
    assert_eq!(pacific_atlantic(&heights).len(), 2500);
}

#[test]
fn medium_slope() {
    let heights: Vec<Vec<u32>> = (0..50u32)
        .map(|r| (0..50u32).map(|c| r + c).collect())
        .collect();
    let mut expected: Vec<(usize, usize)> = (0..50).map(|r| (r, 49)).collect();
    expected.extend((0..49).map(|c| (49, c)));
    check(&heights, expected);
}

#[test]
fn walled_basin() {
    // A flat 72 x 72 basin inside a higher rim: no basin cell drains to either
    // ocean, so a search from each cell walks the whole basin, ~2.7 * 10^7
    // cell visits. Searching uphill from the two coastlines visits each cell once.
    let n = 74;
    let heights: Vec<Vec<u32>> = (0..n)
        .map(|r| (0..n).map(|c| if r == 0 || c == 0 || r == n - 1 || c == n - 1 { 100_000 } else { 0 }).collect())
        .collect();
    let rim: Vec<(usize, usize)> = (0..n)
        .flat_map(|r| (0..n).map(move |c| (r, c)))
        .filter(|&(r, c)| r == 0 || c == 0 || r == n - 1 || c == n - 1)
        .collect();
    let got = sorted(pacific_atlantic(&heights));
    assert!(got == rim, "walled basin, n = {n}: got {} cells, expected the {} rim cells", got.len(), rim.len());
}
