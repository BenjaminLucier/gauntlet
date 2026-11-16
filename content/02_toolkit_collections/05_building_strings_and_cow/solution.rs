// Building Strings and Cow — reference solution.
// Builders allocate once where the size is known; the `Cow` functions check
// first and only allocate when the input actually has to change.

use std::borrow::Cow;
use std::fmt::Write;

// ── Pushing, inserting and removing ─────────────────────────────────────────

pub fn greeting(name: &str) -> String {
    let mut s = String::with_capacity("Hello, !".len() + name.len());
    s.push_str("Hello, ");
    s.push_str(name);
    s.push('!');
    s
}

pub fn bracket(s: &str, open: char, close: char) -> String {
    let mut out = String::with_capacity(s.len() + open.len_utf8() + close.len_utf8());
    out.push(open);
    out.push_str(s);
    out.push(close);
    out
}

pub fn quote_in_place(s: &mut String) {
    s.insert(0, '"');
    s.push('"');
}

pub fn prefix_in_place(s: &mut String, prefix: &str) {
    s.insert_str(0, prefix);
}

pub fn append_words(s: &mut String, words: &[&str]) {
    s.extend(words.iter().copied());
}

pub fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

// ── Formatting ──────────────────────────────────────────────────────────────

pub fn hex_string(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(out, "{b:02x}").expect("writing to a String never fails");
    }
    out
}

pub fn numbered_list(items: &[&str]) -> String {
    let mut out = String::new();
    for (i, item) in items.iter().enumerate() {
        writeln!(out, "{}. {item}", i + 1).expect("writing to a String never fails");
    }
    out
}

pub fn format_point(x: f64, y: f64) -> String {
    format!("({x:.2}, {y:.2})")
}

pub fn right_align(s: &str, width: usize) -> String {
    format!("{s:>width$}")
}

// ── Joining and collecting ──────────────────────────────────────────────────

pub fn concat_parts(parts: &[&str]) -> String {
    parts.concat()
}

pub fn join_path(parts: &[&str]) -> String {
    parts.join("/")
}

pub fn from_chars(chars: &[char]) -> String {
    String::from_iter(chars)
}

// ── Cow: borrow when you can ────────────────────────────────────────────────

pub fn escape_html(s: &str) -> Cow<'_, str> {
    if !s.contains(['<', '>', '&', '"']) {
        return Cow::Borrowed(s);
    }
    let mut out = String::with_capacity(s.len() + 16);
    for c in s.chars() {
        match c {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    Cow::Owned(out)
}

pub fn ensure_trailing_slash(path: &str) -> Cow<'_, str> {
    if path.ends_with('/') {
        Cow::Borrowed(path)
    } else {
        Cow::Owned(format!("{path}/"))
    }
}

pub fn lowercase_if_needed(s: &str) -> Cow<'_, str> {
    if s.chars().any(|c| c.to_lowercase().ne(std::iter::once(c))) {
        Cow::Owned(s.to_lowercase())
    } else {
        Cow::Borrowed(s)
    }
}

pub fn normalize_newlines(s: &str) -> Cow<'_, str> {
    if s.contains("\r\n") {
        Cow::Owned(s.replace("\r\n", "\n"))
    } else {
        Cow::Borrowed(s)
    }
}

pub fn escape_all(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| escape_html(s).into_owned()).collect()
}

pub fn capitalize(text: &mut Cow<'_, str>) {
    let Some(first) = text.chars().next().filter(|c| c.is_lowercase()) else {
        return;
    };
    let upper: String = first.to_uppercase().collect();
    text.to_mut().replace_range(..first.len_utf8(), &upper);
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
