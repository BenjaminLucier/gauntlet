// Diameter of Binary Tree — reference solution.
// One post-order DFS returns each subtree's height and, on the way, records
// the longest path bending at that node (left height + right height).
// Time O(n), space O(h).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

pub type Tree = Option<Box<TreeNode>>;

pub fn diameter_of_binary_tree(root: Option<&TreeNode>) -> usize {
    /// Height of `node` in nodes; records the best path bending at each node.
    fn height(node: Option<&TreeNode>, best: &mut usize) -> usize {
        let Some(node) = node else { return 0 };
        let left = height(node.left.as_deref(), best);
        let right = height(node.right.as_deref(), best);
        *best = (*best).max(left + right);
        1 + left.max(right)
    }

    let mut best = 0;
    height(root, &mut best);
    best
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
