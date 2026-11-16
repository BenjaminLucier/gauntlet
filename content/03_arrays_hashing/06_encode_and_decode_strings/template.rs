// Encode and Decode Strings — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

use std::fmt::Write;

#[derive(Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// No `#` follows the current position.
    MissingSeparator,
    /// The length before `#` is empty, not all ASCII digits, or too large.
    InvalidLength,
    /// Fewer bytes remain than the length says.
    Truncated,
    /// The length ends inside a multi-byte character.
    NotCharBoundary,
}

pub fn encode<S: AsRef<str>>(strs: &[S]) -> String {
    todo!()
}

pub fn decode(encoded: &str) -> Result<Vec<&str>, DecodeError> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
