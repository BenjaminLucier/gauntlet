// Merge k Sorted Lists — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub type List = Option<Box<ListNode>>;

pub fn merge_k_lists(lists: Vec<List>) -> List {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
