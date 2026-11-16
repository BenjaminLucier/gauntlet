// Tests for Swim in Rising Water — shared by template.rs, solution.rs and your attempt.
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

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

fn brute_reachable_at(grid: &[Vec<u32>], t: u32) -> bool {
    let n = grid.len();
    if grid[0][0] > t {
        return false;
    }
    let mut seen = vec![vec![false; n]; n];
    seen[0][0] = true;
    let mut stack = vec![(0usize, 0usize)];
    while let Some((r, c)) = stack.pop() {
        if r == n - 1 && c == n - 1 {
            return true;
        }
        let mut nbs = Vec::new();
        if r > 0 {
            nbs.push((r - 1, c));
        }
        if c > 0 {
            nbs.push((r, c - 1));
        }
        if r + 1 < n {
            nbs.push((r + 1, c));
        }
        if c + 1 < n {
            nbs.push((r, c + 1));
        }
        for (a, b) in nbs {
            if !seen[a][b] && grid[a][b] <= t {
                seen[a][b] = true;
                stack.push((a, b));
            }
        }
    }
    false
}

/// Try every t from 0 upward with a flood fill.
fn brute(grid: &[Vec<u32>]) -> u32 {
    (0..).find(|&t| brute_reachable_at(grid, t)).expect("t = n² - 1 always works")
}

/// Binary search on t with a flood fill per guess.
fn brute_binary(grid: &[Vec<u32>]) -> u32 {
    let n = grid.len() as u32;
    let (mut lo, mut hi) = (0, n * n - 1);
    while lo < hi {
        let mid = (lo + hi) / 2;
        if brute_reachable_at(grid, mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn random_grid(rng: &mut Rng, n: usize) -> Vec<Vec<u32>> {
    let mut vals: Vec<u32> = (0..(n * n) as u32).collect();
    for i in (1..vals.len()).rev() {
        vals.swap(i, rng.below(i as u64 + 1) as usize);
    }
    vals.chunks(n).map(|c| c.to_vec()).collect()
}

#[test]
fn example_1() {
    assert_eq!(swim_in_water(&[vec![0, 2], vec![1, 3]]), 3);
}

#[test]
fn example_2() {
    let grid = vec![
        vec![0, 1, 2, 3, 4],
        vec![24, 23, 22, 21, 5],
        vec![12, 13, 14, 15, 16],
        vec![11, 17, 18, 19, 20],
        vec![10, 9, 8, 7, 6],
    ];
    assert_eq!(swim_in_water(&grid), 16);
}

#[test]
fn single_cell() {
    assert_eq!(swim_in_water(&[vec![0]]), 0);
}

#[test]
fn start_cell_is_highest() {
    // You can't even leave until the water covers the start.
    assert_eq!(swim_in_water(&[vec![3, 0], vec![1, 2]]), 3);
}

#[test]
fn target_cell_is_bottleneck() {
    assert_eq!(swim_in_water(&[vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]), 8);
}

#[test]
fn cheaper_path_via_detour() {
    // Straight-through routes cross 8 or 9; going around the edge peaks at 4.
    let grid = vec![vec![0, 8, 7], vec![1, 9, 6], vec![2, 3, 4]];
    assert_eq!(swim_in_water(&grid), 4);
    let grid = vec![vec![0, 1, 2], vec![7, 8, 3], vec![6, 5, 4]];
    assert_eq!(swim_in_water(&grid), 4);
}

#[test]
fn answer_is_max_not_sum() {
    let grid = vec![vec![0, 1], vec![3, 2]];
    assert_eq!(swim_in_water(&grid), 2);
}

#[test]
fn low_target_behind_high_neighbors() {
    // The target is only 6, but both of its neighbors are higher.
    let grid = vec![vec![0, 1, 2], vec![3, 4, 7], vec![5, 8, 6]];
    assert_eq!(swim_in_water(&grid), 7);
}

#[test]
fn randomized_vs_linear_scan() {
    let mut rng = Rng(0xFACE_B00C_1234_5678);
    for _ in 0..300 {
        let n = 1 + rng.below(7) as usize;
        let grid = random_grid(&mut rng, n);
        let expected = brute(&grid);
        assert_eq!(swim_in_water(&grid), expected, "grid = {grid:?}");
    }
}

#[test]
fn large_random_vs_binary_search() {
    let mut rng = Rng(778);
    for _ in 0..5 {
        let grid = random_grid(&mut rng, 50);
        let expected = brute_binary(&grid);
        assert_eq!(swim_in_water(&grid), expected);
    }
}

#[test]
fn large_serpentine() {
    // Values increase along a snake through the grid; the target is on the snake's
    // last row, so the answer is the largest value needed to get there.
    let n = 50;
    let mut grid = vec![vec![0; n]; n];
    let mut v = 0;
    for (r, row) in grid.iter_mut().enumerate() {
        for i in 0..n {
            let c = if r % 2 == 0 { i } else { n - 1 - i };
            row[c] = v;
            v += 1;
        }
    }
    let expected = brute_binary(&grid);
    assert_eq!(swim_in_water(&grid), expected);
}

#[test]
fn every_2x2_grid() {
    let perms = [
        [0, 1, 2, 3], [0, 1, 3, 2], [0, 2, 1, 3], [0, 2, 3, 1], [0, 3, 1, 2], [0, 3, 2, 1],
        [1, 0, 2, 3], [1, 0, 3, 2], [1, 2, 0, 3], [1, 2, 3, 0], [1, 3, 0, 2], [1, 3, 2, 0],
        [2, 0, 1, 3], [2, 0, 3, 1], [2, 1, 0, 3], [2, 1, 3, 0], [2, 3, 0, 1], [2, 3, 1, 0],
        [3, 0, 1, 2], [3, 0, 2, 1], [3, 1, 0, 2], [3, 1, 2, 0], [3, 2, 0, 1], [3, 2, 1, 0],
    ];
    for p in perms {
        let grid = vec![vec![p[0], p[1]], vec![p[2], p[3]]];
        assert_eq!(swim_in_water(&grid), brute(&grid), "grid = {grid:?}");
    }
}

#[test]
fn input_is_borrowed_and_untouched() {
    let grid = vec![vec![0, 2], vec![1, 3]];
    let copy = grid.clone();
    assert_eq!(swim_in_water(&grid), 3);
    assert_eq!(grid, copy);
}

#[test]
fn works_on_a_slice_of_rows() {
    // Any `&[Vec<u32>]` will do — an array of rows, or a sub-slice of a Vec.
    let rows = [vec![0, 3], vec![2, 1]];
    assert_eq!(swim_in_water(&rows), 2);
    let tall = [vec![0], vec![9]];
    assert_eq!(swim_in_water(&tall[..1]), 0);
}

#[test]
fn large_wall_with_one_gap() {
    // The middle row is a wall of the highest values except for one gap at
    // the far right. Everything below the wall is numbered after everything
    // above it, so the target — the last cell below the wall — sets the level.
    let n = 50;
    let mut next = 0;
    let mut grid = vec![vec![0; n]; n];
    let wall = n / 2;
    for row in grid.iter_mut().take(wall) {
        for cell in row.iter_mut() {
            *cell = next;
            next += 1;
        }
    }
    let gap = next;
    grid[wall][n - 1] = gap;
    next += 1;
    for row in grid.iter_mut().skip(wall + 1) {
        for cell in row.iter_mut() {
            *cell = next;
            next += 1;
        }
    }
    for cell in grid[wall].iter_mut().take(n - 1) {
        *cell = next;
        next += 1;
    }
    let expected = brute_binary(&grid);
    assert_eq!(expected, (n * n - n) as u32, "test setup");
    assert_eq!(swim_in_water(&grid), expected);
}
