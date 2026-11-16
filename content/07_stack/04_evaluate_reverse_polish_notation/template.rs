// Evaluate Reverse Polish Notation — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RpnError {
    /// A token that is neither an operator nor an integer that fits in `i32`.
    InvalidToken(String),
    /// An operator found fewer than two values to work on.
    MissingOperand,
    /// A `/` whose right-hand operand is 0.
    DivisionByZero,
    /// An operation whose result doesn't fit in `i32`.
    Overflow,
    /// No tokens at all.
    Empty,
    /// More than one value left after the last token.
    TooManyOperands,
}

pub fn eval_rpn(tokens: &[&str]) -> Result<i32, RpnError> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
