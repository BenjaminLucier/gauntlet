// Tests for Longest Increasing Path in a Matrix — shared by template.rs, solution.rs and your attempt.
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

    fn grid(&mut self, m: usize, n: usize, max_val: u64) -> Vec<Vec<i32>> {
        (0..m)
            .map(|_| (0..n).map(|_| self.below(max_val) as i32).collect())
            .collect()
    }
}

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

/// In-bounds neighbours of (r, c) in g.
fn adjacent<T>(g: &[Vec<T>], r: usize, c: usize) -> Vec<(usize, usize)> {
    DIRS.iter()
        .filter_map(|&(dr, dc)| {
            let nr = r.checked_add_signed(dr)?;
            let nc = c.checked_add_signed(dc)?;
            (nr < g.len() && nc < g[0].len()).then_some((nr, nc))
        })
        .collect()
}

/// Unmemoized DFS from every cell (exponential in the worst case).
fn brute_lip<T: Ord>(g: &[Vec<T>]) -> usize {
    fn dfs<T: Ord>(g: &[Vec<T>], r: usize, c: usize) -> usize {
        1 + adjacent(g, r, c)
            .into_iter()
            .filter(|&(nr, nc)| g[nr][nc] > g[r][c])
            .map(|(nr, nc)| dfs(g, nr, nc))
            .max()
            .unwrap_or(0)
    }
    let cols = g.first().map_or(0, Vec::len);
    (0..g.len())
        .flat_map(|r| (0..cols).map(move |c| (r, c)))
        .map(|(r, c)| dfs(g, r, c))
        .max()
        .unwrap_or(0)
}

/// Memoized DFS oracle for big random grids (paths there are short).
fn memo_lip(g: &[Vec<i32>]) -> usize {
    fn dfs(g: &[Vec<i32>], memo: &mut [Vec<usize>], r: usize, c: usize) -> usize {
        if memo[r][c] > 0 {
            return memo[r][c];
        }
        let mut best = 1;
        for (nr, nc) in adjacent(g, r, c) {
            if g[nr][nc] > g[r][c] {
                best = best.max(1 + dfs(g, memo, nr, nc));
            }
        }
        memo[r][c] = best;
        best
    }
    let mut memo = vec![vec![0; g[0].len()]; g.len()];
    let mut ans = 0;
    for r in 0..g.len() {
        for c in 0..g[0].len() {
            ans = ans.max(dfs(g, &mut memo, r, c));
        }
    }
    ans
}

/// Runs `f` on a thread with a big stack so recursive solutions survive 40 000-deep paths.
/// Panics (including `todo!()`) are re-raised unchanged.
fn with_big_stack<F: FnOnce() + Send + 'static>(f: F) {
    let handle = std::thread::Builder::new()
        .stack_size(512 << 20)
        .spawn(f)
        .expect("spawning a test thread");
    if let Err(e) = handle.join() {
        std::panic::resume_unwind(e);
    }
}

/// Boustrophedon numbering of an n x n grid: 0, 1, 2, ... snakes through every cell.
fn snake(n: usize) -> Vec<Vec<usize>> {
    (0..n)
        .map(|r| (0..n).map(|c| r * n + if r % 2 == 0 { c } else { n - 1 - c }).collect())
        .collect()
}

#[test]
fn example_1() {
    assert_eq!(longest_increasing_path(&[vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]), 4);
}

#[test]
fn example_2() {
    assert_eq!(longest_increasing_path(&[vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]]), 4);
}

#[test]
fn example_3() {
    assert_eq!(longest_increasing_path(&[vec![1]]), 1);
}

#[test]
fn example_chars() {
    assert_eq!(longest_increasing_path(&[vec!['a', 'b'], vec!['d', 'c']]), 4);
}

#[test]
fn empty_matrix() {
    assert_eq!(longest_increasing_path::<i32>(&[]), 0);
    assert_eq!(longest_increasing_path::<i32>(&[vec![]]), 0);
    assert_eq!(longest_increasing_path::<i32>(&[vec![], vec![]]), 0);
}

#[test]
fn single_row_and_column() {
    assert_eq!(longest_increasing_path(&[vec![1, 2, 3, 2, 1, 0]]), 4);
    assert_eq!(longest_increasing_path(&[vec![5], vec![4], vec![3], vec![7]]), 3);
}

#[test]
fn all_equal_is_one() {
    // Equal neighbours are not "increasing".
    assert_eq!(longest_increasing_path(&vec![vec![7; 4]; 3]), 1);
}

#[test]
fn diagonal_moves_not_allowed() {
    assert_eq!(longest_increasing_path(&[vec![1, 9], vec![9, 2]]), 2);
    assert_eq!(longest_increasing_path(&[vec![3, 0], vec![0, 4]]), 2);
}

#[test]
fn no_wrapping_around_edges() {
    // Stepping from the end of a row to the start of the next (flat index ± 1)
    // would give 1 → 2 → 3 → 4 and 0 → 1 → 2 → 3.
    assert_eq!(longest_increasing_path(&[vec![1, 2, 3], vec![4, 0, 0]]), 3);
    assert_eq!(longest_increasing_path(&[vec![0, 1], vec![2, 3]]), 3);
}

#[test]
fn path_can_turn_corners() {
    // Spiral: 1..=9 around the edge and into the centre.
    let g = [vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
    assert_eq!(longest_increasing_path(&g), 9);
}

#[test]
fn path_goes_up_and_left() {
    let g = [vec![9, 8, 7], vec![2, 1, 6], vec![3, 4, 5]];
    assert_eq!(longest_increasing_path(&g), 9);
}

#[test]
fn extreme_values() {
    let max = i32::MAX;
    assert_eq!(longest_increasing_path(&[vec![0, max]]), 2);
    assert_eq!(longest_increasing_path(&[vec![max, max], vec![max - 1, max]]), 2);
    assert_eq!(longest_increasing_path(&[vec![0, 0], vec![0, 0]]), 1);
    assert_eq!(longest_increasing_path(&[vec![i64::MIN, -1, 0, i64::MAX]]), 4);
}

#[test]
fn negative_values() {
    let g = [vec![-3, -2, -1], vec![-9, -8, 0]];
    assert_eq!(longest_increasing_path(&g), 5);
}

#[test]
fn generic_over_strings_without_copy() {
    let g: Vec<Vec<String>> = [["apple", "banana"], ["date", "cherry"]]
        .iter()
        .map(|row| row.iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(longest_increasing_path(&g), 4);
}

#[test]
fn generic_over_custom_ord_type() {
    // Ordered by height only; not Copy, not Clone.
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct Height(u16);
    let g = vec![vec![Height(1), Height(5)], vec![Height(2), Height(3)]];
    assert_eq!(longest_increasing_path(&g), 4);
}

#[test]
fn random_vs_brute_force() {
    let mut rng = Rng(0x1357_9BDF_2468_ACE0);
    for _ in 0..400 {
        let m = 1 + rng.below(4) as usize;
        let n = 1 + rng.below(4) as usize;
        let max_val = 2 + rng.below(12);
        let g = rng.grid(m, n, max_val);
        let expected = brute_lip(&g);
        assert_eq!(longest_increasing_path(&g), expected, "matrix = {g:?}");
    }
}

#[test]
fn large_all_equal() {
    assert_eq!(longest_increasing_path(&vec![vec![42; 200]; 200]), 1);
}

#[test]
fn large_diagonal_gradient() {
    // value = r + c: every monotone staircase from (0,0) to (199,199) has 399 cells.
    let g: Vec<Vec<i32>> = (0..200).map(|r| (0..200).map(|c| r + c).collect()).collect();
    assert_eq!(longest_increasing_path(&g), 399);
}

#[test]
fn large_snake_visits_every_cell() {
    // The path 0, 1, 2, ... snakes through all 40 000 cells. Every cell also has a
    // larger vertical neighbour, so unmemoized DFS branches exponentially.
    with_big_stack(|| {
        assert_eq!(longest_increasing_path(&snake(200)), 40_000);
    });
}

#[test]
fn large_snake_reversed_large_values() {
    with_big_stack(|| {
        let g: Vec<Vec<i32>> = snake(200)
            .into_iter()
            .map(|row| row.into_iter().map(|v| i32::MAX - v as i32).collect())
            .collect();
        assert_eq!(longest_increasing_path(&g), 40_000);
    });
}

#[test]
fn large_random_vs_memo() {
    let mut rng = Rng(2024);
    let g = rng.grid(200, 200, 1_000_000_000);
    let expected = memo_lip(&g);
    assert_eq!(longest_increasing_path(&g), expected);
    let g = rng.grid(150, 200, 6);
    let expected = memo_lip(&g);
    assert_eq!(longest_increasing_path(&g), expected);
}
