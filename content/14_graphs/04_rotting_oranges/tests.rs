// Tests for Rotting Oranges — shared by template.rs, solution.rs and your attempt.
use super::*;
use Cell::{Empty as E, Fresh as F, Rotten as R};

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

/// Literal minute-by-minute simulation.
fn brute_minutes(grid: &[Vec<Cell>]) -> Option<usize> {
    let mut g = grid.to_vec();
    let m = g.len();
    let n = g.first().map_or(0, Vec::len);
    let mut minutes = 0;
    loop {
        if !g.iter().flatten().any(|&x| x == F) {
            return Some(minutes);
        }
        let mut next = g.clone();
        let mut changed = false;
        for r in 0..m {
            for c in 0..n {
                if g[r][c] != F {
                    continue;
                }
                let rotten_neighbour = (r > 0 && g[r - 1][c] == R)
                    || (r + 1 < m && g[r + 1][c] == R)
                    || (c > 0 && g[r][c - 1] == R)
                    || (c + 1 < n && g[r][c + 1] == R);
                if rotten_neighbour {
                    next[r][c] = R;
                    changed = true;
                }
            }
        }
        if !changed {
            return None;
        }
        g = next;
        minutes += 1;
    }
}

/// A snake of fresh oranges starting at a rotten corner.
fn serpentine(size: usize) -> Vec<Vec<Cell>> {
    (0..size)
        .map(|r| {
            (0..size)
                .map(|c| {
                    if r == 0 && c == 0 {
                        R
                    } else if r % 2 == 0
                        || (r % 4 == 1 && c == size - 1)
                        || (r % 4 == 3 && c == 0)
                    {
                        F
                    } else {
                        E
                    }
                })
                .collect()
        })
        .collect()
}

#[test]
fn example_1() {
    assert_eq!(
        oranges_rotting(&[vec![R, F, F], vec![F, F, E], vec![E, F, F]]),
        Some(4)
    );
}

#[test]
fn example_2_unreachable_is_none() {
    assert_eq!(
        oranges_rotting(&[vec![R, F, F], vec![E, F, F], vec![F, E, F]]),
        None
    );
}

#[test]
fn example_3_nothing_fresh() {
    assert_eq!(oranges_rotting(&[vec![E, R]]), Some(0));
}

#[test]
fn empty_grid() {
    assert_eq!(oranges_rotting(&[]), Some(0));
    assert_eq!(oranges_rotting(&[vec![], vec![]]), Some(0));
}

#[test]
fn single_empty_cell() {
    assert_eq!(oranges_rotting(&[vec![E]]), Some(0));
}

#[test]
fn single_fresh_orange_never_rots() {
    assert_eq!(oranges_rotting(&[vec![F]]), None);
}

#[test]
fn single_rotten_orange() {
    assert_eq!(oranges_rotting(&[vec![R]]), Some(0));
}

#[test]
fn fresh_but_no_rotten() {
    assert_eq!(oranges_rotting(&[vec![F, F], vec![E, F]]), None);
}

#[test]
fn last_level_is_not_counted() {
    // The BFS's last level rots nothing; counting it gives 1 instead of 0.
    assert_eq!(oranges_rotting(&[vec![R, R], vec![R, E]]), Some(0));
    assert_eq!(oranges_rotting(&[vec![R, F]]), Some(1));
}

#[test]
fn simultaneous_spread_from_two_sources() {
    // Rotting from both ends at once: 2 minutes, not 4 from one end.
    assert_eq!(oranges_rotting(&[vec![R, F, F, F, F, R]]), Some(2));
    assert_eq!(oranges_rotting(&[vec![R, F, F, F, F, F, R]]), Some(3));
}

#[test]
fn diagonal_does_not_spread() {
    assert_eq!(oranges_rotting(&[vec![R, E], vec![E, F]]), None);
}

#[test]
fn isolated_fresh_far_away() {
    assert_eq!(oranges_rotting(&[vec![R, F, E, F]]), None);
}

#[test]
fn input_is_not_modified() {
    let grid = vec![vec![R, F, F]];
    assert_eq!(oranges_rotting(&grid), Some(2));
    assert_eq!(grid, vec![vec![R, F, F]]);
    assert_eq!(oranges_rotting(&grid), Some(2));
}

#[test]
fn randomized_vs_simulation() {
    let mut rng = Rng(0x1234_5678_9ABC_DEF1);
    for _ in 0..500 {
        let m = 1 + rng.below(6) as usize;
        let n = 1 + rng.below(6) as usize;
        let grid: Vec<Vec<Cell>> = (0..m)
            .map(|_| {
                (0..n)
                    .map(|_| match rng.below(10) {
                        0..=1 => E,
                        2 => R,
                        _ => F,
                    })
                    .collect()
            })
            .collect();
        assert_eq!(
            oranges_rotting(&grid),
            brute_minutes(&grid),
            "grid = {grid:?}"
        );
    }
}

#[test]
fn serpentine_path() {
    // 10x10 snake: the rot travels the full path length.
    let grid = serpentine(10);
    assert_eq!(brute_minutes(&grid), Some(5 * 10 + 5 - 1));
    assert_eq!(oranges_rotting(&grid), Some(5 * 10 + 5 - 1));
}

#[test]
fn large_grid_single_source() {
    let mut grid = vec![vec![F; 100]; 100];
    grid[0][0] = R;
    assert_eq!(oranges_rotting(&grid), Some(198));
}

#[test]
fn large_serpentines_in_four_orientations() {
    // A path of 50*100 + 50 - 1 oranges from the rotten corner, in its four
    // mirror images: rot advances one cell per minute, so rescanning the whole
    // grid every minute costs ~5 * 10^7 steps per grid, against 10^4 for one
    // multi-source BFS.
    let snake = serpentine(100);
    let flipped_rows: Vec<Vec<Cell>> = snake.iter().rev().cloned().collect();
    let mirror = |g: &[Vec<Cell>]| -> Vec<Vec<Cell>> {
        g.iter().map(|row| row.iter().rev().copied().collect()).collect()
    };
    let grids = [mirror(&snake), mirror(&flipped_rows), snake, flipped_rows];
    for (i, grid) in grids.iter().enumerate() {
        let answer = oranges_rotting(grid);
        assert_eq!(answer, Some(50 * 100 + 50 - 1), "100 x 100 serpentine, mirror image {i}");
    }
}

#[test]
fn large_one_unreachable_orange() {
    let mut grid = vec![vec![F; 100]; 100];
    grid[50][50] = R;
    grid[0][1] = E;
    grid[1][0] = E;
    assert_eq!(oranges_rotting(&grid), None);
}
