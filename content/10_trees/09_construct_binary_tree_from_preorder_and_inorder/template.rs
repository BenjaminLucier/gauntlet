// Construct Binary Tree from Preorder and Inorder Traversal — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

use std::collections::{HashMap, HashSet};
use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

pub type Tree = Option<Box<TreeNode>>;

/// Why two traversals can't be turned into a tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildError {
    /// The traversals have different lengths.
    LengthMismatch { preorder: usize, inorder: usize },
    /// This value occurs more than once in one of the traversals.
    DuplicateValue(i32),
    /// No binary tree has both traversals.
    Inconsistent,
}

pub fn build_tree(preorder: &[i32], inorder: &[i32]) -> Result<Tree, BuildError> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
