// Longest Consecutive Sequence — reference solution.
// HashSet of values; count upward only from run starts (x - 1 absent), so
// every value is touched O(1) times. Checked arithmetic keeps i32::MIN/MAX
// safe. Time O(n) expected, space O(n).

use std::collections::HashSet;

pub fn longest_consecutive(nums: &[i32]) -> usize {
    let set: HashSet<i32> = nums.iter().copied().collect();
    let contains = |x: Option<i32>| x.is_some_and(|x| set.contains(&x));
    set.iter()
        .filter(|&&x| !contains(x.checked_sub(1)))
        .map(|&start| {
            let mut len = 1;
            let mut x = start;
            while contains(x.checked_add(1)) {
                x += 1;
                len += 1;
            }
            len
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
