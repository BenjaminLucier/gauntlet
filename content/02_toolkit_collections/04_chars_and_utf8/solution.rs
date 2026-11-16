// Chars and UTF-8 — reference solution.
// Byte offsets come from `char_indices` or `floor_char_boundary`, never from
// arithmetic on character counts, so no slice can land inside a character.

use std::iter;
use std::str::Utf8Error;

// ── Lengths ─────────────────────────────────────────────────────────────────

pub fn byte_and_char_len(s: &str) -> (usize, usize) {
    (s.len(), s.chars().count())
}

pub fn utf8_width(c: char) -> usize {
    c.len_utf8()
}

pub fn is_plain_ascii(s: &str) -> bool {
    s.is_ascii()
}

// ── Indexing by character ───────────────────────────────────────────────────

pub fn char_at(s: &str, n: usize) -> Option<char> {
    s.chars().nth(n)
}

pub fn char_prefix(s: &str, n: usize) -> &str {
    s.char_indices().nth(n).map_or(s, |(i, _)| &s[..i])
}

pub fn char_slice(s: &str, start: usize, end: usize) -> Option<&str> {
    let byte_offset = |n: usize| {
        s.char_indices()
            .map(|(i, _)| i)
            .chain(iter::once(s.len()))
            .nth(n)
    };
    s.get(byte_offset(start)?..byte_offset(end)?)
}

pub fn byte_slice(s: &str, start: usize, end: usize) -> Option<&str> {
    s.get(start..end)
}

pub fn truncate_bytes(s: &str, max_bytes: usize) -> &str {
    &s[..s.floor_char_boundary(max_bytes)]
}

// ── Classifying and converting chars ────────────────────────────────────────

pub fn digit_value(c: char, radix: u32) -> Option<u32> {
    c.to_digit(radix)
}

pub fn digit_char(d: u32, radix: u32) -> Option<char> {
    char::from_digit(d, radix)
}

pub fn sum_digits(s: &str) -> u32 {
    s.chars().filter_map(|c| c.to_digit(10)).sum()
}

pub fn alnum_only(s: &str) -> String {
    s.chars().filter(|c| c.is_alphanumeric()).collect()
}

pub fn ascii_counts(s: &str) -> (usize, usize, usize) {
    let count = |pred: fn(&char) -> bool| s.chars().filter(pred).count();
    (
        count(char::is_ascii_alphabetic),
        count(char::is_ascii_digit),
        count(char::is_ascii_whitespace),
    )
}

pub fn rot13(s: &str) -> String {
    let rotate = |c: char, base: u8| ((c as u8 - base + 13) % 26 + base) as char;
    s.chars()
        .map(|c| match c {
            'a'..='z' => rotate(c, b'a'),
            'A'..='Z' => rotate(c, b'A'),
            _ => c,
        })
        .collect()
}

// ── Bytes and code points ───────────────────────────────────────────────────

pub fn utf8_bytes(c: char) -> Vec<u8> {
    c.encode_utf8(&mut [0; 4]).as_bytes().to_vec()
}

pub fn decode(bytes: &[u8]) -> Result<&str, Utf8Error> {
    std::str::from_utf8(bytes)
}

pub fn decode_lossy(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

pub fn code_points(s: &str) -> Vec<u32> {
    s.chars().map(u32::from).collect()
}

pub fn from_code_point(u: u32) -> Option<char> {
    char::from_u32(u)
}

pub fn latin1_to_string(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| b as char).collect()
}

pub fn reverse_chars(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
