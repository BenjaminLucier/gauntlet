// Quicksort — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no sort/sort_unstable/sort_by*/select_nth_unstable, no BinaryHeap/BTreeMap.
#![allow(unused_imports, unused_variables, dead_code)]

use std::cmp::Ordering;
use std::mem;

/// In-place quicksort: 3-way partition, random or median-of-three pivot.
pub fn quicksort<T: Ord>(v: &mut [T]) {
    todo!()
}

/// Rearrange `v` around `p = v[pivot_index]` and return `(lt, gt)` with
/// `v[..lt] < p`, `v[lt..gt] == p`, `v[gt..] > p`.
/// Panics if `pivot_index >= v.len()`.
pub fn partition3<T: Ord>(v: &mut [T], pivot_index: usize) -> (usize, usize) {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
