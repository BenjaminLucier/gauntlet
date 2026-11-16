// Multiply Strings — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumberError {
    Empty,
    InvalidDigit(char),
}

pub fn multiply(a: &str, b: &str) -> Result<String, NumberError> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
