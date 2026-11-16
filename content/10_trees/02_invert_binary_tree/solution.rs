// Invert Binary Tree — reference solution.
// Recursively mirror both subtrees and put them back swapped, reusing the
// existing boxes in place. Time O(n), space O(h) for the recursion.

use std::mem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

pub type Tree = Option<Box<TreeNode>>;

pub fn invert_tree(mut root: Tree) -> Tree {
    if let Some(node) = root.as_deref_mut() {
        mem::swap(&mut node.left, &mut node.right);
        node.left = invert_tree(node.left.take());
        node.right = invert_tree(node.right.take());
    }
    root
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
