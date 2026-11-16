// Sliding Window Maximum — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

use std::collections::VecDeque;
use std::num::NonZeroUsize;

pub fn max_sliding_window<T: Ord + Copy>(nums: &[T], k: NonZeroUsize) -> Vec<T> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
