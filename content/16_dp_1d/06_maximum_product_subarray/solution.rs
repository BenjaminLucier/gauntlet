// Maximum Product Subarray — reference solution.
// Fold over the slice tracking the largest and smallest product of a subarray
// ending at each index (a negative x swaps their roles), widened to i64.
// Time O(n), space O(1).

pub fn max_product(nums: &[i32]) -> Option<i64> {
    let (&first, rest) = nums.split_first()?;
    let first = i64::from(first);
    let (_, _, best) = rest.iter().fold((first, first, first), |(hi, lo, best), &x| {
        let x = i64::from(x);
        // x * hi and x * lo are products of real subarrays, so they fit.
        let (a, b) = (x * hi, x * lo);
        let hi = x.max(a).max(b);
        let lo = x.min(a).min(b);
        (hi, lo, best.max(hi))
    });
    Some(best)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
