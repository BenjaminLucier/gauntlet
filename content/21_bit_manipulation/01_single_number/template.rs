// Single Number — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

use std::ops::BitXor;

pub fn single_number<T: BitXor<Output = T> + Copy>(nums: &[T]) -> Option<T> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
