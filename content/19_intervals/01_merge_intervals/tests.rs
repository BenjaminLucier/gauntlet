// Tests for Merge Intervals — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Paint coverage on doubled coordinates (so touching intervals connect but
/// (1, 2) and (3, 4) stay apart), then read off the runs.
fn brute_merge(intervals: &[(i32, i32)]) -> Vec<(i32, i32)> {
    const OFFSET: i32 = 10_000;
    let mut cover = vec![false; 4 * OFFSET as usize + 3];
    for &(start, end) in intervals {
        let (a, b) = (2 * (start + OFFSET), 2 * (end + OFFSET));
        cover[a as usize..=b as usize].fill(true);
    }
    let mut merged = Vec::new();
    let mut p = 0;
    while p < cover.len() {
        if cover[p] {
            let s = p;
            while cover[p + 1] {
                p += 1;
            }
            merged.push((s as i32 / 2 - OFFSET, p as i32 / 2 - OFFSET));
        }
        p += 1;
    }
    merged
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

    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        lo + (self.next() % (hi - lo + 1) as u64) as i32
    }
}

#[test]
fn example_1() {
    assert_eq!(
        merge_intervals(vec![(1, 3), (2, 6), (8, 10), (15, 18)]),
        [(1, 6), (8, 10), (15, 18)]
    );
}

#[test]
fn example_2() {
    assert_eq!(merge_intervals(vec![(1, 4), (4, 5)]), [(1, 5)]);
}

#[test]
fn example_3() {
    assert_eq!(merge_intervals(vec![(4, 7), (1, 4)]), [(1, 7)]);
}

#[test]
fn empty_input() {
    assert_eq!(merge_intervals(vec![]), []);
}

#[test]
fn single_interval() {
    assert_eq!(merge_intervals(vec![(0, 0)]), [(0, 0)]);
}

#[test]
fn no_overlap_comes_back_sorted() {
    assert_eq!(merge_intervals(vec![(5, 6), (1, 2), (3, 4)]), [(1, 2), (3, 4), (5, 6)]);
}

#[test]
fn nested_interval_keeps_outer_end() {
    // Trap: setting end = next_end instead of max(end, next_end).
    assert_eq!(merge_intervals(vec![(1, 10), (2, 3), (4, 5)]), [(1, 10)]);
}

#[test]
fn chain_merge() {
    assert_eq!(merge_intervals(vec![(1, 2), (2, 3), (3, 4), (4, 5)]), [(1, 5)]);
}

#[test]
fn duplicates() {
    assert_eq!(merge_intervals(vec![(2, 3), (2, 3), (2, 3)]), [(2, 3)]);
}

#[test]
fn same_start_different_ends() {
    assert_eq!(merge_intervals(vec![(1, 4), (1, 2), (1, 9), (1, 3)]), [(1, 9)]);
}

#[test]
fn unsorted_input_needs_sort() {
    // Trap: assuming the input is sorted.
    assert_eq!(merge_intervals(vec![(8, 9), (1, 3), (2, 8)]), [(1, 9)]);
}

#[test]
fn point_intervals() {
    assert_eq!(merge_intervals(vec![(0, 0), (0, 0), (1, 1)]), [(0, 0), (1, 1)]);
    assert_eq!(merge_intervals(vec![(1, 1), (0, 2)]), [(0, 2)]);
}

#[test]
fn reverse_sorted() {
    assert_eq!(
        merge_intervals(vec![(9, 10), (6, 8), (3, 6), (0, 1)]),
        [(0, 1), (3, 8), (9, 10)]
    );
}

#[test]
fn negative_coordinates() {
    assert_eq!(
        merge_intervals(vec![(-5, -3), (-10_000, -9_000), (-3, 2), (4, 10_000)]),
        [(-10_000, -9_000), (-5, 2), (4, 10_000)]
    );
}

#[test]
fn caller_can_clone_to_keep_original() {
    let original = vec![(3, 4), (1, 3)];
    let merged = merge_intervals(original.clone());
    assert_eq!(merged, [(1, 4)]);
    assert_eq!(original, [(3, 4), (1, 3)]);
}

#[test]
fn random_against_coverage() {
    let mut rng = Rng(0xFEED_FACE_CAFE_BEEF);
    for _ in 0..500 {
        let n = rng.range(0, 8) as usize;
        let intervals: Vec<(i32, i32)> = (0..n)
            .map(|_| {
                let s = rng.range(-8, 15);
                (s, s + rng.range(0, 4))
            })
            .collect();
        assert_eq!(
            merge_intervals(intervals.clone()),
            brute_merge(&intervals),
            "intervals: {intervals:?}"
        );
    }
}

#[test]
fn large_input() {
    // 1800 shuffled short intervals, some of them overlapping.
    let mut rng = Rng(42);
    let intervals: Vec<(i32, i32)> = (0..1800)
        .map(|_| {
            let s = rng.range(-10_000, 9_990);
            (s, s + rng.range(0, 3))
        })
        .collect();
    assert_eq!(merge_intervals(intervals.clone()), brute_merge(&intervals));
}

#[test]
fn large_all_overlapping() {
    let intervals: Vec<(i32, i32)> = (0..10_000).rev().map(|i| (i, i + 1)).collect();
    assert_eq!(merge_intervals(intervals), [(0, 10_000)]);
}

#[test]
fn large_nothing_overlaps() {
    // 10^4 distinct points in shuffled order: nothing merges, and the result
    // must still come back sorted.
    let intervals: Vec<(i32, i32)> = (0..10_000).map(|i| ((i * 7_919) % 10_000, (i * 7_919) % 10_000)).collect();
    let merged = merge_intervals(intervals);
    assert_eq!(merged.len(), 10_000);
    assert!(merged.windows(2).all(|w| w[0].1 < w[1].0));
}
