// Consumers and Folds — reference solution.
// One chain per function, ending in a consumer. Everything is a single O(n)
// pass, except `distinct_sorted` (O(n log n), it builds a BTreeSet).

use std::collections::{BTreeSet, HashMap};

// ── Arithmetic ──────────────────────────────────────────────────────────────

pub fn total(xs: &[i32]) -> i64 {
    xs.iter().copied().map(i64::from).sum()
}

pub fn factorial(n: u32) -> u64 {
    (1..=u64::from(n)).product()
}

pub fn mean(xs: &[f64]) -> Option<f64> {
    (!xs.is_empty()).then(|| xs.iter().sum::<f64>() / xs.len() as f64)
}

// ── Counting and testing ────────────────────────────────────────────────────

pub fn count_vowels(s: &str) -> usize {
    s.chars()
        .filter(|c| "aeiouAEIOU".contains(*c))
        .count()
}

pub fn has_negative(xs: &[i32]) -> bool {
    xs.iter().any(|&x| x < 0)
}

pub fn all_lowercase(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_lowercase())
}

pub fn same_words(a: &str, b: &str) -> bool {
    a.split_whitespace().eq(b.split_whitespace())
}

// ── Extremes ────────────────────────────────────────────────────────────────

pub fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|w| w.len())
}

pub fn longest_word(s: &str) -> Option<&str> {
    s.split_whitespace().max_by_key(|w| w.len())
}

pub fn min_max(xs: &[i32]) -> Option<(i32, i32)> {
    let (&first, rest) = xs.split_first()?;
    Some(
        rest.iter()
            .fold((first, first), |(lo, hi), &x| (lo.min(x), hi.max(x))),
    )
}

// ── Folding ─────────────────────────────────────────────────────────────────

pub fn to_number(digits: &[u8]) -> u64 {
    digits
        .iter()
        .fold(0, |acc, &d| acc * 10 + u64::from(d))
}

pub fn hyphenate(words: &[&str]) -> Option<String> {
    words
        .iter()
        .map(|w| w.to_string())
        .reduce(|acc, w| acc + "-" + &w)
}

pub fn nest(names: &[&str]) -> String {
    names.split_last().map_or_else(String::new, |(last, rest)| {
        rest.iter()
            .rfold(last.to_string(), |inner, name| format!("{name}({inner})"))
    })
}

// ── Searching ───────────────────────────────────────────────────────────────

pub fn index_of(xs: &[i32], target: i32) -> Option<usize> {
    xs.iter().position(|&x| x == target)
}

pub fn last_index_of(xs: &[i32], target: i32) -> Option<usize> {
    xs.iter().rposition(|&x| x == target)
}

pub fn first_long_word<'a>(words: &[&'a str], min_len: usize) -> Option<&'a str> {
    words.iter().find(|w| w.len() >= min_len).copied()
}

pub fn last_before_zero(xs: &[i32]) -> Option<i32> {
    xs.iter().take_while(|&&x| x != 0).last().copied()
}

pub fn nth_word(s: &str, n: usize) -> Option<&str> {
    s.split_whitespace().nth(n)
}

// ── Splitting ───────────────────────────────────────────────────────────────

pub fn split_even_odd(xs: &[i32]) -> (Vec<i32>, Vec<i32>) {
    xs.iter().partition(|&&x| x % 2 == 0)
}

pub fn split_pairs(pairs: &[(char, u32)]) -> (String, Vec<u32>) {
    pairs.iter().copied().unzip()
}

// ── Collecting ──────────────────────────────────────────────────────────────

pub fn word_lengths<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    words.iter().map(|&w| (w, w.chars().count())).collect()
}

pub fn initials(names: &[&str]) -> String {
    names.iter().filter_map(|n| n.chars().next()).collect()
}

pub fn distinct_sorted(xs: &[i32]) -> BTreeSet<i32> {
    xs.iter().copied().collect()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
