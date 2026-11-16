// Option Combinators — reference solution.
// One combinator (occasionally two) per function; no matching by hand.

use std::collections::HashMap;
use std::num::ParseIntError;

// ── Transforming ────────────────────────────────────────────────────────────

pub fn double(x: Option<i32>) -> Option<i32> {
    x.map(|n| n * 2)
}

pub fn len_or_zero(s: Option<&str>) -> usize {
    s.map_or(0, str::len)
}

pub fn first_char_upper(s: &str) -> Option<char> {
    s.chars().next().map(|c| c.to_ascii_uppercase())
}

// ── Defaults ────────────────────────────────────────────────────────────────

pub fn name_or_anonymous(name: Option<&str>) -> &str {
    name.unwrap_or("anonymous")
}

pub fn or_else_build(x: Option<String>, build: impl FnOnce() -> String) -> String {
    x.unwrap_or_else(build)
}

// ── Filtering and chaining ──────────────────────────────────────────────────

pub fn even_only(x: Option<u32>) -> Option<u32> {
    x.filter(|n| n % 2 == 0)
}

pub fn parse_positive(s: &str) -> Option<u32> {
    s.parse().ok().filter(|&n| n > 0)
}

pub fn lookup_twice<'a>(map: &HashMap<&str, &'a str>, key: &str) -> Option<&'a str> {
    map.get(key).and_then(|mid| map.get(mid)).copied()
}

pub fn flatten_nested(x: Option<Option<i32>>) -> Option<i32> {
    x.flatten()
}

// ── Combining ───────────────────────────────────────────────────────────────

pub fn first_some<T>(a: Option<T>, b: Option<T>) -> Option<T> {
    a.or(b)
}

pub fn exactly_one<T>(a: Option<T>, b: Option<T>) -> Option<T> {
    a.xor(b)
}

pub fn add_both(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    a.zip(b).map(|(a, b)| a + b)
}

pub fn split_pair(x: Option<(i32, char)>) -> (Option<i32>, Option<char>) {
    x.unzip()
}

// ── Inspecting ──────────────────────────────────────────────────────────────

pub fn is_positive(x: Option<i32>) -> bool {
    x.is_some_and(|n| n > 0)
}

// ── Slots ───────────────────────────────────────────────────────────────────

pub fn take_out(slot: &mut Option<String>) -> Option<String> {
    slot.take()
}

pub fn swap_in(slot: &mut Option<i32>, value: i32) -> Option<i32> {
    slot.replace(value)
}

pub fn get_or_start(slot: &mut Option<Vec<i32>>) -> &mut Vec<i32> {
    slot.get_or_insert_with(Vec::new)
}

// ── Borrowing ───────────────────────────────────────────────────────────────

pub fn as_name(name: &Option<String>) -> Option<&str> {
    name.as_deref()
}

// ── To and from Result ──────────────────────────────────────────────────────

pub fn require(x: Option<u8>) -> Result<u8, &'static str> {
    x.ok_or("missing")
}

pub fn parse_if_present(s: Option<&str>) -> Result<Option<i32>, ParseIntError> {
    s.map(str::parse).transpose()
}

// ── Many at once ────────────────────────────────────────────────────────────

pub fn sum_all(xs: &[Option<i32>]) -> Option<i32> {
    xs.iter().copied().sum()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
