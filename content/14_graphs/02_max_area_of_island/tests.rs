// Tests for Max Area of Island — shared by template.rs, solution.rs and your attempt.
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

/// Builds the rows as owned strings; pass them on with `as_rows`.
fn build(m: usize, n: usize, mut land: impl FnMut(usize, usize) -> bool) -> Vec<String> {
    (0..m)
        .map(|r| (0..n).map(|c| if land(r, c) { '1' } else { '0' }).collect())
        .collect()
}

fn as_rows(rows: &[String]) -> Vec<&str> {
    rows.iter().map(String::as_str).collect()
}

fn find(parent: &mut [usize], mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    x
}

/// Union-find with component sizes: an independent oracle.
fn brute_max_area(grid: &[&str]) -> usize {
    let cells: Vec<&[u8]> = grid.iter().map(|r| r.as_bytes()).collect();
    let m = cells.len();
    let n = cells.first().map_or(0, |r| r.len());
    let mut parent: Vec<usize> = (0..m * n).collect();
    for r in 0..m {
        for c in 0..n {
            if cells[r][c] != b'1' {
                continue;
            }
            for (a, b) in [(r + 1, c), (r, c + 1)] {
                if a < m && b < n && cells[a][b] == b'1' {
                    let x = find(&mut parent, r * n + c);
                    let y = find(&mut parent, a * n + b);
                    parent[x] = y;
                }
            }
        }
    }
    let mut size = vec![0; m * n];
    for i in 0..m * n {
        if cells[i / n][i % n] == b'1' {
            size[find(&mut parent, i)] += 1;
        }
    }
    size.into_iter().max().unwrap_or(0)
}

#[test]
fn example_1() {
    let grid = [
        "0010000100000",
        "0000000111000",
        "0110100000000",
        "0100110010100",
        "0100110011100",
        "0000000000100",
        "0000000111000",
        "0000000110000",
    ];
    assert_eq!(max_area_of_island(&grid), 6);
}

#[test]
fn example_2() {
    assert_eq!(max_area_of_island(&["00000000"]), 0);
}

#[test]
fn empty_grid() {
    assert_eq!(max_area_of_island(&[]), 0);
    assert_eq!(max_area_of_island(&[""]), 0);
}

#[test]
fn single_land() {
    assert_eq!(max_area_of_island(&["1"]), 1);
}

#[test]
fn single_water() {
    assert_eq!(max_area_of_island(&["0"]), 0);
}

#[test]
fn diagonal_cells_are_separate() {
    assert_eq!(max_area_of_island(&["101", "010", "101"]), 1);
}

#[test]
fn u_shape_counts_every_cell_once() {
    // Revisiting the bottom row would inflate the count.
    assert_eq!(max_area_of_island(&["101", "101", "111"]), 7);
}

#[test]
fn block_counts_every_cell_once() {
    // Marking visited only on pop (not on push) double-counts here.
    assert_eq!(max_area_of_island(&["111", "111", "111"]), 9);
}

#[test]
fn largest_island_last() {
    assert_eq!(max_area_of_island(&["1000", "0000", "0011", "0011"]), 4);
}

#[test]
fn largest_island_first() {
    assert_eq!(max_area_of_island(&["1100", "1100", "0000", "0001"]), 4);
}

#[test]
fn ring_with_lake() {
    assert_eq!(
        max_area_of_island(&["11111", "10001", "10101", "10001", "11111"]),
        16
    );
}

#[test]
fn single_row_and_column() {
    assert_eq!(max_area_of_island(&["1101110"]), 3);
    assert_eq!(max_area_of_island(&["1", "1", "0", "1"]), 2);
}

#[test]
fn input_is_not_modified() {
    let grid = ["110", "011"];
    assert_eq!(max_area_of_island(&grid), 4);
    assert_eq!(max_area_of_island(&grid), 4);
}

#[test]
fn randomized_vs_union_find() {
    let mut rng = Rng(0xDEAD_BEEF_CAFE_BABE);
    for _ in 0..400 {
        let m = 1 + rng.below(8) as usize;
        let n = 1 + rng.below(8) as usize;
        let density = 1 + rng.below(9);
        let owned = build(m, n, |_, _| rng.below(10) < density);
        let grid = as_rows(&owned);
        assert_eq!(max_area_of_island(&grid), brute_max_area(&grid), "grid = {grid:?}");
    }
}

#[test]
fn large_all_land() {
    // 90,000 connected cells: a recursive flood fill overflows the stack.
    let owned = build(300, 300, |_, _| true);
    assert_eq!(max_area_of_island(&as_rows(&owned)), 90_000);
}

#[test]
fn large_comb() {
    // Columns of land joined by the top row: one island of 50 + 25*49 cells.
    let owned = build(50, 50, |r, c| r == 0 || c % 2 == 0);
    assert_eq!(max_area_of_island(&as_rows(&owned)), 50 + 25 * 49);
}

#[test]
fn large_random_vs_union_find() {
    let mut rng = Rng(12345);
    for _ in 0..5 {
        let owned = build(200, 200, |_, _| rng.below(100) < 60);
        let grid = as_rows(&owned);
        assert_eq!(max_area_of_island(&grid), brute_max_area(&grid));
    }
}
