// Kth Largest Element in an Array — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn kth_largest<T: Ord + Copy>(items: &[T], k: usize) -> Option<T> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
