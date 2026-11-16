// Validate Binary Search Tree — reference solution.
// DFS passing down the open interval each node must lie in; bounds are
// `Option<i32>` so the full i32 range works without sentinels.
// Time O(n), space O(h) for the recursion.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

pub type Tree = Option<Box<TreeNode>>;

pub fn is_valid_bst(root: Option<&TreeNode>) -> bool {
    fn within(node: Option<&TreeNode>, low: Option<i32>, high: Option<i32>) -> bool {
        let Some(node) = node else { return true };
        let v = node.val;
        low.is_none_or(|low| low < v)
            && high.is_none_or(|high| v < high)
            && within(node.left.as_deref(), low, Some(v))
            && within(node.right.as_deref(), Some(v), high)
    }
    within(root, None, None)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
