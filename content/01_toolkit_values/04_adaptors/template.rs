// Iterator Adaptors — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no for, while or loop. Iterator adaptors only.
#![allow(unused_imports, unused_variables, dead_code)]

// ── Transforming ────────────────────────────────────────────────────────────

/// Each value squared. (`map`)
pub fn squares(xs: &[i32]) -> Vec<i32> {
    todo!()
}

/// The even values, in order. (`filter`)
pub fn evens(xs: &[i32]) -> Vec<i32> {
    todo!()
}

/// The decimal digits at the start of `s`, stopping at the first non-digit.
/// (`map_while`, `char::to_digit`)
pub fn leading_digits(s: &str) -> Vec<u32> {
    todo!()
}

// ── Positions ───────────────────────────────────────────────────────────────

/// `"<index>: <word>"` for each word, counting from 0. (`enumerate`)
pub fn indexed_labels(words: &[&str]) -> Vec<String> {
    todo!()
}

/// The byte offsets at which `target` occurs in `s`. (`char_indices`, `filter`)
pub fn positions_of(s: &str, target: char) -> Vec<usize> {
    todo!()
}

// ── Combining ───────────────────────────────────────────────────────────────

/// `a[i] + b[i]`, for as long as both slices last. (`zip`)
pub fn pairwise_sums(a: &[i32], b: &[i32]) -> Vec<i32> {
    todo!()
}

/// `a` followed by `b`. (`chain`)
pub fn concat(a: &[i32], b: &[i32]) -> Vec<i32> {
    todo!()
}

/// `open`, then every value of `xs`, then `close`. (`std::iter::once`, `chain`)
pub fn framed(xs: &[i32], open: i32, close: i32) -> Vec<i32> {
    todo!()
}

/// Alternate `a[0], b[0], a[1], b[1], …`; once the shorter one runs out,
/// the rest of the longer one follows. (`zip`, `flat_map`, `chain`, `skip`)
pub fn interleave(a: &[i32], b: &[i32]) -> Vec<i32> {
    todo!()
}

/// `xs[i + 1] - xs[i]` for each neighbouring pair — without `windows`.
/// (`zip` with `skip(1)`)
pub fn adjacent_differences(xs: &[i32]) -> Vec<i32> {
    todo!()
}

// ── Slicing ─────────────────────────────────────────────────────────────────

/// Page `n` (0-based) of `xs`, `per_page` items per page; short or empty past
/// the end. (`skip`, `take`)
pub fn page(xs: &[i32], n: usize, per_page: usize) -> Vec<i32> {
    todo!()
}

/// Drop the zeros at the front, keep everything from the first non-zero on.
/// (`skip_while`)
pub fn strip_leading_zeros(digits: &[u8]) -> Vec<u8> {
    todo!()
}

/// The values up to (not including) the first one that isn't positive.
/// (`take_while`)
pub fn take_while_positive(xs: &[i32]) -> Vec<i32> {
    todo!()
}

/// `xs[0], xs[n], xs[2n], …`. `n` is at least 1. (`step_by`)
pub fn every_nth(xs: &[i32], n: usize) -> Vec<i32> {
    todo!()
}

/// The last `n` values (all of them if there are fewer), in their original
/// order. (`rev`, `take`, `rev`)
pub fn last_n(xs: &[i32], n: usize) -> Vec<i32> {
    todo!()
}

/// Split at the first empty line: the lines before it and the lines after it
/// (the empty line itself belongs to neither). With no empty line, everything
/// is in the first part. Walk the lines with a single iterator. (`by_ref`,
/// `take_while`)
pub fn split_at_blank<'a>(lines: &[&'a str]) -> (Vec<&'a str>, Vec<&'a str>) {
    todo!()
}

// ── Reversing ───────────────────────────────────────────────────────────────

/// The whitespace-separated words in reverse order, joined by single spaces.
/// (`split_whitespace`, `rev`)
pub fn reversed_words(s: &str) -> String {
    todo!()
}

// ── Nesting ─────────────────────────────────────────────────────────────────

/// Expand `(char, count)` runs into a string: `[('a', 2), ('b', 1)]` → `"aab"`.
/// (`flat_map`, `std::iter::repeat_n`)
pub fn expand(runs: &[(char, usize)]) -> String {
    todo!()
}

// ── State ───────────────────────────────────────────────────────────────────

/// The prefix sums: `[1, 2, 3]` → `[1, 3, 6]`. (`scan`)
pub fn running_total(xs: &[i32]) -> Vec<i32> {
    todo!()
}

/// Run-length encode: `"aaabcc"` → `[('a', 3), ('b', 1), ('c', 2)]`.
/// (`peekable`, `next_if_eq`, `std::iter::from_fn`)
pub fn run_lengths(s: &str) -> Vec<(char, usize)> {
    todo!()
}

// ── Watching and repeating ──────────────────────────────────────────────────

/// Each value doubled; call `log` with each *original* value as it passes.
/// Calling an `FnMut` needs a mutable binding: write `mut log`. (`inspect`)
pub fn doubled_with_log(xs: &[i32], log: impl FnMut(i32)) -> Vec<i32> {
    todo!()
}

/// `pattern` repeated to exactly `len` bytes (empty if `pattern` is). (`cycle`,
/// `take`)
pub fn repeat_pattern(pattern: &[u8], len: usize) -> Vec<u8> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
