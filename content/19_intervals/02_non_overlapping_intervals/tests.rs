// Tests for Non-overlapping Intervals — shared by template.rs, solution.rs and your attempt.
use super::*;

fn overlap(a: (i32, i32), b: (i32, i32)) -> bool {
    a.0 < b.1 && b.0 < a.1
}

/// Enumerate every subset to keep; the answer is n minus the largest
/// pairwise non-overlapping one.
fn brute_min_removals(intervals: &[(i32, i32)]) -> usize {
    let n = intervals.len();
    let mut best = 0;
    for mask in 0u32..1 << n {
        let kept: Vec<(i32, i32)> = (0..n)
            .filter(|&i| (mask >> i) & 1 == 1)
            .map(|i| intervals[i])
            .collect();
        if kept.len() <= best {
            continue;
        }
        let ok = kept
            .iter()
            .enumerate()
            .all(|(p, &a)| kept[p + 1..].iter().all(|&b| !overlap(a, b)));
        if ok {
            best = kept.len();
        }
    }
    n - best
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
    assert_eq!(min_removals(vec![(1, 2), (2, 3), (3, 4), (1, 3)]), 1);
}

#[test]
fn example_2() {
    assert_eq!(min_removals(vec![(1, 2), (1, 2), (1, 2)]), 2);
}

#[test]
fn example_3() {
    assert_eq!(min_removals(vec![(1, 2), (2, 3)]), 0);
}

#[test]
fn empty_needs_no_removals() {
    assert_eq!(min_removals(vec![]), 0);
}

#[test]
fn single_interval() {
    assert_eq!(min_removals(vec![(-3, 7)]), 0);
}

#[test]
fn touching_is_not_overlap() {
    assert_eq!(min_removals(vec![(0, 1), (1, 2), (2, 3), (3, 4)]), 0);
}

#[test]
fn long_interval_covers_many() {
    // Trap: sorting by start and keeping the first — (0, 100) would block all.
    assert_eq!(min_removals(vec![(0, 100), (1, 2), (3, 4), (5, 6)]), 1);
}

#[test]
fn sort_by_start_keep_first_is_wrong() {
    assert_eq!(min_removals(vec![(1, 10), (2, 3), (3, 4), (4, 5)]), 1);
}

#[test]
fn shortest_first_is_wrong() {
    // Greedy by length keeps (4, 6) and loses both (1, 5) and (5, 9);
    // optimal keeps (1, 5) and (5, 9) and removes only (4, 6).
    assert_eq!(min_removals(vec![(1, 5), (5, 9), (4, 6)]), 1);
}

#[test]
fn fewest_conflicts_first_is_tricky() {
    assert_eq!(
        min_removals(vec![
            (0, 2),
            (1, 3),
            (1, 3),
            (1, 3),
            (2, 4),
            (3, 5),
            (4, 6),
            (5, 7),
            (5, 7),
            (5, 7),
            (6, 8),
        ]),
        7
    );
}

#[test]
fn negatives_and_bounds() {
    assert_eq!(min_removals(vec![(-50_000, 50_000), (-50_000, 0), (0, 50_000)]), 1);
}

#[test]
fn sentinel_does_not_overflow() {
    // Trap: a "last end" sentinel of i32::MIN must only be compared, never
    // subtracted from.
    assert_eq!(min_removals(vec![(i32::MIN, 0), (i32::MIN + 1, 1)]), 1);
}

#[test]
fn unsorted_input() {
    assert_eq!(min_removals(vec![(7, 9), (1, 4), (3, 8), (2, 3)]), 2);
}

#[test]
fn random_against_subset_enumeration() {
    let mut rng = Rng(0xC0FF_EE00_1122_3344);
    for _ in 0..400 {
        let n = rng.range(0, 10) as usize;
        let intervals: Vec<(i32, i32)> = (0..n)
            .map(|_| {
                let s = rng.range(-5, 10);
                (s, s + rng.range(1, 5))
            })
            .collect();
        assert_eq!(
            min_removals(intervals.clone()),
            brute_min_removals(&intervals),
            "intervals: {intervals:?}"
        );
    }
}

#[test]
fn large_chain() {
    // 2000 touching intervals, shuffled: nothing to remove.
    let intervals: Vec<(i32, i32)> = (0..2000)
        .map(|i| (i * 7_919) % 2000)
        .map(|i| (i - 1000, i - 1000 + 1))
        .collect();
    assert_eq!(min_removals(intervals), 0);
}

#[test]
fn large_all_overlapping() {
    // n intervals that all share a common point: only one can be kept. A DP
    // over earlier intervals finds no compatible one and scans them all,
    // ~n^2 / 2 = 2 * 10^8 checks per call; two calls. Greedy by end is
    // n log n for the sort plus one pass.
    let n = 20_000;
    for (a, b) in [(10, 7), (13, 11)] {
        let intervals: Vec<(i32, i32)> = (0..n).map(|i| (-50_000 + i % a, 50_000 - i % b)).collect();
        assert_eq!(min_removals(intervals), n as usize - 1, "{n} intervals (-50000 + i % {a}, 50000 - i % {b})");
    }
}
