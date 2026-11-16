// Binary Search & Bounds — reference solution.
// One primitive, partition_point, on a half-open window [lo, hi) that always
// shrinks; the bounds, equal_range and binary_search are one-liners on top of it.
// O(log n) comparisons per call, O(1) space.

use std::ops::Range;

pub fn partition_point<T>(items: &[T], mut pred: impl FnMut(&T) -> bool) -> usize {
    // Invariant: pred holds for items[..lo], fails for items[hi..].
    let (mut lo, mut hi) = (0, items.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if pred(&items[mid]) {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

pub fn lower_bound<T: Ord>(items: &[T], target: &T) -> usize {
    partition_point(items, |x| x < target)
}

pub fn upper_bound<T: Ord>(items: &[T], target: &T) -> usize {
    partition_point(items, |x| x <= target)
}

pub fn equal_range<T: Ord>(items: &[T], target: &T) -> Range<usize> {
    let start = lower_bound(items, target);
    // Everything before `start` is < target, so the upper bound is at or after it.
    let end = start + upper_bound(&items[start..], target);
    start..end
}

pub fn binary_search<T: Ord>(items: &[T], target: &T) -> Option<usize> {
    let i = lower_bound(items, target);
    (items.get(i)? == target).then_some(i)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
