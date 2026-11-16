// Koko Eating Bananas — reference solution.
// Binary search the smallest speed k in [1, max] with sum(ceil(p / k)) <= h,
// summing hours in u64; impossible exactly when h < n. Time O(n log max), space O(1).

pub fn min_eating_speed(piles: &[u32], h: u64) -> Option<u32> {
    if h < piles.len() as u64 {
        return None;
    }
    let hours_at = |k: u32| -> u64 { piles.iter().map(|&p| u64::from(p.div_ceil(k))).sum() };
    let (mut lo, mut hi) = (1, piles.iter().copied().max().unwrap_or(1));
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if hours_at(mid) <= h {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    Some(lo)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
