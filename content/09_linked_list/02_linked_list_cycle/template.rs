// Linked List Cycle — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

/// A list node stored in an arena (a slice); `next` is the index of the next node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub next: Option<usize>,
}

pub fn has_cycle(nodes: &[Node], head: Option<usize>) -> bool {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
