// House Robber II — reference solution.
// The first and last house can't both be robbed, so the answer is the better
// of the linear robber on "all but the first" and "all but the last" (a single
// house is its own case). Time O(n), space O(1).

/// Linear House Robber: best(i) = max(best(i-1), best(i-2) + houses[i]).
fn rob_line(houses: &[u32]) -> u64 {
    let (_, best) = houses.iter().fold((0u64, 0u64), |(skip, take), &cash| {
        (take, take.max(skip + u64::from(cash)))
    });
    best
}

pub fn rob_circle(houses: &[u32]) -> u64 {
    match houses {
        [] => 0,
        [only] => u64::from(*only),
        [_, without_first @ ..] => {
            let without_last = &houses[..houses.len() - 1];
            rob_line(without_first).max(rob_line(without_last))
        }
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
