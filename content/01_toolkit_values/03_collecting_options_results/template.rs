// Collecting Options and Results — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no for, while or loop; no unwrap/expect. Iterator chains only.
#![allow(unused_imports, unused_variables, dead_code)]

use std::collections::{BTreeSet, HashMap};
use std::num::ParseIntError;
use std::ops::ControlFlow;

// ── Collecting ──────────────────────────────────────────────────────────────

/// Parse every word, or return the first error. (`collect` into `Result<Vec>`)
pub fn parse_all(words: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    todo!()
}

/// All the values if every entry is `Some`; `None` otherwise.
/// (`collect` into `Option<Vec>`)
pub fn all_present(xs: &[Option<u8>]) -> Option<Vec<u8>> {
    todo!()
}

/// Look every key up; `None` if any is missing. (`map`, `collect` into
/// `Option<Vec>`)
pub fn lookup_all(map: &HashMap<&str, i32>, keys: &[&str]) -> Option<Vec<i32>> {
    todo!()
}

/// Parse every word into a sorted set without duplicates, or return the first
/// error. (`collect` into `Result<BTreeSet>`)
pub fn parse_unique(words: &[&str]) -> Result<BTreeSet<u32>, ParseIntError> {
    todo!()
}

/// The string made of all the characters, or `None` if any is missing.
/// (`collect` into `Option<String>`)
pub fn collect_string(chars: &[Option<char>]) -> Option<String> {
    todo!()
}

/// Apply `f` to each item, stopping at (and returning) the first error. `f`
/// must not be called on the items after it. (`map`, `collect` into
/// `Result<Vec>`)
pub fn try_map<T, U, E>(items: &[T], f: impl FnMut(&T) -> Result<U, E>) -> Result<Vec<U>, E> {
    todo!()
}

/// Parse one `u32` per line, ignoring surrounding whitespace and blank lines.
/// (`lines`, `filter`, `collect` into `Result<Vec>`)
pub fn parse_lines(text: &str) -> Result<Vec<u32>, ParseIntError> {
    todo!()
}

// ── Summing ─────────────────────────────────────────────────────────────────

/// The sum of the parsed words, or the first parse error. (`Sum for Result`)
pub fn sum_parsed(words: &[&str]) -> Result<i64, ParseIntError> {
    todo!()
}

/// The product, or `None` if any entry is missing. The product of the
/// present values always fits in a `u32`. (`Product for Option`)
pub fn product_present(xs: &[Option<u32>]) -> Option<u32> {
    todo!()
}

// ── Folding with an exit ────────────────────────────────────────────────────

/// The sum of the values before the first negative one (all of them if none
/// is negative). The values after it are never looked at. (`try_fold` with
/// `ControlFlow::Break` / `Continue`, then take the value out of either)
pub fn sum_until_negative(xs: &[i32]) -> i32 {
    todo!()
}

/// Apply each transaction to `start`. `Ok(final balance)` if it never goes
/// below zero, otherwise `Err(index of the transaction that took it below)`.
/// Balances stay far from the `i64` limits.
/// (`enumerate`, `try_fold`)
pub fn running_balance(start: i64, txns: &[i64]) -> Result<i64, usize> {
    todo!()
}

/// `Ok(())` if every value is non-negative; otherwise `Err(first negative)`.
/// (`try_for_each`)
pub fn all_non_negative(xs: &[i32]) -> Result<(), i32> {
    todo!()
}

// ── Keeping the good ones ───────────────────────────────────────────────────

/// Every word that parses, in order; the rest are skipped. (`filter_map`, `ok`)
pub fn valid_numbers(words: &[&str]) -> Vec<i32> {
    todo!()
}

/// The `Some` values, in order. (`flatten`)
pub fn present_values(xs: &[Option<i32>]) -> Vec<i32> {
    todo!()
}

/// The `Ok` values, in order. (`into_iter`, `flatten`)
pub fn ok_values(rs: Vec<Result<i32, String>>) -> Vec<i32> {
    todo!()
}

// ── Finding ─────────────────────────────────────────────────────────────────

/// The first word that parses. (`find_map`)
pub fn first_number(words: &[&str]) -> Option<i32> {
    todo!()
}

/// The first error message, borrowed. (`find_map`, `as_ref`, `err`)
pub fn first_error(rs: &[Result<i32, String>]) -> Option<&str> {
    todo!()
}

// ── Splitting ───────────────────────────────────────────────────────────────

/// The parsed numbers and the words that didn't parse, each in order.
/// (`partition` on `is_ok`, then `flatten` / `filter_map(Result::err)`)
pub fn split_parsed<'a>(words: &[&'a str]) -> (Vec<i32>, Vec<&'a str>) {
    todo!()
}

// ── Nesting ─────────────────────────────────────────────────────────────────

/// Parse each field that is present; a missing field stays `None`. Any parse
/// error fails the whole thing. (`map`, `transpose`, `collect`)
pub fn parse_optional_fields(fields: &[Option<&str>]) -> Result<Vec<Option<i32>>, ParseIntError> {
    todo!()
}

/// The largest parsed number (`Ok(None)` for no words), or the first parse
/// error. (`collect` into `Result<Vec>`, then `Result::map`)
pub fn max_parsed(words: &[&str]) -> Result<Option<i32>, ParseIntError> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
