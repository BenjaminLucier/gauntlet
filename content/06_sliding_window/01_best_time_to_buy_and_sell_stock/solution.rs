// Best Time to Buy and Sell Stock — reference solution.
// One fold tracking the cheapest price so far and the best profit so far;
// the minimum includes today's price, so the subtraction can't underflow.
// Time O(n), space O(1).

pub fn max_profit(prices: &[u32]) -> u32 {
    let Some(&first) = prices.first() else {
        return 0;
    };
    let (_, best) = prices.iter().fold((first, 0), |(cheapest, best), &price| {
        let cheapest = cheapest.min(price);
        (cheapest, best.max(price - cheapest))
    });
    best
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
