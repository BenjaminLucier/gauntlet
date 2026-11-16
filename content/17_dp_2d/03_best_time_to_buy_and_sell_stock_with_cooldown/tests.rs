// Tests for Best Time to Buy and Sell Stock with Cooldown — shared by template.rs, solution.rs and your attempt.
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

/// Exponential: on each day either do nothing or take the allowed action.
fn brute_profit(p: &[u32], day: usize, holding: Option<u32>) -> i64 {
    if day >= p.len() {
        return 0;
    }
    let skip = brute_profit(p, day + 1, holding);
    let act = match holding {
        None => brute_profit(p, day + 1, Some(p[day])),
        // sell today, tomorrow is a cooldown
        Some(bought) => i64::from(p[day]) - i64::from(bought) + brute_profit(p, day + 2, None),
    };
    skip.max(act)
}

/// Independent O(n^2) oracle: best[i] = best profit from day i on while free to buy.
fn quadratic_profit(p: &[u32]) -> u64 {
    let n = p.len();
    let mut best = vec![0i64; n + 2];
    for i in (0..n).rev() {
        let mut b = best[i + 1];
        for j in i + 1..n {
            b = b.max(i64::from(p[j]) - i64::from(p[i]) + best[j + 2]);
        }
        best[i] = b;
    }
    best[0] as u64
}

/// The same recurrence in O(n): carry the best `p[j] + best[j + 2]` over `j > i`.
fn linear_profit(p: &[u32]) -> u64 {
    let n = p.len();
    let mut best = vec![0i64; n + 2];
    let mut top = i64::MIN;
    for i in (0..n).rev() {
        if i + 1 < n {
            top = top.max(i64::from(p[i + 1]) + best[i + 3]);
        }
        best[i] = best[i + 1].max(top.saturating_sub(i64::from(p[i])));
    }
    best[0] as u64
}

#[test]
fn example_1() {
    assert_eq!(max_profit(&[1, 2, 3, 0, 2]), 3);
}

#[test]
fn example_2() {
    assert_eq!(max_profit(&[1]), 0);
}

#[test]
fn empty_prices() {
    assert_eq!(max_profit(&[]), 0);
}

#[test]
fn two_days() {
    assert_eq!(max_profit(&[1, 5]), 4);
    assert_eq!(max_profit(&[5, 1]), 0);
    assert_eq!(max_profit(&[3, 3]), 0);
}

#[test]
fn strictly_decreasing_is_zero() {
    assert_eq!(max_profit(&[9, 7, 5, 3, 1, 0]), 0);
    assert_eq!(max_profit(&[5, 4, 3]), 0);
}

#[test]
fn strictly_increasing_one_long_hold() {
    assert_eq!(max_profit(&[1, 2, 3, 4, 5]), 4);
}

#[test]
fn all_zero_prices() {
    assert_eq!(max_profit(&[0; 10]), 0);
}

#[test]
fn ignoring_cooldown_overcounts() {
    // Summing positive differences gives 4; the cooldown forbids it.
    assert_eq!(max_profit(&[1, 2, 3, 0, 2]), 3);
    assert_eq!(max_profit(&[1, 3, 1, 3]), 2);
}

#[test]
fn cooldown_only_after_sell_not_after_buy() {
    // Buying on day 0 and selling on day 1 is fine.
    assert_eq!(max_profit(&[1, 4]), 3);
    assert_eq!(max_profit(&[1, 4, 0, 0, 4]), 7);
}

#[test]
fn skip_a_small_trade_for_a_bigger_one() {
    // Selling at 2 forces a cooldown on the day the price is 0.
    assert_eq!(max_profit(&[1, 2, 0, 5]), 5);
    assert_eq!(max_profit(&[2, 1, 4]), 3);
}

#[test]
fn two_trades_separated_by_cooldown() {
    assert_eq!(max_profit(&[1, 3, 0, 1, 3]), 4);
    assert_eq!(max_profit(&[6, 1, 6, 4, 3, 0, 2]), 7);
}

#[test]
fn moderate_prices() {
    assert_eq!(max_profit(&[0, 1000, 1000, 0, 1000]), 2000);
}

#[test]
fn extreme_prices_do_not_overflow() {
    // A u32 subtraction would underflow; a u32 total would overflow.
    let max = u32::MAX;
    assert_eq!(max_profit(&[0, max]), u64::from(max));
    assert_eq!(max_profit(&[max, 0]), 0);
    assert_eq!(max_profit(&[0, max, 0, 0, max]), 2 * u64::from(max));
    assert_eq!(max_profit(&[max; 5]), 0);
}

#[test]
fn hold_sentinel_trap() {
    // Seeding "holding" with i64::MIN (or i32::MIN) and adding a price overflows.
    assert_eq!(max_profit(&[u32::MAX]), 0);
    assert_eq!(max_profit(&[7, 3]), 0);
}

#[test]
fn random_vs_brute_force() {
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    for _ in 0..400 {
        let n = rng.below(13) as usize;
        let prices: Vec<u32> = (0..n).map(|_| rng.below(10) as u32).collect();
        let expected = brute_profit(&prices, 0, None) as u64;
        assert_eq!(max_profit(&prices), expected, "prices = {prices:?}");
    }
}

#[test]
fn random_wide_values_vs_quadratic() {
    let mut rng = Rng(0xABCD_EF01_2345_6789);
    for _ in 0..200 {
        let n = rng.below(40) as usize;
        let prices: Vec<u32> = (0..n).map(|_| rng.next() as u32).collect();
        assert_eq!(max_profit(&prices), quadratic_profit(&prices), "prices = {prices:?}");
    }
}

#[test]
fn large_increasing() {
    let prices: Vec<u32> = (0..5000).map(|i| i / 5).collect();
    assert_eq!(max_profit(&prices), 999);
}

#[test]
fn large_zigzag() {
    // 0, 1000, 0, 1000, ...: a trade every 4 days (buy, sell, cooldown, wait).
    let prices: Vec<u32> = (0..5000).map(|i| if i % 2 == 0 { 0 } else { 1000 }).collect();
    let expected = quadratic_profit(&prices);
    assert_eq!(max_profit(&prices), expected);
}

#[test]
fn large_random_vs_quadratic() {
    let mut rng = Rng(99);
    let prices: Vec<u32> = (0..5000).map(|_| rng.below(1001) as u32).collect();
    let expected = quadratic_profit(&prices);
    assert_eq!(max_profit(&prices), expected);
}

#[test]
fn large_total_exceeds_u32() {
    // 4000 days of 0, MAX, 0, 0 repeating: one full-range trade every 4 days.
    let prices: Vec<u32> = (0..4000)
        .map(|i| if i % 4 == 1 { u32::MAX } else { 0 })
        .collect();
    assert_eq!(max_profit(&prices), 1000 * u64::from(u32::MAX));
}

#[test]
fn large_random_inputs() {
    // A DP that tries every sell day j for each buy day i does n^2 / 2 =
    // 1.6 * 10^8 steps per input; carrying the best sell term along is 1.8 * 10^4.
    let n = 18_000;
    let mut rng = Rng(0xC001_D0E5_0003_0017);
    for (round, max) in [1001, 1 << 32].into_iter().enumerate() {
        let prices: Vec<u32> = (0..n).map(|_| rng.below(max) as u32).collect();
        assert_eq!(max_profit(&prices), linear_profit(&prices), "round {round}, n = {n}");
    }
}
