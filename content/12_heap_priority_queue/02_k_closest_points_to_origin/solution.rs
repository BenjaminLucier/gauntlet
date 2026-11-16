// K Closest Points to Origin — reference solution.
// Quickselect on a copy (`select_nth_unstable_by_key`) by squared distance in
// u64, then keep the first k. Time O(n) average, space O(n) for the copy.

fn dist2((x, y): (i32, i32)) -> u64 {
    u64::from(x.unsigned_abs()).pow(2) + u64::from(y.unsigned_abs()).pow(2)
}

pub fn k_closest(points: &[(i32, i32)], k: usize) -> Vec<(i32, i32)> {
    let mut out = points.to_vec();
    if k < out.len() {
        out.select_nth_unstable_by_key(k, |&p| dist2(p));
        out.truncate(k);
    }
    out
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
