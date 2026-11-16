// Tests for Container With Most Water — shared by template.rs, solution.rs and your attempt.
use super::*;

fn check(heights: &[u32], expected: u64) {
    let got = max_area(heights);
    if heights.len() <= 50 {
        assert_eq!(got, expected, "heights = {heights:?}");
    } else {
        assert_eq!(got, expected, "heights.len() = {}", heights.len());
    }
}

/// O(n²) oracle: try every pair.
fn brute_max_area(h: &[u32]) -> u64 {
    let mut best = 0;
    for i in 0..h.len() {
        for j in i + 1..h.len() {
            best = best.max((j - i) as u64 * u64::from(h[i].min(h[j])));
        }
    }
    best
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

    fn range(&mut self, lo: i64, hi: i64) -> i64 {
        lo + (self.next() % (hi - lo + 1) as u64) as i64
    }
}

#[test]
fn example_1() {
    check(&[1, 8, 6, 2, 5, 4, 8, 3, 7], 49);
}

#[test]
fn example_2() {
    check(&[1, 1], 1);
}

#[test]
fn example_single_line() {
    check(&[4], 0);
}

#[test]
fn example_extremes() {
    check(&[u32::MAX, 0, u32::MAX], 2 * u64::from(u32::MAX));
}

#[test]
fn empty() {
    // Trap: `heights.len() - 1` underflows.
    check(&[], 0);
}

#[test]
fn two_lines_one_zero() {
    check(&[0, 10_000], 0);
}

#[test]
fn all_zero() {
    check(&[0, 0, 0, 0], 0);
}

#[test]
fn tallest_pair_is_not_best() {
    // Trap: picking the two tallest lines (area 1*50) instead of widest.
    check(&[10, 1, 1, 1, 1, 50, 50, 1, 1, 10], 90);
}

#[test]
fn widest_is_not_best() {
    check(&[1, 50, 1, 1, 50, 1], 150);
}

#[test]
fn increasing() {
    check(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 25);
}

#[test]
fn decreasing() {
    check(&[10, 9, 8, 7, 6, 5, 4, 3, 2, 1], 25);
}

#[test]
fn move_taller_pointer_is_wrong() {
    // Trap: moving the taller side inward skips the 2..9 pair (area 7*9 = 63).
    check(&[2, 9, 1, 1, 1, 1, 1, 1, 9, 3], 63);
}

#[test]
fn equal_heights_at_ends() {
    // When heights tie, either pointer may move.
    check(&[5, 1, 9, 9, 1, 5], 25);
}

#[test]
fn overflow_in_u32_product() {
    // Trap: computing width * height in u32 before widening.
    check(&[u32::MAX, u32::MAX], u64::from(u32::MAX));
    check(&[3_000_000_000, 1, 1, 3_000_000_000], 9_000_000_000);
}

#[test]
fn randomized_against_brute() {
    let mut rng = Rng(0x6A09_E667_F3BC_C908);
    for _ in 0..400 {
        let n = rng.range(0, 20) as usize;
        let max = match rng.range(0, 2) {
            0 => 10,
            1 => 10_000,
            _ => i64::from(u32::MAX),
        };
        let h: Vec<u32> = (0..n).map(|_| rng.range(0, max) as u32).collect();
        check(&h, brute_max_area(&h));
    }
}

#[test]
fn max_answer() {
    // Area (n - 1) * u32::MAX ~ 4.3 * 10^12: needs u64.
    let n = 1_000;
    let mut h = vec![1; n];
    h[0] = u32::MAX;
    h[n - 1] = u32::MAX;
    check(&h, (n as u64 - 1) * u64::from(u32::MAX));
}

#[test]
fn large_peak_in_middle() {
    // Heights rise then fall and are symmetric, so the best container is some
    // (i, n - 1 - i). Trying every pair is ~n^2 / 2 = 6 * 10^8 pairs here;
    // two pointers take n steps.
    let n = 35_000usize;
    let h: Vec<u32> = (0..n).map(|i| (i.min(n - 1 - i) / 5).min(10_000) as u32).collect();
    let best = (0..n / 2).map(|i| (n - 1 - 2 * i) as u64 * u64::from(h[i])).max().expect("non-empty");
    check(&h, best);
}
