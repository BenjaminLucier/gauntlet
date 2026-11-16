// Jump Game II — reference solution.
// Implicit BFS: indices reachable in k jumps form a range; the farthest
// i + jumps[i] inside it ends the next range. A range that fails to grow means
// the end is unreachable. Time O(n), space O(1).

pub fn min_jumps(jumps: &[usize]) -> Option<usize> {
    let last = jumps.len().saturating_sub(1);
    let (mut count, mut range_end, mut farthest) = (0, 0, 0);
    for (i, &jump) in jumps.iter().enumerate().take(last) {
        farthest = farthest.max(i.saturating_add(jump));
        if i == range_end {
            if farthest == range_end {
                return None; // the next range is empty: stuck
            }
            count += 1;
            range_end = farthest;
            if range_end >= last {
                break;
            }
        }
    }
    (range_end >= last).then_some(count)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
