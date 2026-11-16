// Gas Station — reference solution.
// If total gas >= total cost an answer exists; whenever the running tank goes
// negative, the next station becomes the candidate start. i64 for the sums.
// Time O(n), space O(1).

pub fn can_complete_circuit(gas: &[u32], cost: &[u32]) -> Option<usize> {
    assert_eq!(gas.len(), cost.len(), "one cost per station");
    let (mut total, mut tank, mut start) = (0i64, 0i64, 0usize);
    for (i, (&g, &c)) in gas.iter().zip(cost).enumerate() {
        let surplus = i64::from(g) - i64::from(c);
        total += surplus;
        tank += surplus;
        if tank < 0 {
            tank = 0;
            start = i + 1;
        }
    }
    (!gas.is_empty() && total >= 0).then_some(start)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
