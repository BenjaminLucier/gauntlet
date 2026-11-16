// Tree Traversals — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: the `_iter` functions and `level_order` must not recurse — use a Vec stack / VecDeque queue.
#![allow(unused_imports, unused_variables, dead_code)]

use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

pub type Tree = Option<Box<TreeNode>>;

/// Node, left subtree, right subtree. Recursive.
pub fn preorder(root: Option<&TreeNode>) -> Vec<i32> {
    todo!()
}

/// Left subtree, node, right subtree. Recursive.
pub fn inorder(root: Option<&TreeNode>) -> Vec<i32> {
    todo!()
}

/// Left subtree, right subtree, node. Recursive.
pub fn postorder(root: Option<&TreeNode>) -> Vec<i32> {
    todo!()
}

/// Preorder with an explicit stack — no recursion.
pub fn preorder_iter(root: Option<&TreeNode>) -> Vec<i32> {
    todo!()
}

/// Inorder with an explicit stack — no recursion.
pub fn inorder_iter(root: Option<&TreeNode>) -> Vec<i32> {
    todo!()
}

/// Postorder with an explicit stack — no recursion.
pub fn postorder_iter(root: Option<&TreeNode>) -> Vec<i32> {
    todo!()
}

/// One `Vec` per depth, top to bottom, each left to right. Uses a queue.
pub fn level_order(root: Option<&TreeNode>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
