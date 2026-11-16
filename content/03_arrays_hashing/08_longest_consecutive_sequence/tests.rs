// Tests for Longest Consecutive Sequence — shared by template.rs, solution.rs and your attempt.
use super::*;

fn check(nums: &[i32], expected: usize) {
    let got = longest_consecutive(nums);
    if nums.len() <= 50 {
        assert_eq!(got, expected, "nums = {nums:?}");
    } else {
        assert_eq!(got, expected, "nums.len() = {}", nums.len());
    }
}

/// Sort-based oracle, computed in i64 so it can't overflow.
fn brute_longest(nums: &[i32]) -> usize {
    let mut v = nums.to_vec();
    v.sort_unstable();
    v.dedup();
    let (mut best, mut cur) = (0, 0);
    for i in 0..v.len() {
        cur = if i > 0 && i64::from(v[i - 1]) + 1 == i64::from(v[i]) {
            cur + 1
        } else {
            1
        };
        best = best.max(cur);
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

/// Deterministic Fisher–Yates shuffle.
fn shuffle(v: &mut [i32], rng: &mut Rng) {
    for i in (1..v.len()).rev() {
        let j = rng.range(0, i as i64) as usize;
        v.swap(i, j);
    }
}

#[test]
fn example_1() {
    check(&[100, 4, 200, 1, 3, 2], 4);
}

#[test]
fn example_2() {
    check(&[0, 3, 7, 2, 5, 8, 4, 6, 0, 1], 9);
}

#[test]
fn example_3() {
    check(&[1, 0, 1, 2], 3);
}

#[test]
fn example_top_of_range() {
    check(&[i32::MAX - 1, i32::MAX], 2);
}

#[test]
fn empty() {
    check(&[], 0);
}

#[test]
fn single() {
    check(&[42], 1);
}

#[test]
fn all_duplicates() {
    // Trap: counting duplicates as part of the run.
    check(&[5, 5, 5, 5], 1);
}

#[test]
fn duplicates_inside_run() {
    check(&[3, 2, 2, 1, 3, 4, 4], 4);
}

#[test]
fn no_two_consecutive() {
    check(&[10, 20, 30, 2, 4, 6], 1);
}

#[test]
fn negatives_crossing_zero() {
    check(&[-3, -1, 1, -2, 0, 2, 9], 6);
}

#[test]
fn extremes() {
    check(
        &[
            -1_000_000_000,
            1_000_000_000,
            999_999_999,
            -999_999_999,
            -999_999_998,
        ],
        3,
    );
}

#[test]
fn overflow_at_i32_max() {
    // Trap: `x + 1` panics when x == i32::MAX.
    check(&[i32::MAX], 1);
    check(&[i32::MAX, i32::MAX - 2, i32::MAX - 1, 0], 3);
}

#[test]
fn overflow_at_i32_min() {
    // Trap: `x - 1` panics when x == i32::MIN.
    check(&[i32::MIN], 1);
    check(&[i32::MIN + 1, i32::MIN, 5], 2);
}

#[test]
fn min_and_max_are_not_adjacent() {
    // Trap: wrapping arithmetic makes i32::MAX + 1 == i32::MIN.
    check(&[i32::MIN, i32::MAX], 1);
    check(&[i32::MAX - 1, i32::MAX, i32::MIN, i32::MIN + 1], 2);
}

#[test]
fn two_runs_pick_longer() {
    check(&[1, 2, 3, 10, 11, 12, 13, 14, 4], 5);
}

#[test]
fn randomized_against_brute() {
    let mut rng = Rng(0x2468_ACE0_1357_9BDF);
    for _ in 0..400 {
        let n = rng.range(0, 25) as usize;
        let spread = rng.range(1, 30);
        // Sometimes centre the values next to an end of the i32 range.
        let centre = match rng.range(0, 2) {
            0 => 0,
            1 => i64::from(i32::MAX) - spread,
            _ => i64::from(i32::MIN) + spread,
        };
        let nums: Vec<i32> = (0..n)
            .map(|_| (centre + rng.range(-spread, spread)) as i32)
            .collect();
        check(&nums, brute_longest(&nums));
    }
}

#[test]
fn large_shuffled_single_run() {
    // One run of N values in random order. Counting upward from *every*
    // element, not just from the start of a run, is ~N^2 / 2 = 2.5 * 10^7
    // lookups here; counting from run starts only is ~1.4 * 10^4.
    let n = 7_000;
    let mut rng = Rng(0xFEED_FACE_CAFE_BEEF);
    let mut nums: Vec<i32> = (0..n).map(|i| i - n / 2).collect();
    shuffle(&mut nums, &mut rng);
    check(&nums, n as usize);
}

#[test]
fn descending_run_with_duplicates() {
    let mut nums: Vec<i32> = (0..500).rev().map(|i| i * 3).collect(); // gaps of 3
    nums.extend((0..800).rev().map(|i| 1_000_000_000 - i)); // run of 800 ending at 10^9
    nums.extend((0..800).map(|i| 1_000_000_000 - i)); // the same run again
    check(&nums, 800);
}

#[test]
fn run_ending_at_i32_max() {
    let nums: Vec<i32> = (0..1_000).map(|i| i32::MAX - i).collect();
    check(&nums, 1_000);
}
