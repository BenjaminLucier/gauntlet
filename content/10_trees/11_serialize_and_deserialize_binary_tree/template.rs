// Serialize and Deserialize Binary Tree — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

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
    todo!()
}

pub fn deserialize(data: &str) -> Result<Tree, ParseError> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
