// Result Combinators — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no match or if let on a Result; no unwrap/expect. Combinators and ? only.
#![allow(unused_imports, unused_variables, dead_code)]

use std::collections::HashMap;
use std::num::{IntErrorKind, ParseIntError};

/// Everything that can go wrong reading a `key = value` setting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    /// The line has no `=`.
    MissingEquals,
    /// The key (left of `=`) is blank.
    EmptyKey,
    /// The value isn't an integer.
    BadNumber(ParseIntError),
    /// The value parsed but is outside the allowed range.
    OutOfRange(i64),
}

impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::BadNumber(e)
    }
}

// ── Transforming ────────────────────────────────────────────────────────────

/// Parse an `i32` and double it; parse errors pass through. Inputs are small
/// enough that doubling can't overflow. (`map`)
pub fn parse_double(s: &str) -> Result<i32, ParseIntError> {
    todo!()
}

/// Turn the error into its `Display` text; leave `Ok` alone. (`map_err`)
pub fn describe_error(r: Result<u8, ParseIntError>) -> Result<u8, String> {
    todo!()
}

/// Parse a `u32`, wrapping a failure in `ConfigError::BadNumber`. (`map_err`
/// with a variant used as a function)
pub fn parse_or_tag(s: &str) -> Result<u32, ConfigError> {
    todo!()
}

// ── Chaining ────────────────────────────────────────────────────────────────

/// Parse a `u32` that must be even. A parse failure becomes its `Display`
/// text; an odd number becomes `"<n> is odd"`. (`map_err`, `and_then`)
pub fn parse_even(s: &str) -> Result<u32, String> {
    todo!()
}

/// Parse `primary`; if that fails, return what `fallback()` gives instead.
/// `fallback` is expensive: call it only when `primary` fails. (`or_else`)
pub fn parse_with_fallback(
    primary: &str,
    fallback: impl FnOnce() -> Result<i32, ParseIntError>,
) -> Result<i32, ParseIntError> {
    todo!()
}

/// `a` if it's `Ok`, otherwise `b`. (`or`)
pub fn first_ok<T, E>(a: Result<T, E>, b: Result<T, E>) -> Result<T, E> {
    todo!()
}

/// `b` if `a` is `Ok`; otherwise `a`'s error. (`and`)
pub fn then_second<T, U, E>(a: Result<T, E>, b: Result<U, E>) -> Result<U, E> {
    todo!()
}

// ── Defaults ────────────────────────────────────────────────────────────────

/// Parse an `i32`, or 0 if it doesn't parse. (`unwrap_or_default`)
pub fn parse_or_zero(s: &str) -> i32 {
    todo!()
}

/// `"value: <v>"` or `"error: <e>"`. (`map_or_else`)
pub fn render(r: Result<i32, String>) -> String {
    todo!()
}

// ── Inspecting ──────────────────────────────────────────────────────────────

/// Does `s` parse as an `i32` between -9 and 9? (`is_ok_and`)
pub fn is_small_number(s: &str) -> bool {
    todo!()
}

/// Did the parse fail because the input was empty? Don't consume `r`.
/// (`as_ref`, `is_err_and`, `ParseIntError::kind`)
pub fn failed_because_empty(r: &Result<i32, ParseIntError>) -> bool {
    todo!()
}

/// Parse an `i32`, calling `log` with `"ok <n>"` or `"err <message>"`, and
/// return the parse result unchanged. Calling an `FnMut` needs a mutable
/// binding: write `mut log`. (`inspect`, `inspect_err`)
pub fn parse_logging(s: &str, log: impl FnMut(String)) -> Result<i32, ParseIntError> {
    todo!()
}

// ── To Option and borrowing ─────────────────────────────────────────────────

/// The value, dropping the error. (`ok`)
pub fn ok_value(r: Result<i32, String>) -> Option<i32> {
    todo!()
}

/// The error, dropping the value. (`err`)
pub fn err_value(r: Result<i32, String>) -> Option<String> {
    todo!()
}

/// Borrow both sides as `&str` without cloning. (`as_deref`, `map_err`)
pub fn borrow_message(r: &Result<String, String>) -> Result<&str, &str> {
    todo!()
}

// ── The ? operator ──────────────────────────────────────────────────────────

/// Parse both and add them; the first parse error wins. (`?`)
pub fn parse_and_add(a: &str, b: &str) -> Result<i32, ParseIntError> {
    todo!()
}

/// Parse a TCP port (1..=65535) from `s` (surrounding whitespace allowed).
/// Not an `i64` → `BadNumber` (via `?` and `From`), which includes numbers
/// too big for an `i64`; an `i64` outside the range → `OutOfRange(n)`.
/// (`?`, `u16::try_from`, `ok_or`)
pub fn parse_port(s: &str) -> Result<u16, ConfigError> {
    todo!()
}

/// Parse `key = value` (spaces around either side are trimmed) into
/// `(key, value)`. No `=` → `MissingEquals`; blank key → `EmptyKey`; bad
/// value → `BadNumber`. (`split_once`, `ok_or`, `?`)
pub fn parse_setting(line: &str) -> Result<(String, i64), ConfigError> {
    todo!()
}

// ── Result<(), E> ───────────────────────────────────────────────────────────

/// `Ok(())` if every value is `<= max`; otherwise `OutOfRange` with the first
/// value that isn't. (early `return Err(..)`)
pub fn check_all_below(values: &[i64], max: i64) -> Result<(), ConfigError> {
    todo!()
}

/// Parse each line with `parse_setting` and insert it into `config`, stopping
/// at the first bad line. Lines before it stay applied. (`?` in a loop)
pub fn apply_settings(lines: &[&str], config: &mut HashMap<String, i64>) -> Result<(), ConfigError> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
