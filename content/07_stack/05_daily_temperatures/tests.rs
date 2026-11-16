// Tests for Daily Temperatures — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Write expected answers compactly: 0 means "no warmer day" (None).
fn waits(raw: &[usize]) -> Vec<Option<usize>> {
    raw.iter().map(|&d| (d > 0).then_some(d)).collect()
}

fn check(t: &[i32], expected: &[usize]) {
    assert_eq!(daily_temperatures(t), waits(expected), "temperatures = {t:?}");
}

/// Oracle: scan forward from every day. O(n^2).
fn brute_daily(t: &[i32]) -> Vec<Option<usize>> {
    (0..t.len()).map(|i| (i + 1..t.len()).find(|&j| t[j] > t[i]).map(|j| j - i)).collect()
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

    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        lo + self.below((hi - lo + 1) as u64) as i32
    }
}

#[test]
fn example_1() {
    assert_eq!(
        daily_temperatures(&[73, 74, 75, 71, 69, 72, 76, 73]),
        [Some(1), Some(1), Some(4), Some(2), Some(1), Some(1), None, None],
    );
}

#[test]
fn example_2() {
    check(&[30, 40, 50, 60], &[1, 1, 1, 0]);
}

#[test]
fn example_3() {
    check(&[30, 60, 90], &[1, 1, 0]);
}

#[test]
fn empty_input() {
    assert_eq!(daily_temperatures(&[]), []);
}

#[test]
fn single_day() {
    assert_eq!(daily_temperatures(&[50]), [None]);
}

#[test]
fn two_days() {
    check(&[30, 31], &[1, 0]);
    check(&[31, 30], &[0, 0]);
}

#[test]
fn equal_is_not_warmer_trap() {
    check(&[70, 70, 70], &[0, 0, 0]);
    check(&[70, 70, 71], &[2, 1, 0]);
}

#[test]
fn strictly_decreasing() {
    check(&[100, 90, 80, 70, 60, 50], &[0, 0, 0, 0, 0, 0]);
}

#[test]
fn valley_resolves_many_at_once() {
    check(&[80, 70, 60, 50, 90], &[4, 3, 2, 1, 0]);
}

#[test]
fn answer_is_distance_not_temperature_or_index() {
    check(&[55, 38, 53, 81, 61, 93, 97, 32, 43, 78], &[3, 1, 1, 2, 1, 1, 0, 1, 1, 0]);
}

#[test]
fn extreme_temperatures() {
    check(&[30, 100, 30, 100], &[1, 0, 1, 0]);
    check(&[100, 30, 30, 30], &[0, 0, 0, 0]);
    check(&[-100, 100, -100, -99], &[1, 0, 1, 0]);
}

#[test]
fn negative_temperatures() {
    check(&[-5, -10, -3, -3, 0, -1], &[2, 1, 2, 1, 0, 0]);
}

#[test]
fn last_day_is_always_none() {
    let got = daily_temperatures(&[1, 2, 3, 4, 5]);
    assert_eq!(got.last(), Some(&None));
    assert_eq!(got.len(), 5);
}

#[test]
fn randomized_vs_brute() {
    let mut rng = Rng(0x2501_5eed_0001);
    for _ in 0..400 {
        let n = rng.range(0, 30) as usize;
        let hi = if rng.below(2) == 0 { 35 } else { 100 };
        let t: Vec<i32> = (0..n).map(|_| rng.range(30, hi)).collect();
        assert_eq!(daily_temperatures(&t), brute_daily(&t), "temperatures = {t:?}");
    }
}

#[test]
fn large_inputs_with_no_warmer_day_until_the_end() {
    // Three inputs of 15_000 days where a forward scan from each day runs to
    // the end: about 10^8 steps per input, where a stack touches each day at
    // most twice.
    let n = 15_000;
    let none = vec![None; n];
    // All equal: no day ever gets warmer.
    assert_eq!(daily_temperatures(&vec![75; n]), none, "all equal, n = {n}");
    // Cooling slowly, with long plateaus: still no warmer day.
    let cooling: Vec<i32> = (0..n).map(|i| 100 - (200 * i / n) as i32).collect();
    assert_eq!(daily_temperatures(&cooling), none, "cooling, n = {n}");
    // The only warmer day is the last.
    let mut t = vec![30; n];
    t[n - 1] = 31;
    let expected: Vec<Option<usize>> = (0..n).map(|i| (i < n - 1).then_some(n - 1 - i)).collect();
    assert_eq!(daily_temperatures(&t), expected, "30 then 31 on the last day, n = {n}");
}

#[test]
fn large_sawtooth() {
    // 100, 99, ..., 30 repeated: within a block nothing is warmer except the
    // next block's first day (100) for everything but each block's first day.
    let block: Vec<i32> = (30..=100).rev().collect();
    let len = block.len();
    let reps = 100_000 / len;
    let t: Vec<i32> = block.iter().cycle().take(len * reps).copied().collect();
    let expected: Vec<Option<usize>> = (0..t.len())
        .map(|i| {
            let pos = i % len;
            (pos != 0 && i / len != reps - 1).then_some(len - pos)
        })
        .collect();
    assert_eq!(daily_temperatures(&t), expected);
}
