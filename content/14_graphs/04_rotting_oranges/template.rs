// Rotting Oranges — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Fresh,
    Rotten,
}

pub fn oranges_rotting(grid: &[Vec<Cell>]) -> Option<usize> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
