// Result Combinators — reference solution.
// One combinator (occasionally two) per function; `?` with `From` where errors
// change type. Everything is O(length of the input).

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

pub fn parse_double(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>().map(|n| n * 2)
}

pub fn describe_error(r: Result<u8, ParseIntError>) -> Result<u8, String> {
    r.map_err(|e| e.to_string())
}

pub fn parse_or_tag(s: &str) -> Result<u32, ConfigError> {
    s.parse().map_err(ConfigError::BadNumber)
}

// ── Chaining ────────────────────────────────────────────────────────────────

pub fn parse_even(s: &str) -> Result<u32, String> {
    s.parse::<u32>()
        .map_err(|e| e.to_string())
        .and_then(|n| if n % 2 == 0 { Ok(n) } else { Err(format!("{n} is odd")) })
}

pub fn parse_with_fallback(
    primary: &str,
    fallback: impl FnOnce() -> Result<i32, ParseIntError>,
) -> Result<i32, ParseIntError> {
    primary.parse().or_else(|_| fallback())
}

pub fn first_ok<T, E>(a: Result<T, E>, b: Result<T, E>) -> Result<T, E> {
    a.or(b)
}

pub fn then_second<T, U, E>(a: Result<T, E>, b: Result<U, E>) -> Result<U, E> {
    a.and(b)
}

// ── Defaults ────────────────────────────────────────────────────────────────

pub fn parse_or_zero(s: &str) -> i32 {
    s.parse().unwrap_or_default()
}

pub fn render(r: Result<i32, String>) -> String {
    r.map_or_else(|e| format!("error: {e}"), |v| format!("value: {v}"))
}

// ── Inspecting ──────────────────────────────────────────────────────────────

pub fn is_small_number(s: &str) -> bool {
    s.parse::<i32>().is_ok_and(|n| (-9..=9).contains(&n))
}

pub fn failed_because_empty(r: &Result<i32, ParseIntError>) -> bool {
    r.as_ref().is_err_and(|e| *e.kind() == IntErrorKind::Empty)
}

pub fn parse_logging(s: &str, mut log: impl FnMut(String)) -> Result<i32, ParseIntError> {
    s.parse()
        .inspect(|n| log(format!("ok {n}")))
        .inspect_err(|e| log(format!("err {e}")))
}

// ── To Option and borrowing ─────────────────────────────────────────────────

pub fn ok_value(r: Result<i32, String>) -> Option<i32> {
    r.ok()
}

pub fn err_value(r: Result<i32, String>) -> Option<String> {
    r.err()
}

pub fn borrow_message(r: &Result<String, String>) -> Result<&str, &str> {
    r.as_deref().map_err(String::as_str)
}

// ── The ? operator ──────────────────────────────────────────────────────────

pub fn parse_and_add(a: &str, b: &str) -> Result<i32, ParseIntError> {
    Ok(a.parse::<i32>()? + b.parse::<i32>()?)
}

pub fn parse_port(s: &str) -> Result<u16, ConfigError> {
    let n: i64 = s.trim().parse()?;
    u16::try_from(n)
        .ok()
        .filter(|&p| p != 0)
        .ok_or(ConfigError::OutOfRange(n))
}

pub fn parse_setting(line: &str) -> Result<(String, i64), ConfigError> {
    let (key, value) = line.split_once('=').ok_or(ConfigError::MissingEquals)?;
    let key = key.trim();
    if key.is_empty() {
        return Err(ConfigError::EmptyKey);
    }
    Ok((key.to_string(), value.trim().parse()?))
}

// ── Result<(), E> ───────────────────────────────────────────────────────────

pub fn check_all_below(values: &[i64], max: i64) -> Result<(), ConfigError> {
    for &v in values {
        if v > max {
            return Err(ConfigError::OutOfRange(v));
        }
    }
    Ok(())
}

pub fn apply_settings(lines: &[&str], config: &mut HashMap<String, i64>) -> Result<(), ConfigError> {
    for line in lines {
        let (key, value) = parse_setting(line)?;
        config.insert(key, value);
    }
    Ok(())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
