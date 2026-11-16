// Best Time to Buy and Sell Stock with Cooldown — reference solution.
// State machine over days: best profit while holding / having just sold / resting,
// in i64 because "holding" is negative. Time O(n), space O(1).

pub fn max_profit(prices: &[u32]) -> u64 {
    let Some((&first, later)) = prices.split_first() else {
        return 0;
    };
    let (mut hold, mut sold, mut rest) = (-i64::from(first), 0i64, 0i64);
    for price in later.iter().map(|&p| i64::from(p)) {
        (hold, sold, rest) = (hold.max(rest - price), hold + price, rest.max(sold));
    }
    u64::try_from(sold.max(rest)).expect("resting from day 0 earns 0, so the best is >= 0")
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
