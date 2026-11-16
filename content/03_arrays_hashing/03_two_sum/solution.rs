// Two Sum — reference solution.
// One pass with a HashMap value -> index; look up the complement before
// inserting, so an element never pairs with itself. Time O(n), space O(n).

use std::collections::HashMap;

pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut seen: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
    for (j, &n) in nums.iter().enumerate() {
        // |target - n| <= 2 * 10^9 < i32::MAX, so this can't overflow.
        if let Some(&i) = seen.get(&(target - n)) {
            return Some((i, j));
        }
        seen.insert(n, j);
    }
    None
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
