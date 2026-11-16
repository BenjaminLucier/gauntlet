// Single Number — reference solution.
// XOR everything together: pairs cancel (a ^ a = 0), leaving the single value.
// `reduce` yields None for an empty slice. Time O(n), space O(1).

use std::ops::BitXor;

pub fn single_number<T: BitXor<Output = T> + Copy>(nums: &[T]) -> Option<T> {
    nums.iter().copied().reduce(|acc, x| acc ^ x)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
