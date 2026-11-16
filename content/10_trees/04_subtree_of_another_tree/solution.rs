// Subtree of Another Tree — reference solution.
// Visit every node of `root` with an explicit stack and compare the subtree
// there with `sub` using the derived structural `==`.
// Time O(n * m) worst case, space O(h).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

pub type Tree = Option<Box<TreeNode>>;

pub fn is_subtree(root: Option<&TreeNode>, sub: Option<&TreeNode>) -> bool {
    // The empty tree hangs under every leaf, so it is a subtree of anything.
    let Some(sub) = sub else { return true };
    let mut stack: Vec<&TreeNode> = root.into_iter().collect();
    while let Some(node) = stack.pop() {
        // Derived `PartialEq` compares shape and values recursively,
        // stopping at the first difference.
        if node == sub {
            return true;
        }
        stack.extend(node.left.as_deref());
        stack.extend(node.right.as_deref());
    }
    false
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
