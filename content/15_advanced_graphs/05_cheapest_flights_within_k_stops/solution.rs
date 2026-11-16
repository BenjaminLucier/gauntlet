// Cheapest Flights Within K Stops — reference solution.
// Bellman-Ford for max_stops + 1 rounds, each relaxing from a snapshot of the
// previous round so a round adds at most one flight. Time O(k·E), space O(n).

pub fn find_cheapest_price(
    n: usize,
    flights: &[(usize, usize, u32)],
    src: usize,
    dst: usize,
    max_stops: usize,
) -> Option<u32> {
    // cost[v]: cheapest price to reach v using at most `round` flights.
    let mut cost: Vec<Option<u32>> = vec![None; n];
    cost[src] = Some(0);
    for _round in 0..=max_stops {
        let prev = cost.clone();
        for &(from, to, price) in flights {
            let Some(via) = prev[from].map(|c| c + price) else {
                continue;
            };
            if cost[to].is_none_or(|best| via < best) {
                cost[to] = Some(via);
            }
        }
    }
    cost[dst]
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
