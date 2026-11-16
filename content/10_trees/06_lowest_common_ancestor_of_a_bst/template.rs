// Lowest Common Ancestor of a Binary Search Tree — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

pub type Tree = Option<Box<TreeNode>>;

pub fn lowest_common_ancestor(root: Option<&TreeNode>, p: i32, q: i32) -> Option<&TreeNode> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
