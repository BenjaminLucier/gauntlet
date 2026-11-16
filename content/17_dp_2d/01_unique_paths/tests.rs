// Tests for Unique Paths — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Exponential recursion: only usable for tiny grids.
fn brute_paths(rows: usize, cols: usize) -> u64 {
    if rows == 1 || cols == 1 {
        return 1;
    }
    brute_paths(rows - 1, cols) + brute_paths(rows, cols - 1)
}

/// Exact binomial C(rows + cols - 2, min - 1) in u128, or None once it exceeds u64.
/// The partial products C(total, i) only grow, so stopping early is sound.
fn binom_paths(rows: usize, cols: usize) -> Option<u64> {
    let total = (rows + cols - 2) as u128;
    let k = (rows.min(cols) - 1) as u128;
    let mut r: u128 = 1;
    for i in 0..k {
        r = r * (total - i) / (i + 1);
        if r > u128::from(u64::MAX) {
            return None;
        }
    }
    u64::try_from(r).ok()
}

#[test]
fn example_1() {
    assert_eq!(unique_paths(3, 7), Some(28));
}

#[test]
fn example_2() {
    assert_eq!(unique_paths(3, 2), Some(3));
}

#[test]
fn example_single_cell() {
    assert_eq!(unique_paths(1, 1), Some(1));
}

#[test]
fn empty_grid_has_no_paths() {
    assert_eq!(unique_paths(0, 5), Some(0));
    assert_eq!(unique_paths(5, 0), Some(0));
    assert_eq!(unique_paths(0, 0), Some(0));
}

#[test]
fn example_overflow_is_none() {
    assert_eq!(unique_paths(100, 100), None);
}

#[test]
fn single_row_or_column() {
    assert_eq!(unique_paths(1, 100), Some(1));
    assert_eq!(unique_paths(100, 1), Some(1));
    assert_eq!(unique_paths(1, 2), Some(1));
    assert_eq!(unique_paths(2, 1), Some(1));
}

#[test]
fn two_by_n() {
    for n in 1..=100 {
        assert_eq!(unique_paths(2, n), Some(n as u64), "rows = 2, cols = {n}");
        assert_eq!(unique_paths(n, 2), Some(n as u64), "rows = {n}, cols = 2");
    }
}

#[test]
fn symmetric() {
    for r in 0..=40 {
        for c in 0..=40 {
            assert_eq!(unique_paths(r, c), unique_paths(c, r), "rows = {r}, cols = {c}");
        }
    }
}

#[test]
fn square_grids() {
    assert_eq!(unique_paths(2, 2), Some(2));
    assert_eq!(unique_paths(3, 3), Some(6));
    assert_eq!(unique_paths(10, 10), Some(48_620));
    assert_eq!(unique_paths(17, 17), Some(601_080_390));
}

#[test]
fn naive_binomial_overflow_trap() {
    // C(105, 6): a factorial or a careless 32-bit product overflows long before the end.
    assert_eq!(unique_paths(100, 7), Some(1_609_344_100));
    assert_eq!(unique_paths(7, 100), Some(1_609_344_100));
}

#[test]
fn answer_exceeds_u32() {
    // C(40, 20) = 137 846 528 820 — a u32 or i32 accumulator would overflow.
    assert_eq!(unique_paths(21, 21), Some(137_846_528_820));
}

#[test]
fn u64_boundary() {
    // C(67, 33) = 14 226 520 737 620 288 370 fits; C(68, 34) does not.
    assert_eq!(unique_paths(35, 34), Some(14_226_520_737_620_288_370));
    assert_eq!(unique_paths(34, 35), Some(14_226_520_737_620_288_370));
    assert_eq!(unique_paths(35, 35), None);
}

#[test]
fn overflow_must_not_panic_or_wrap() {
    // A plain `+` panics under overflow checks; `wrapping_add` returns garbage.
    assert_eq!(unique_paths(40, 40), None);
    assert_eq!(unique_paths(1000, 1000), None);
}

#[test]
fn brute_cross_check_small() {
    for r in 1..=11 {
        for c in 1..=11 {
            assert_eq!(unique_paths(r, c), Some(brute_paths(r, c)), "rows = {r}, cols = {c}");
        }
    }
}

#[test]
fn binomial_cross_check_all() {
    // Every (rows, cols) in 1..=100, both the fitting and the overflowing ones.
    for r in 1..=100 {
        for c in 1..=100 {
            assert_eq!(unique_paths(r, c), binom_paths(r, c), "rows = {r}, cols = {c}");
        }
    }
}

#[test]
fn large_grids_are_not_exponential() {
    // Plain recursion makes about as many calls as there are paths: ~1.6 * 10^9
    // for (100, 7) and ~1.4 * 10^19 for (35, 34).
    assert_eq!(unique_paths(100, 7), Some(1_609_344_100));
    assert_eq!(unique_paths(35, 34), Some(14_226_520_737_620_288_370));
    assert_eq!(unique_paths(100, 3), Some(5050));
    assert_eq!(unique_paths(23, 12), Some(193_536_720));
    assert_eq!(unique_paths(500_000, 2), Some(500_000));
    assert_eq!(unique_paths(1, 1_000_000), Some(1));
    assert_eq!(unique_paths(1_000_000, 1), Some(1));
}
