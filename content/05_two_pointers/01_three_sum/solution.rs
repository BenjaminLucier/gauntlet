// 3Sum — reference solution.
// Sort a copy; for each distinct first value run two pointers on the rest,
// skipping repeated values to avoid duplicate triplets.
// Time O(n^2), space O(n) for the copy.

use std::cmp::Ordering;

pub fn three_sum(nums: &[i32]) -> Vec<[i32; 3]> {
    let mut nums = nums.to_vec();
    nums.sort_unstable();
    let n = nums.len();
    let mut out = Vec::new();
    for i in 0..n {
        let a = nums[i];
        if a > 0 {
            break; // everything to the right is positive too
        }
        if i > 0 && a == nums[i - 1] {
            continue;
        }
        let (mut l, mut r) = (i + 1, n - 1);
        while l < r {
            match (a + nums[l] + nums[r]).cmp(&0) {
                Ordering::Less => l += 1,
                Ordering::Greater => r -= 1,
                Ordering::Equal => {
                    out.push([a, nums[l], nums[r]]);
                    l += 1;
                    while l < r && nums[l] == nums[l - 1] {
                        l += 1;
                    }
                    r -= 1;
                }
            }
        }
    }
    out
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
