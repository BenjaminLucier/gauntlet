// Binary Tree Maximum Path Sum — reference solution.
// One post-order DFS returns, for each subtree, the best downward path from its
// root and the best path anywhere inside it; a path bends at exactly one node.
// Time O(n), space O(h).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

pub type Tree = Option<Box<TreeNode>>;

pub fn max_path_sum(root: Option<&TreeNode>) -> Option<i32> {
    root.map(|node| sums(node).1)
}

/// `(best path starting at `node` going down, best path anywhere in the subtree)`.
fn sums(node: &TreeNode) -> (i32, i32) {
    let left = node.left.as_deref().map(sums);
    let right = node.right.as_deref().map(sums);
    // A branch that only loses value is simply left out.
    let gain = |child: Option<(i32, i32)>| child.map_or(0, |(down, _)| down.max(0));
    let (l, r) = (gain(left), gain(right));
    let bend_here = node.val + l + r;
    let best = [left, right]
        .into_iter()
        .flatten()
        .map(|(_, best)| best)
        .fold(bend_here, i32::max);
    (node.val + l.max(r), best)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
