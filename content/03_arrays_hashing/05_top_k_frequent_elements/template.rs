// Top K Frequent Elements — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

use std::collections::HashMap;
use std::hash::Hash;

pub fn top_k_frequent<T: Hash + Eq + Clone>(items: &[T], k: usize) -> Vec<T> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
