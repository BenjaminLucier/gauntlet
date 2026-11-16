// str Methods — reference solution.
// Each function is a short chain of `str` methods; results borrow from the
// input wherever they can.

use std::num::ParseIntError;

// ── Splitting ───────────────────────────────────────────────────────────────

pub fn words(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

pub fn csv_fields(line: &str) -> Vec<&str> {
    line.split(',').map(str::trim).collect()
}

pub fn key_value(s: &str) -> Option<(&str, &str)> {
    s.split_once('=').map(|(k, v)| (k.trim(), v.trim()))
}

pub fn extension(file_name: &str) -> Option<&str> {
    file_name
        .rsplit_once('.')
        .filter(|(stem, _)| !stem.is_empty())
        .map(|(_, ext)| ext)
}

pub fn first_fields(s: &str, n: usize) -> Vec<&str> {
    s.splitn(n, ':').collect()
}

pub fn non_blank_lines(text: &str) -> Vec<&str> {
    text.lines().filter(|l| !l.trim().is_empty()).collect()
}

pub fn tokens(s: &str) -> Vec<&str> {
    s.split([',', ';', ' ']).filter(|t| !t.is_empty()).collect()
}

// ── Trimming and stripping ──────────────────────────────────────────────────

pub fn unquote(s: &str) -> &str {
    s.strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
        .unwrap_or(s)
}

pub fn trim_leading_zeros(s: &str) -> &str {
    match s.trim_start_matches('0') {
        "" if !s.is_empty() => "0",
        rest => rest,
    }
}

pub fn trim_punctuation(s: &str) -> &str {
    s.trim_matches(|c: char| c.is_ascii_punctuation())
}

pub fn strip_comment(line: &str) -> &str {
    line.find('#').map_or(line, |i| &line[..i]).trim_end()
}

// ── Searching ───────────────────────────────────────────────────────────────

pub fn starts_with_digit(s: &str) -> bool {
    s.starts_with(|c: char| c.is_ascii_digit())
}

pub fn base_name(path: &str) -> &str {
    path.rfind('/').map_or(path, |i| &path[i + 1..])
}

pub fn count_occurrences(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}

pub fn vowel_offsets(s: &str) -> Vec<usize> {
    s.char_indices()
        .filter(|&(_, c)| "aeiouAEIOU".contains(c))
        .map(|(i, _)| i)
        .collect()
}

// ── Transforming ────────────────────────────────────────────────────────────

pub fn censor(s: &str, word: &str) -> String {
    s.replace(word, "***")
}

pub fn replace_first(s: &str, from: &str, to: &str) -> String {
    s.replacen(from, to, 1)
}

pub fn shout(s: &str) -> String {
    s.to_uppercase() + "!"
}

pub fn same_command(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}

pub fn underline(title: &str) -> String {
    "=".repeat(title.chars().count())
}

// ── Parsing ─────────────────────────────────────────────────────────────────

pub fn parse_numbers(s: &str) -> Result<Vec<i32>, ParseIntError> {
    s.split_whitespace().map(str::parse).collect()
}

pub fn parse_point(s: &str) -> Option<(i32, i32)> {
    let (x, y) = s.split_once(',')?;
    Some((x.trim().parse().ok()?, y.trim().parse().ok()?))
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
