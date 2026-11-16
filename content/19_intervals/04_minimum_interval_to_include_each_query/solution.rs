// Minimum Interval to Include Each Query — reference solution.
// Answer queries offline in sorted order: push intervals with left <= q into a
// min-heap by (size, right) and lazily pop those with right < q; the top is
// the answer. Time O((n + q) log(n + q)), space O(n + q).

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn min_interval_sizes(mut intervals: Vec<(i32, i32)>, queries: &[i32]) -> Vec<Option<u32>> {
    intervals.sort_unstable();
    let mut order: Vec<usize> = (0..queries.len()).collect();
    order.sort_unstable_by_key(|&i| queries[i]);

    let mut pending = intervals.into_iter().peekable();
    let mut active: BinaryHeap<Reverse<(u32, i32)>> = BinaryHeap::new();
    let mut answers = vec![None; queries.len()];
    for i in order {
        let q = queries[i];
        while let Some((left, right)) = pending.next_if(|&(left, _)| left <= q) {
            active.push(Reverse((right.abs_diff(left) + 1, right)));
        }
        while active.peek().is_some_and(|&Reverse((_, right))| right < q) {
            active.pop();
        }
        answers[i] = active.peek().map(|&Reverse((size, _))| size);
    }
    answers
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
