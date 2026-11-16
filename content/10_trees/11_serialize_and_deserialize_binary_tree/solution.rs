// Serialize and Deserialize Binary Tree — reference solution.
// Level order with `#` for every missing child; decoding replays the same BFS,
// reading two tokens per dequeued node. No recursion, so path-shaped trees are
// fine. Time O(n), space O(n) in both directions.

use std::collections::VecDeque;
use std::fmt::{self, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

pub type Tree = Option<Box<TreeNode>>;

/// Why a string isn't a valid encoded tree. Indices count comma-separated
/// tokens from 0.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The token at `index` is neither `#` nor an `i32`.
    BadToken { index: usize },
    /// The input ended while some node's children were still unread.
    UnexpectedEnd,
    /// The tree was complete, but more tokens follow, starting at `index`.
    TrailingData { index: usize },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::BadToken { index } => write!(f, "token {index} is not `#` or an i32"),
            ParseError::UnexpectedEnd => write!(f, "input ended before the tree was complete"),
            ParseError::TrailingData { index } => {
                write!(f, "unexpected data after the tree, from token {index}")
            }
        }
    }
}

impl std::error::Error for ParseError {}

pub fn serialize(root: Option<&TreeNode>) -> String {
    let mut out = String::new();
    let mut queue = VecDeque::from([root]);
    while let Some(node) = queue.pop_front() {
        if !out.is_empty() {
            out.push(',');
        }
        match node {
            Some(n) => {
                write!(out, "{}", n.val).expect("writing to a String cannot fail");
                queue.push_back(n.left.as_deref());
                queue.push_back(n.right.as_deref());
            }
            None => out.push('#'),
        }
    }
    out
}

pub fn deserialize(data: &str) -> Result<Tree, ParseError> {
    let mut tokens = data.split(',').enumerate();
    let mut next = || -> Result<Tree, ParseError> {
        let (index, token) = tokens.next().ok_or(ParseError::UnexpectedEnd)?;
        if token == "#" {
            return Ok(None);
        }
        let val = token.parse().map_err(|_| ParseError::BadToken { index })?;
        Ok(Some(Box::new(TreeNode { val, left: None, right: None })))
    };

    let mut root = next()?;
    // Nodes whose children haven't been read yet, in level order. They borrow
    // from `root`, so the queue must be gone before `root` is returned.
    let mut queue: VecDeque<&mut TreeNode> = root.as_deref_mut().into_iter().collect();
    while let Some(node) = queue.pop_front() {
        node.left = next()?;
        node.right = next()?;
        queue.extend(node.left.as_deref_mut());
        queue.extend(node.right.as_deref_mut());
    }

    match tokens.next() {
        Some((index, _)) => Err(ParseError::TrailingData { index }),
        None => Ok(root),
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
