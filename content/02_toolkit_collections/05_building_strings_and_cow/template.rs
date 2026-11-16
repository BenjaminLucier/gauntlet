// Building Strings and Cow — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// (clippy::ptr_arg is allowed only because `todo!()` bodies hide that the `&mut String` is needed.)
#![allow(unused_imports, unused_variables, dead_code, clippy::ptr_arg)]

use std::borrow::Cow;
use std::fmt::Write;

// ── Pushing, inserting and removing ─────────────────────────────────────────

/// `"Hello, <name>!"`, allocating exactly once.
/// (`String::with_capacity`, `push_str`, `push`)
pub fn greeting(name: &str) -> String {
    todo!()
}

/// `s` between `open` and `close`: `bracket("x", '«', '»')` is `"«x»"`.
/// (`push`)
pub fn bracket(s: &str, open: char, close: char) -> String {
    todo!()
}

/// Wrap the string in double quotes, in place. (`insert`, `push`)
pub fn quote_in_place(s: &mut String) {
    todo!()
}

/// Put `prefix` at the front, in place. (`insert_str`)
pub fn prefix_in_place(s: &mut String, prefix: &str) {
    todo!()
}

/// Append every word, with nothing in between. (`extend`; `String` is
/// `Extend<&str>`, and `words.iter()` yields `&&str`)
pub fn append_words(s: &mut String, words: &[&str]) {
    todo!()
}

/// Delete every whitespace character (Unicode's definition), editing `s`
/// rather than building a new `String`. (`String::retain`)
pub fn remove_whitespace(s: &mut String) {
    todo!()
}

// ── Formatting ──────────────────────────────────────────────────────────────

/// Two lower-case hex digits per byte: `[0, 255]` → `"00ff"`. Write into one
/// `String` rather than allocating a `format!` per byte. (`write!`,
/// `fmt::Write`)
pub fn hex_string(bytes: &[u8]) -> String {
    todo!()
}

/// `"1. first\n2. second\n"` — every line ends with `\n`. (`writeln!`)
pub fn numbered_list(items: &[&str]) -> String {
    todo!()
}

/// `"(x, y)"` with two decimals each. (`format!` with a precision)
pub fn format_point(x: f64, y: f64) -> String {
    todo!()
}

/// Pad on the left with spaces to at least `width` characters.
/// (`format!` with a `width$` argument)
pub fn right_align(s: &str, width: usize) -> String {
    todo!()
}

// ── Joining and collecting ──────────────────────────────────────────────────

/// All the parts end to end. (`concat`)
pub fn concat_parts(parts: &[&str]) -> String {
    todo!()
}

/// The parts joined with `/`. (`join`)
pub fn join_path(parts: &[&str]) -> String {
    todo!()
}

/// A `String` made of these characters. (`String::from_iter`)
pub fn from_chars(chars: &[char]) -> String {
    todo!()
}

// ── Cow: borrow when you can ────────────────────────────────────────────────

/// Escape `<`, `>`, `&` and `"` as `&lt;`, `&gt;`, `&amp;`, `&quot;`.
/// Borrow the input if none of them occur. (`Cow::Borrowed`, `Cow::Owned`)
pub fn escape_html(s: &str) -> Cow<'_, str> {
    todo!()
}

/// The path with a trailing `/`, allocating only if it's missing. (`Cow`)
pub fn ensure_trailing_slash(path: &str) -> Cow<'_, str> {
    todo!()
}

/// Lower-cased (Unicode rules), borrowing if lower-casing would change
/// nothing. Upper-case letters aren't the only ones that change: title-case
/// `'ǅ'` lower-cases to `'ǆ'`. (`Cow`, `char::to_lowercase`)
pub fn lowercase_if_needed(s: &str) -> Cow<'_, str> {
    todo!()
}

/// Every `"\r\n"` turned into `"\n"`, borrowing if there are none. (`Cow`)
pub fn normalize_newlines(s: &str) -> Cow<'_, str> {
    todo!()
}

/// Escape every item and hand back owned strings. (`Cow::into_owned`)
pub fn escape_all(items: &[&str]) -> Vec<String> {
    todo!()
}

/// Upper-case the first character if it's lower-case. Only clone a borrowed
/// text if it actually changes. (`Cow::to_mut`)
pub fn capitalize(text: &mut Cow<'_, str>) {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
