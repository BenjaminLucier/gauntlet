// Remove Nth Node From End of List — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub type List = Option<Box<ListNode>>;

pub fn remove_nth_from_end(head: List, n: usize) -> List {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
