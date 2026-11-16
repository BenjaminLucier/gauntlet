// Option Combinators — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no match, if let, let-else or ?; no unwrap/expect. Combinators only.
#![allow(unused_imports, unused_variables, dead_code)]

use std::collections::HashMap;
use std::num::ParseIntError;

// ── Transforming ────────────────────────────────────────────────────────────

/// Double the value, if there is one. (`map`)
pub fn double(x: Option<i32>) -> Option<i32> {
    todo!()
}

/// The length of the string, or 0 for `None`. (`map_or`)
pub fn len_or_zero(s: Option<&str>) -> usize {
    todo!()
}

/// The first character, upper-cased (ASCII). `None` for an empty string. (`map`)
pub fn first_char_upper(s: &str) -> Option<char> {
    todo!()
}

// ── Defaults ────────────────────────────────────────────────────────────────

/// The name, or `"anonymous"`. (`unwrap_or`)
pub fn name_or_anonymous(name: Option<&str>) -> &str {
    todo!()
}

/// The value, or whatever `build` makes — calling `build` only if needed.
/// (`unwrap_or_else`)
pub fn or_else_build(x: Option<String>, build: impl FnOnce() -> String) -> String {
    todo!()
}

// ── Filtering and chaining ──────────────────────────────────────────────────

/// Keep the value only if it's even. (`filter`)
pub fn even_only(x: Option<u32>) -> Option<u32> {
    todo!()
}

/// Parse a strictly positive number; anything else is `None`. (`ok`, `filter`)
pub fn parse_positive(s: &str) -> Option<u32> {
    todo!()
}

/// Look `key` up, then look the result up again: `a → b → c` gives `c`.
/// (`and_then`)
pub fn lookup_twice<'a>(map: &HashMap<&str, &'a str>, key: &str) -> Option<&'a str> {
    todo!()
}

/// `Some(Some(x))` → `Some(x)`; anything else → `None`. (`flatten`)
pub fn flatten_nested(x: Option<Option<i32>>) -> Option<i32> {
    todo!()
}

// ── Combining ───────────────────────────────────────────────────────────────

/// `a` if it's `Some`, otherwise `b`. (`or`)
pub fn first_some<T>(a: Option<T>, b: Option<T>) -> Option<T> {
    todo!()
}

/// Whichever is `Some` if exactly one of them is; `None` otherwise. (`xor`)
pub fn exactly_one<T>(a: Option<T>, b: Option<T>) -> Option<T> {
    todo!()
}

/// `a + b` if both are present. (`zip`, `map`)
pub fn add_both(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    todo!()
}

/// Split an optional pair into a pair of options. (`unzip`)
pub fn split_pair(x: Option<(i32, char)>) -> (Option<i32>, Option<char>) {
    todo!()
}

// ── Inspecting ──────────────────────────────────────────────────────────────

/// Is there a value, and is it positive? (`is_some_and`)
pub fn is_positive(x: Option<i32>) -> bool {
    todo!()
}

// ── Slots ───────────────────────────────────────────────────────────────────

/// Move the value out of the slot, leaving `None`. (`take`)
pub fn take_out(slot: &mut Option<String>) -> Option<String> {
    todo!()
}

/// Put `value` in the slot and return what was there before. (`replace`)
pub fn swap_in(slot: &mut Option<i32>, value: i32) -> Option<i32> {
    todo!()
}

/// The list in the slot, creating an empty one first if the slot is `None`.
/// (`get_or_insert_with`)
pub fn get_or_start(slot: &mut Option<Vec<i32>>) -> &mut Vec<i32> {
    todo!()
}

// ── Borrowing ───────────────────────────────────────────────────────────────

/// Borrow the name as a `&str` without cloning it. (`as_deref`)
pub fn as_name(name: &Option<String>) -> Option<&str> {
    todo!()
}

// ── To and from Result ──────────────────────────────────────────────────────

/// `Ok(value)`, or `Err("missing")`. (`ok_or`)
pub fn require(x: Option<u8>) -> Result<u8, &'static str> {
    todo!()
}

/// No input is fine (`Ok(None)`); input must parse. (`map`, `transpose`)
pub fn parse_if_present(s: Option<&str>) -> Result<Option<i32>, ParseIntError> {
    todo!()
}

// ── Many at once ────────────────────────────────────────────────────────────

/// The sum if every entry is `Some`; `None` if any is missing. (`Sum`)
pub fn sum_all(xs: &[Option<i32>]) -> Option<i32> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
