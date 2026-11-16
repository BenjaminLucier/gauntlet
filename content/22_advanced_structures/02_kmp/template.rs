// Knuth–Morris–Pratt — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no str::find/contains/matches/match_indices/split, no windows/starts_with-based search;
// the &str functions must use the prefix function too, not slice == per window.
#![allow(unused_imports, unused_variables, dead_code)]

/// `pi[i]`: length of the longest proper prefix of `pattern[..=i]` that is
/// also a suffix of it.
pub fn prefix_function<T: Eq>(pattern: &[T]) -> Vec<usize> {
    todo!()
}

/// Start index of every (possibly overlapping) match, in increasing order.
/// The empty pattern matches at every index `0..=text.len()`.
pub fn find_all_in<T: Eq>(text: &[T], pattern: &[T]) -> Vec<usize> {
    todo!()
}

/// Index of the first match, without scanning the text past it.
/// The empty pattern matches at `0`.
pub fn find_first_in<T: Eq>(text: &[T], pattern: &[T]) -> Option<usize> {
    todo!()
}

/// Byte offset of every match of `pattern` in `text`.
pub fn find_all(text: &str, pattern: &str) -> Vec<usize> {
    todo!()
}

/// Byte offset of the first match; `Some(0)` for the empty pattern.
pub fn find_first(text: &str, pattern: &str) -> Option<usize> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
