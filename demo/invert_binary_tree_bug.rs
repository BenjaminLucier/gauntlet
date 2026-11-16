// Invert Binary Tree — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.

use std::mem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

pub type Tree = Option<Box<TreeNode>>;

pub fn invert_tree(mut root: Tree) -> Tree {
    // Swaps the root's children, but forgets to mirror below them.
    if let Some(node) = root.as_deref_mut() {
        mem::swap(&mut node.left, &mut node.right);
    }
    root
}
