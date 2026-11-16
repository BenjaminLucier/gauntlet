// Construct Binary Tree from Preorder and Inorder Traversal — reference solution.
// Map each value to its inorder index, then recurse on inorder ranges while a
// shared cursor walks preorder; a root outside its range means the traversals
// disagree. Time O(n), space O(n).

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

struct Builder<'a> {
    preorder: &'a [i32],
    next: usize,
    inorder_index: HashMap<i32, usize>,
}

impl Builder<'_> {
    /// Builds the subtree whose inorder traversal is `inorder[range]`.
    fn build(&mut self, range: Range<usize>) -> Result<Tree, BuildError> {
        if range.is_empty() {
            return Ok(None);
        }
        // The non-empty ranges partition 0..n, so at most n values are taken.
        let val = self.preorder[self.next];
        self.next += 1;
        let mid = *self.inorder_index.get(&val).ok_or(BuildError::Inconsistent)?;
        if !range.contains(&mid) {
            return Err(BuildError::Inconsistent);
        }
        let left = self.build(range.start..mid)?;
        let right = self.build(mid + 1..range.end)?;
        Ok(Some(Box::new(TreeNode { val, left, right })))
    }
}

pub fn build_tree(preorder: &[i32], inorder: &[i32]) -> Result<Tree, BuildError> {
    if preorder.len() != inorder.len() {
        return Err(BuildError::LengthMismatch { preorder: preorder.len(), inorder: inorder.len() });
    }
    let mut inorder_index = HashMap::with_capacity(inorder.len());
    for (i, &v) in inorder.iter().enumerate() {
        if inorder_index.insert(v, i).is_some() {
            return Err(BuildError::DuplicateValue(v));
        }
    }
    let mut seen = HashSet::with_capacity(preorder.len());
    if let Some(&v) = preorder.iter().find(|&&v| !seen.insert(v)) {
        return Err(BuildError::DuplicateValue(v));
    }
    Builder { preorder, next: 0, inorder_index }.build(0..inorder.len())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
