// Chars and UTF-8 — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

use std::iter;
use std::str::Utf8Error;

// ── Lengths ─────────────────────────────────────────────────────────────────

/// `(bytes, chars)`: `"日本"` is `(6, 2)`. (`len`, `chars().count()`)
pub fn byte_and_char_len(s: &str) -> (usize, usize) {
    todo!()
}

/// How many bytes `c` takes in UTF-8: 1 to 4. (`len_utf8`)
pub fn utf8_width(c: char) -> usize {
    todo!()
}

/// Is every character ASCII? (`is_ascii`)
pub fn is_plain_ascii(s: &str) -> bool {
    todo!()
}

// ── Indexing by character ───────────────────────────────────────────────────

/// The `n`-th character (0-based), if there is one. (`chars().nth`)
pub fn char_at(s: &str, n: usize) -> Option<char> {
    todo!()
}

/// The first `n` characters (the whole string if it's shorter), as a slice
/// of `s`. (`char_indices`)
pub fn char_prefix(s: &str, n: usize) -> &str {
    todo!()
}

/// Characters `start..end` (by character index, not byte) as a slice of `s`;
/// `None` if the range is out of bounds or backwards. (`char_indices`, `get`)
pub fn char_slice(s: &str, start: usize, end: usize) -> Option<&str> {
    todo!()
}

/// Bytes `start..end` — but `None` instead of a panic if either end is out of
/// range or splits a character. (`str::get`)
pub fn byte_slice(s: &str, start: usize, end: usize) -> Option<&str> {
    todo!()
}

/// The longest prefix of at most `max_bytes` bytes that doesn't cut a
/// character in half. (`floor_char_boundary`)
pub fn truncate_bytes(s: &str, max_bytes: usize) -> &str {
    todo!()
}

// ── Classifying and converting chars ────────────────────────────────────────

/// The value of `c` as a digit in `radix` (2..=36): `'f'` in 16 is 15.
/// (`char::to_digit`)
pub fn digit_value(c: char, radix: u32) -> Option<u32> {
    todo!()
}

/// The character for digit `d` in `radix` (2..=36), lower-case for letters.
/// (`char::from_digit`)
pub fn digit_char(d: u32, radix: u32) -> Option<char> {
    todo!()
}

/// The sum of the decimal digits in `s`, ignoring everything else.
/// (`to_digit`)
pub fn sum_digits(s: &str) -> u32 {
    todo!()
}

/// Only the alphanumeric characters — Unicode letters and digits count.
/// (`char::is_alphanumeric`)
pub fn alnum_only(s: &str) -> String {
    todo!()
}

/// `(ASCII letters, ASCII digits, ASCII whitespace)`.
/// (`is_ascii_alphabetic`, `is_ascii_digit`, `is_ascii_whitespace`)
pub fn ascii_counts(s: &str) -> (usize, usize, usize) {
    todo!()
}

/// Rotate ASCII letters 13 places, keeping case; everything else is left
/// alone. (`u8` arithmetic, `as char`)
pub fn rot13(s: &str) -> String {
    todo!()
}

// ── Bytes and code points ───────────────────────────────────────────────────

/// The UTF-8 encoding of `c`. (`encode_utf8`)
pub fn utf8_bytes(c: char) -> Vec<u8> {
    todo!()
}

/// Borrow the bytes as a `&str` if they're valid UTF-8. (`str::from_utf8`)
pub fn decode(bytes: &[u8]) -> Result<&str, Utf8Error> {
    todo!()
}

/// Decode, replacing each invalid sequence with `U+FFFD` (`�`).
/// (`String::from_utf8_lossy`)
pub fn decode_lossy(bytes: &[u8]) -> String {
    todo!()
}

/// The Unicode scalar value of each character. (`u32::from(char)`)
pub fn code_points(s: &str) -> Vec<u32> {
    todo!()
}

/// The character with that code point, if it is one (surrogates and values
/// above `0x10FFFF` aren't). (`char::from_u32`)
pub fn from_code_point(u: u32) -> Option<char> {
    todo!()
}

/// Decode Latin-1, where every byte is the code point of the same value.
/// (`u8 as char`)
pub fn latin1_to_string(bytes: &[u8]) -> String {
    todo!()
}

/// The characters in reverse order. (`chars().rev()`)
pub fn reverse_chars(s: &str) -> String {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
