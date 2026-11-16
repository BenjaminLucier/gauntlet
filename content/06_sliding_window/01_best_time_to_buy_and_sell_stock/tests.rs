// Tests for Best Time to Buy and Sell Stock — shared by template.rs, solution.rs and your attempt.
use super::*;

fn check(prices: &[u32], expected: u32) {
    let got = max_profit(prices);
    if prices.len() <= 50 {
        assert_eq!(got, expected, "prices = {prices:?}");
    } else {
        assert_eq!(got, expected, "prices.len() = {}", prices.len());
    }
}

/// O(n²) oracle: try every buy/sell pair.
fn brute_max_profit(p: &[u32]) -> u32 {
    let mut best = 0;
    for i in 0..p.len() {
        for j in i + 1..p.len() {
            best = best.max(p[j].saturating_sub(p[i]));
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
    check(&[7, 1, 5, 3, 6, 4], 5);
}

#[test]
fn example_2() {
    check(&[7, 6, 4, 3, 1], 0);
}

#[test]
fn example_buy_before_sell() {
    check(&[2, 10, 1, 4], 8);
}

#[test]
fn empty() {
    check(&[], 0);
}

#[test]
fn single_day() {
    check(&[5], 0);
}

#[test]
fn two_days() {
    check(&[1, 2], 1);
    check(&[2, 1], 0);
}

#[test]
fn flat_prices() {
    check(&[3, 3, 3, 3], 0);
}

#[test]
fn global_min_after_global_max() {
    // Trap: max(prices) - min(prices) ignores order (would give 9).
    check(&[2, 10, 1, 4], 8);
}

#[test]
fn new_min_later_gives_better_profit() {
    // Trap: committing to the first local minimum.
    check(&[3, 5, 1, 7], 6);
}

#[test]
fn new_min_later_but_old_profit_wins() {
    // Trap: resetting the best profit when a new minimum appears.
    check(&[1, 9, 0, 3], 8);
}

#[test]
fn increasing() {
    check(&[1, 2, 3, 4, 5], 4);
}

#[test]
fn extreme_values() {
    check(&[10_000, 0, 10_000], 10_000);
    check(&[0, u32::MAX], u32::MAX);
    check(&[u32::MAX, 0], 0);
    check(&[u32::MAX, u32::MAX], 0);
}

#[test]
fn unsigned_underflow_trap() {
    // Trap: computing price - cheapest before updating cheapest.
    check(&[5, 3], 0);
    check(&[1, u32::MAX, 0], u32::MAX - 1);
}

#[test]
fn randomized_against_brute() {
    let mut rng = Rng(0x3C6E_F372_FE94_F82B);
    for _ in 0..400 {
        let n = rng.range(0, 25) as usize;
        let max = match rng.range(0, 2) {
            0 => 10,
            1 => 10_000,
            _ => i64::from(u32::MAX),
        };
        let p: Vec<u32> = (0..n).map(|_| rng.range(0, max) as u32).collect();
        check(&p, brute_max_profit(&p));
    }
}

#[test]
fn large_falling_with_jump_at_the_end() {
    // Falling prices, so no profit until the final day's jump. Trying every
    // (buy, sell) pair is ~n^2 / 2 = 6 * 10^8 pairs here; one pass is n steps.
    let n = 35_000u32;
    let step = 10;
    let mut p: Vec<u32> = (0..n).map(|i| 50_000 - i / step).collect();
    let low = p[n as usize - 2];
    p[n as usize - 1] = low + step;
    assert_eq!(max_profit(&p), step, "n = {n}, step = {step}");
}

#[test]
fn best_at_ends() {
    let n = 1_000;
    let mut p = vec![5_000; n];
    p[1] = 0;
    p[n - 1] = 10_000;
    p[n / 2] = 9_999;
    check(&p, 10_000);
}
