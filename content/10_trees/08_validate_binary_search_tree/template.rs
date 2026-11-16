// Validate Binary Search Tree — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

pub type Tree = Option<Box<TreeNode>>;

pub fn is_valid_bst(root: Option<&TreeNode>) -> bool {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
