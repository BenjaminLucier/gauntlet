// Tests for Trapping Rain Water — shared by template.rs, solution.rs and your attempt.
use super::*;

fn check(heights: &[u32], expected: u64) {
    let got = trap(heights);
    if heights.len() <= 50 {
        assert_eq!(got, expected, "heights = {heights:?}");
    } else {
        assert_eq!(got, expected, "heights.len() = {}", heights.len());
    }
}

/// O(n^2) oracle straight from the definition.
fn brute_trap(h: &[u32]) -> u64 {
    (0..h.len())
        .map(|i| {
            let lm = h[..=i].iter().max().expect("non-empty");
            let rm = h[i..].iter().max().expect("non-empty");
            u64::from(lm.min(rm) - h[i])
        })
        .sum()
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
    check(&[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6);
}

#[test]
fn example_2() {
    check(&[4, 2, 0, 3, 2, 5], 9);
}

#[test]
fn example_extremes() {
    check(&[u32::MAX, 0, 0, u32::MAX], 2 * u64::from(u32::MAX));
}

#[test]
fn empty() {
    // Trap: `heights.len() - 1` underflows.
    check(&[], 0);
}

#[test]
fn single_bar() {
    check(&[5], 0);
}

#[test]
fn two_bars() {
    check(&[5, 7], 0);
}

#[test]
fn all_zero() {
    check(&[0, 0, 0, 0], 0);
}

#[test]
fn flat() {
    check(&[3, 3, 3, 3], 0);
}

#[test]
fn increasing_and_decreasing() {
    check(&[1, 2, 3, 4, 5], 0);
    check(&[5, 4, 3, 2, 1], 0);
}

#[test]
fn simple_basin() {
    check(&[3, 0, 3], 3);
    check(&[3, 0, 0, 2], 4);
}

#[test]
fn lower_side_bounds_water() {
    // Trap: using the left max only.
    check(&[5, 0, 0, 0, 1], 3);
    // Trap: using the right max only.
    check(&[1, 0, 0, 0, 5], 3);
}

#[test]
fn nested_basins() {
    check(&[5, 1, 3, 1, 5], 10);
}

#[test]
fn peak_in_middle_no_water() {
    check(&[0, 1, 2, 9, 2, 1, 0], 0);
}

#[test]
fn equal_maxima_both_sides() {
    check(&[4, 1, 4, 1, 4], 6);
}

#[test]
fn unsigned_underflow_trap() {
    // Trap: subtracting a bar from a max that doesn't include it yet.
    check(&[0, 7, 0], 0);
    check(&[2, 9, 1, 9, 2], 8);
}

#[test]
fn overflow_in_u32_total() {
    // Trap: summing in u32. Each bar holds < u32::MAX, but the total doesn't fit.
    check(&[u32::MAX, 1, 1, 1, u32::MAX], 3 * u64::from(u32::MAX - 1));
}

#[test]
fn randomized_against_brute() {
    let mut rng = Rng(0xBB67_AE85_84CA_A73B);
    for _ in 0..400 {
        let n = rng.range(0, 20) as usize;
        let max = match rng.range(0, 2) {
            0 => 6,
            1 => 100_000,
            _ => i64::from(u32::MAX),
        };
        let h: Vec<u32> = (0..n).map(|_| rng.range(0, max) as u32).collect();
        check(&h, brute_trap(&h));
    }
}

#[test]
fn max_water() {
    // 998 * u32::MAX ~ 4.3 * 10^12: needs u64.
    let mut h = vec![0; 1_000];
    h[0] = u32::MAX;
    h[999] = u32::MAX;
    check(&h, 998 * u64::from(u32::MAX));
}

#[test]
fn large_sawtooth_and_walled_basin() {
    // Two inputs of 2*10^4 bars. Scanning left and right for the maxima at
    // every bar is about 4 * 10^8 steps per input, whatever the heights; one
    // two-pointer pass is 2 * 10^4.
    let n = 20_000;
    // Alternating tall walls and shallow dips.
    let h: Vec<u32> = (0..n as u32).map(|i| if i % 2 == 0 { 1000 } else { i % 7 }).collect();
    let expected: u64 = (0..n as u32 - 1).filter(|i| i % 2 == 1).map(|i| u64::from(1000 - i % 7)).sum();
    assert_eq!(trap(&h), expected, "sawtooth, n = {n}");
    // A wall of height `wall` at each end, lower bars in between: every
    // inner bar holds water up to the wall.
    let mut rng = Rng(0x9B05_688C_2B3E_6C1F);
    let wall = 1_000 + rng.range(0, 1 << 20) as u32;
    let mut h: Vec<u32> = (0..n).map(|_| rng.range(0, i64::from(wall) - 1) as u32).collect();
    h[0] = wall;
    h[n - 1] = wall;
    let expected: u64 = h[1..n - 1].iter().map(|&x| u64::from(wall - x)).sum();
    assert_eq!(trap(&h), expected, "walled basin, n = {n}");
}
