// Regular Expression Matching — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

use std::fmt;
use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternError {
    /// A `*` with nothing to repeat; `at` is its byte offset in the pattern.
    NothingToRepeat { at: usize },
}

impl fmt::Display for PatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatternError::NothingToRepeat { at } => {
                write!(f, "`*` at byte {at} has nothing to repeat")
            }
        }
    }
}

impl std::error::Error for PatternError {}

pub fn is_match(s: &str, p: &str) -> Result<bool, PatternError> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
