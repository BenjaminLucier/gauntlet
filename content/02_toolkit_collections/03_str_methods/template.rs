// str Methods — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no for/while/loop and no .chars()/.bytes() scanning where a str method does the job.
#![allow(unused_imports, unused_variables, dead_code)]

use std::num::ParseIntError;

// ── Splitting ───────────────────────────────────────────────────────────────

/// The words, separated by any run of whitespace. (`split_whitespace`)
pub fn words(s: &str) -> Vec<&str> {
    todo!()
}

/// The comma-separated fields, each trimmed. Empty fields stay.
/// (`split`, `trim`)
pub fn csv_fields(line: &str) -> Vec<&str> {
    todo!()
}

/// `"key = value"` → `("key", "value")`, split at the first `=` and trimmed.
/// (`split_once`)
pub fn key_value(s: &str) -> Option<(&str, &str)> {
    todo!()
}

/// What follows the last `.`; `None` if there's no dot, or if the name starts
/// with its only dot (`".bashrc"` has no extension). (`rsplit_once`)
pub fn extension(file_name: &str) -> Option<&str> {
    todo!()
}

/// Split on `:` into at most `n` pieces; the last piece keeps the rest.
/// `n == 0` gives no pieces at all. (`splitn`)
pub fn first_fields(s: &str, n: usize) -> Vec<&str> {
    todo!()
}

/// The lines that aren't empty or all whitespace, without their line endings
/// (`\n` or `\r\n`). (`lines`)
pub fn non_blank_lines(text: &str) -> Vec<&str> {
    todo!()
}

/// Split on any of `,` `;` or space, dropping empty pieces. (`split` with an
/// array of chars as the pattern)
pub fn tokens(s: &str) -> Vec<&str> {
    todo!()
}

// ── Trimming and stripping ──────────────────────────────────────────────────

/// Remove one pair of surrounding double quotes, if both are there.
/// (`strip_prefix`, `strip_suffix`)
pub fn unquote(s: &str) -> &str {
    todo!()
}

/// Remove leading zeros, but leave a single `"0"` for an all-zero number.
/// (`trim_start_matches`)
pub fn trim_leading_zeros(s: &str) -> &str {
    todo!()
}

/// Trim ASCII punctuation from both ends. (`trim_matches` with a closure)
pub fn trim_punctuation(s: &str) -> &str {
    todo!()
}

/// Everything before the first `#`, with trailing whitespace trimmed.
/// (`find`, `trim_end`)
pub fn strip_comment(line: &str) -> &str {
    todo!()
}

// ── Searching ───────────────────────────────────────────────────────────────

/// Does the string start with an ASCII digit? (`starts_with` with a closure)
pub fn starts_with_digit(s: &str) -> bool {
    todo!()
}

/// The part of a `/`-separated path after the last slash. (`rfind`)
pub fn base_name(path: &str) -> &str {
    todo!()
}

/// Non-overlapping occurrences of `needle` (not empty). (`matches`)
pub fn count_occurrences(haystack: &str, needle: &str) -> usize {
    todo!()
}

/// The byte offsets of the ASCII vowels (either case). (`char_indices`)
pub fn vowel_offsets(s: &str) -> Vec<usize> {
    todo!()
}

// ── Transforming ────────────────────────────────────────────────────────────

/// Every occurrence of `word` replaced by `"***"`. (`replace`)
pub fn censor(s: &str, word: &str) -> String {
    todo!()
}

/// Only the first occurrence of `from` replaced by `to`. (`replacen`)
pub fn replace_first(s: &str, from: &str, to: &str) -> String {
    todo!()
}

/// Upper-cased (full Unicode rules), with a `!` on the end. (`to_uppercase`)
pub fn shout(s: &str) -> String {
    todo!()
}

/// Equal ignoring ASCII case? (`eq_ignore_ascii_case`)
pub fn same_command(a: &str, b: &str) -> bool {
    todo!()
}

/// A line of `=` as long as the title in characters (not bytes). (`repeat`)
pub fn underline(title: &str) -> String {
    todo!()
}

// ── Parsing ─────────────────────────────────────────────────────────────────

/// Whitespace-separated integers; the first bad one is the error.
/// (`parse`, collecting into `Result`)
pub fn parse_numbers(s: &str) -> Result<Vec<i32>, ParseIntError> {
    todo!()
}

/// `"3, -4"` → `(3, -4)`; anything malformed is `None`.
/// (`split_once`, `parse::<i32>`)
pub fn parse_point(s: &str) -> Option<(i32, i32)> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
