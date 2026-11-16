// Product of Array Except Self — reference solution.
// Write prefix products into the output, then multiply in a running suffix
// product from the right. Time O(n), extra space O(1).

pub fn product_except_self(nums: &[i32]) -> Vec<i32> {
    let mut out = vec![1; nums.len()];

    // out[i] = product of nums[..i]
    let mut prefix = 1;
    for (slot, &x) in out.iter_mut().zip(nums) {
        *slot = prefix;
        prefix *= x;
    }

    // multiply in the product of nums[i + 1..]
    let mut suffix = 1;
    for (slot, &x) in out.iter_mut().zip(nums).rev() {
        *slot *= suffix;
        suffix *= x;
    }
    out
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
