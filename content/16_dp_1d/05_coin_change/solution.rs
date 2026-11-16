// Coin Change — reference solution.
// Bottom-up DP over amounts: best[a] = 1 + min(best[a - c]) over coins c <= a,
// with None marking unreachable amounts (no sentinel value).
// Time O(amount * coins), space O(amount).

pub fn coin_change(coins: &[u32], amount: u32) -> Option<usize> {
    let amount = amount as usize;
    let mut best: Vec<Option<usize>> = vec![None; amount + 1];
    best[0] = Some(0);
    for a in 1..=amount {
        best[a] = coins
            .iter()
            .filter_map(|&coin| a.checked_sub(coin as usize))
            .filter_map(|rest| best[rest])
            .min()
            .map(|fewest| fewest + 1);
    }
    best[amount]
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
