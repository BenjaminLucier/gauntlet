// Binary Search & Bounds — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no slice::binary_search*, no slice::partition_point, no sorting or linear scans.
#![allow(unused_imports, unused_variables, dead_code)]

use std::cmp::Ordering;
use std::ops::Range;

/// Index of the first item for which `pred` is false (`items.len()` if none),
/// given that every `true` item comes before every `false` one.
pub fn partition_point<T>(items: &[T], pred: impl FnMut(&T) -> bool) -> usize {
    todo!()
}

/// First index `i` with `items[i] >= target` (`items.len()` if none).
pub fn lower_bound<T: Ord>(items: &[T], target: &T) -> usize {
    todo!()
}

/// First index `i` with `items[i] > target` (`items.len()` if none).
pub fn upper_bound<T: Ord>(items: &[T], target: &T) -> usize {
    todo!()
}

/// The indices of the items equal to `target`: `lower_bound..upper_bound`.
pub fn equal_range<T: Ord>(items: &[T], target: &T) -> Range<usize> {
    todo!()
}

/// Index of the first occurrence of `target`, or `None`.
pub fn binary_search<T: Ord>(items: &[T], target: &T) -> Option<usize> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
