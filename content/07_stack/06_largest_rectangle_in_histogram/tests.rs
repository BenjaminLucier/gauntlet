// Tests for Largest Rectangle in Histogram — shared by template.rs, solution.rs and your attempt.
use super::*;

fn check(h: &[u32], expected: u64) {
    assert_eq!(largest_rectangle_area(h), expected, "heights = {h:?}");
}

/// Oracle: try every contiguous range, tracking its minimum. O(n^2).
fn brute_area(h: &[u32]) -> u64 {
    let mut best = 0;
    for i in 0..h.len() {
        let mut min = u32::MAX;
        for (j, &x) in h.iter().enumerate().skip(i) {
            min = min.min(x);
            best = best.max(u64::from(min) * (j - i + 1) as u64);
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

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

#[test]
fn example_1() {
    check(&[2, 1, 5, 6, 2, 3], 10);
}

#[test]
fn example_2() {
    check(&[2, 4], 4);
}

#[test]
fn example_3_empty() {
    check(&[], 0);
}

#[test]
fn example_4_area_exceeds_u32() {
    check(&[u32::MAX, u32::MAX], 2 * u64::from(u32::MAX));
}

#[test]
fn single_bar() {
    check(&[7], 7);
    check(&[0], 0);
    check(&[u32::MAX], u64::from(u32::MAX));
}

#[test]
fn all_zero() {
    check(&[0, 0, 0, 0], 0);
}

#[test]
fn all_equal_full_width() {
    check(&[3, 3, 3, 3, 3], 15);
}

#[test]
fn equal_heights_popped_early_trap() {
    // Popping on equal heights must still compute the full width for the last
    // equal bar popped.
    check(&[2, 2, 1, 2, 2], 5);
    check(&[1, 1], 2);
}

#[test]
fn increasing() {
    check(&[1, 2, 3, 4, 5], 9);
}

#[test]
fn decreasing() {
    check(&[5, 4, 3, 2, 1], 9);
}

#[test]
fn widest_low_rectangle_wins() {
    check(&[1, 1, 1, 1, 1, 1, 6], 7);
    check(&[2, 1, 2], 3);
}

#[test]
fn tallest_bar_alone_wins() {
    check(&[1, 100, 1], 100);
}

#[test]
fn zero_splits_histogram() {
    check(&[4, 4, 0, 3, 3, 3], 9);
}

#[test]
fn width_must_extend_left_past_popped_bars_trap() {
    // The height-2 rectangle spans all of [3,5,2,...] once 5 and 3 are popped:
    // width must start after the previous stack element, not at the popped index.
    check(&[3, 5, 2, 4, 2], 10);
    check(&[4, 2, 0, 3, 2, 5], 6);
}

#[test]
fn product_overflows_u32_trap() {
    // 2^24 * 256 = 2^32: one more than u32::MAX.
    check(&[65_536, 65_536, 65_536, 65_536], 4 * 65_536);
    check(&[1 << 24; 256], 1 << 32);
    check(&[u32::MAX, 1, u32::MAX, u32::MAX, u32::MAX], 3 * u64::from(u32::MAX));
}

#[test]
fn randomized_vs_brute() {
    let mut rng = Rng(0x2701_feed_beef);
    for _ in 0..500 {
        let n = rng.below(26) as usize;
        let hi = [4, 11, 10_001, u64::from(u32::MAX) + 1][rng.below(4) as usize];
        let h: Vec<u32> = (0..n).map(|_| rng.below(hi) as u32).collect();
        assert_eq!(largest_rectangle_area(&h), brute_area(&h), "heights = {h:?}");
    }
}

#[test]
fn large_flat_staircase_and_valley_inputs() {
    // Four inputs of 18_000 bars. Trying every range, widening each bar until
    // a shorter one, or splitting at the minimum each cost about 10^8 steps on
    // them; a stack pushes and pops each bar once.
    let n = 18_000u32;
    let h = vec![u32::MAX; n as usize];
    assert_eq!(
        largest_rectangle_area(&h),
        u64::from(n) * u64::from(u32::MAX),
        "all equal to u32::MAX, n = {n}"
    );
    // heights[i] = i / 10. For height h the rectangle spans indices 10h..n,
    // so area = h * (n - 10h), best at h = n / 20.
    let best = u64::from(n / 20) * u64::from(n / 2);
    let up: Vec<u32> = (0..n).map(|i| i / 10).collect();
    assert_eq!(largest_rectangle_area(&up), best, "staircase up, n = {n}");
    let down: Vec<u32> = up.iter().rev().copied().collect();
    assert_eq!(largest_rectangle_area(&down), best, "staircase down, n = {n}");
    // `peak` down to 1 then back up to `peak`, five times. Height 1 spans all
    // n bars; where an "up" meets the next "down", bars >= k span
    // 2 * (peak + 1 - k), best at k = peak / 2.
    let peak = n / 10;
    let mut valley = Vec::new();
    for _ in 0..5 {
        valley.extend((1..=peak).rev());
        valley.extend(1..=peak);
    }
    let k = u64::from(peak / 2);
    let best = (2 * k * (u64::from(peak) + 1 - k)).max(u64::from(n));
    assert_eq!(largest_rectangle_area(&valley), best, "valley, n = {n}");
}
