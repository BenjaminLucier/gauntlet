// Reverse Nodes in k-Group — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub type List = Option<Box<ListNode>>;

pub fn reverse_k_group(head: List, k: usize) -> List {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
