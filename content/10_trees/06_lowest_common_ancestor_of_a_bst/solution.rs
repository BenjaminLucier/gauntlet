// Lowest Common Ancestor of a Binary Search Tree — reference solution.
// Walk down from the root while both values lie on the same side; the first
// node where they split (or that equals one of them) is the ancestor, provided
// two BST searches from there find both values. Time O(h), space O(1).

use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

pub type Tree = Option<Box<TreeNode>>;

pub fn lowest_common_ancestor(root: Option<&TreeNode>, p: i32, q: i32) -> Option<&TreeNode> {
    let (lo, hi) = (p.min(q), p.max(q));
    let mut cur = root;
    while let Some(node) = cur {
        cur = if hi < node.val {
            node.left.as_deref()
        } else if lo > node.val {
            node.right.as_deref()
        } else {
            // The search paths to p and q part here; it's the answer only if
            // both values really are below.
            let found = contains(Some(node), p) && contains(Some(node), q);
            return found.then_some(node);
        };
    }
    None
}

/// Plain BST search.
fn contains(mut cur: Option<&TreeNode>, v: i32) -> bool {
    while let Some(node) = cur {
        cur = match v.cmp(&node.val) {
            Ordering::Equal => return true,
            Ordering::Less => node.left.as_deref(),
            Ordering::Greater => node.right.as_deref(),
        };
    }
    false
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
