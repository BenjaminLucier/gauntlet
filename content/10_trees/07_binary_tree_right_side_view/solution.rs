// Binary Tree Right Side View — reference solution.
// Level-order BFS over borrowed nodes, keeping the last value of each level.
// Time O(n), space O(w) where w is the widest level.

use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

pub type Tree = Option<Box<TreeNode>>;

pub fn right_side_view(root: Option<&TreeNode>) -> Vec<i32> {
    let mut view = vec![];
    let mut queue: VecDeque<&TreeNode> = root.into_iter().collect();
    while let Some(rightmost) = queue.back() {
        view.push(rightmost.val);
        for _ in 0..queue.len() {
            let node = queue.pop_front().expect("the level has this many nodes");
            queue.extend(node.left.as_deref());
            queue.extend(node.right.as_deref());
        }
    }
    view
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
