// Maximum Subarray — reference solution.
// Kadane: the best sum ending at x is max(x, best ending before + x); `scan`
// produces those running bests and `fold` keeps the largest. Time O(n), space O(1).

pub fn max_subarray_sum(nums: &[i32]) -> Option<i32> {
    let (&first, rest) = nums.split_first()?;
    let best = rest
        .iter()
        .scan(first, |ending_here, &x| {
            *ending_here = x.max(*ending_here + x);
            Some(*ending_here)
        })
        .fold(first, i32::max);
    Some(best)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
