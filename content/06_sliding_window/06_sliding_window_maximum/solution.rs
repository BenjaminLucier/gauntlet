// Sliding Window Maximum — reference solution.
// Monotonic deque of indices with strictly decreasing values: drop smaller or
// equal values from the back, expired indices from the front; the front is the
// window maximum. Time O(n), space O(k).

use std::collections::VecDeque;
use std::num::NonZeroUsize;

pub fn max_sliding_window<T: Ord + Copy>(nums: &[T], k: NonZeroUsize) -> Vec<T> {
    let k = k.get();
    let mut deque: VecDeque<usize> = VecDeque::with_capacity(k.min(nums.len()));
    let mut out = Vec::with_capacity(nums.len().saturating_sub(k - 1));
    for (i, &x) in nums.iter().enumerate() {
        while deque.back().is_some_and(|&j| nums[j] <= x) {
            deque.pop_back();
        }
        deque.push_back(i);
        // `i - j` rather than `j + k`: k may be as large as usize::MAX.
        if deque.front().is_some_and(|&j| i - j >= k) {
            deque.pop_front();
        }
        if i + 1 >= k {
            let &front = deque.front().expect("deque holds at least index i");
            out.push(nums[front]);
        }
    }
    out
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
