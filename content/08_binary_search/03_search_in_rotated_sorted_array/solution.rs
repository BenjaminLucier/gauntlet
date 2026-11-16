// Search in Rotated Sorted Array — reference solution.
// One binary search over a half-open window; at each step the half on one side
// of mid is sorted, so check whether the target lies in it and discard the
// other half. Time O(log n), space O(1).

pub fn search_rotated<T: Ord>(nums: &[T], target: &T) -> Option<usize> {
    let (mut lo, mut hi) = (0, nums.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if &nums[mid] == target {
            return Some(mid);
        }
        if nums[lo] <= nums[mid] {
            // [lo, mid] is sorted.
            if (&nums[lo]..&nums[mid]).contains(&target) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        } else if &nums[mid] < target && target <= &nums[hi - 1] {
            // [mid, hi) is sorted and holds the target's value range.
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    None
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
