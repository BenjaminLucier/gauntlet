// Merge Sort — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no sort/sort_unstable/sort_by*/select_nth_unstable, no BinaryHeap/BTreeMap.
#![allow(unused_imports, unused_variables, dead_code)]

use std::cmp::Ordering;
use std::mem;

/// Stable top-down merge sort.
pub fn merge_sort<T: Ord + Clone>(v: &mut [T]) {
    todo!()
}

/// Stable top-down merge sort ordered by `compare`.
pub fn merge_sort_by<T: Clone, F>(v: &mut [T], compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
