// Consumers and Folds — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no for, while or loop; no unwrap/expect. Iterator consumers only.
#![allow(unused_imports, unused_variables, dead_code)]

use std::collections::{BTreeSet, HashMap};

// ── Arithmetic ──────────────────────────────────────────────────────────────

/// The sum, as an `i64` so it can't overflow. (`map(i64::from)`, `sum`)
pub fn total(xs: &[i32]) -> i64 {
    todo!()
}

/// `n!` (`0! = 1`). `n` is at most 20. (`product` over a range)
pub fn factorial(n: u32) -> u64 {
    todo!()
}

/// The arithmetic mean, or `None` for an empty slice. (`sum`, `bool::then`)
pub fn mean(xs: &[f64]) -> Option<f64> {
    todo!()
}

// ── Counting and testing ────────────────────────────────────────────────────

/// How many characters are ASCII vowels (either case). (`filter`, `count`)
pub fn count_vowels(s: &str) -> usize {
    todo!()
}

/// Is any value negative? (`any`)
pub fn has_negative(xs: &[i32]) -> bool {
    todo!()
}

/// Is every character an ASCII lowercase letter? (`true` for `""`.) (`all`)
pub fn all_lowercase(s: &str) -> bool {
    todo!()
}

/// Do the two strings hold the same words in the same order, ignoring how
/// much whitespace separates them? Compare lazily, without collecting. (`eq`)
pub fn same_words(a: &str, b: &str) -> bool {
    todo!()
}

// ── Extremes ────────────────────────────────────────────────────────────────

/// The shortest whitespace-separated word (by bytes); the *first* one on
/// ties. (`min_by_key`)
pub fn shortest_word(s: &str) -> Option<&str> {
    todo!()
}

/// The longest whitespace-separated word (by bytes); the *last* one on
/// ties. (`max_by_key`)
pub fn longest_word(s: &str) -> Option<&str> {
    todo!()
}

/// The smallest and largest value in one pass. (`split_first`, `fold` with a
/// tuple)
pub fn min_max(xs: &[i32]) -> Option<(i32, i32)> {
    todo!()
}

// ── Folding ─────────────────────────────────────────────────────────────────

/// The number whose decimal digits these are, most significant first. There
/// are at most 19 digits, so the result fits in a `u64`. (`fold`)
pub fn to_number(digits: &[u8]) -> u64 {
    todo!()
}

/// The words joined by `-`, or `None` if there are none — without `join`.
/// (`reduce`)
pub fn hyphenate(words: &[&str]) -> Option<String> {
    todo!()
}

/// Each name applied to the rest: `["f", "g", "x"]` → `"f(g(x))"`. One name
/// is just that name; no names give `""`. Build it from the right.
/// (`split_last`, `rfold`)
pub fn nest(names: &[&str]) -> String {
    todo!()
}

// ── Searching ───────────────────────────────────────────────────────────────

/// The index of the first `target`. (`position`)
pub fn index_of(xs: &[i32], target: i32) -> Option<usize> {
    todo!()
}

/// The index of the last `target`. (`rposition`)
pub fn last_index_of(xs: &[i32], target: i32) -> Option<usize> {
    todo!()
}

/// The first word with at least `min_len` bytes. (`find`)
pub fn first_long_word<'a>(words: &[&'a str], min_len: usize) -> Option<&'a str> {
    todo!()
}

/// The last value before the first 0 (or before the end, if there's no 0).
/// (`take_while`, `last`)
pub fn last_before_zero(xs: &[i32]) -> Option<i32> {
    todo!()
}

/// The `n`-th whitespace-separated word, counting from 0. (`nth`)
pub fn nth_word(s: &str, n: usize) -> Option<&str> {
    todo!()
}

// ── Splitting ───────────────────────────────────────────────────────────────

/// The even values and the odd values, each in order. (`partition`)
pub fn split_even_odd(xs: &[i32]) -> (Vec<i32>, Vec<i32>) {
    todo!()
}

/// The characters as a string and the numbers as a list. (`unzip`)
pub fn split_pairs(pairs: &[(char, u32)]) -> (String, Vec<u32>) {
    todo!()
}

// ── Collecting ──────────────────────────────────────────────────────────────

/// Each word mapped to its length in characters. (`collect` into a
/// `HashMap`)
pub fn word_lengths<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    todo!()
}

/// The first character of each name, skipping empty names. (`filter_map`,
/// `collect` into a `String`)
pub fn initials(names: &[&str]) -> String {
    todo!()
}

/// The values without duplicates, in sorted order. (`collect` into a
/// `BTreeSet`)
pub fn distinct_sorted(xs: &[i32]) -> BTreeSet<i32> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
