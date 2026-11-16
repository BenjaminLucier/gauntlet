// Iterator Adaptors — reference solution.
// One chain per function, no loops. Everything is a single O(n) pass.

use std::iter;

// ── Transforming ────────────────────────────────────────────────────────────

pub fn squares(xs: &[i32]) -> Vec<i32> {
    xs.iter().map(|x| x * x).collect()
}

pub fn evens(xs: &[i32]) -> Vec<i32> {
    xs.iter().copied().filter(|x| x % 2 == 0).collect()
}

pub fn leading_digits(s: &str) -> Vec<u32> {
    s.chars().map_while(|c| c.to_digit(10)).collect()
}

// ── Positions ───────────────────────────────────────────────────────────────

pub fn indexed_labels(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .enumerate()
        .map(|(i, w)| format!("{i}: {w}"))
        .collect()
}

pub fn positions_of(s: &str, target: char) -> Vec<usize> {
    s.char_indices()
        .filter(|&(_, c)| c == target)
        .map(|(i, _)| i)
        .collect()
}

// ── Combining ───────────────────────────────────────────────────────────────

pub fn pairwise_sums(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter().zip(b).map(|(x, y)| x + y).collect()
}

pub fn concat(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter().chain(b).copied().collect()
}

pub fn framed(xs: &[i32], open: i32, close: i32) -> Vec<i32> {
    iter::once(open)
        .chain(xs.iter().copied())
        .chain(iter::once(close))
        .collect()
}

pub fn interleave(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter()
        .zip(b)
        .flat_map(|(&x, &y)| [x, y])
        .chain(a.iter().skip(b.len()).copied())
        .chain(b.iter().skip(a.len()).copied())
        .collect()
}

pub fn adjacent_differences(xs: &[i32]) -> Vec<i32> {
    xs.iter().zip(xs.iter().skip(1)).map(|(a, b)| b - a).collect()
}

// ── Slicing ─────────────────────────────────────────────────────────────────

pub fn page(xs: &[i32], n: usize, per_page: usize) -> Vec<i32> {
    xs.iter().skip(n * per_page).take(per_page).copied().collect()
}

pub fn strip_leading_zeros(digits: &[u8]) -> Vec<u8> {
    digits.iter().skip_while(|&&d| d == 0).copied().collect()
}

pub fn take_while_positive(xs: &[i32]) -> Vec<i32> {
    xs.iter().take_while(|&&x| x > 0).copied().collect()
}

pub fn every_nth(xs: &[i32], n: usize) -> Vec<i32> {
    xs.iter().step_by(n).copied().collect()
}

pub fn last_n(xs: &[i32], n: usize) -> Vec<i32> {
    xs.iter().rev().take(n).rev().copied().collect()
}

pub fn split_at_blank<'a>(lines: &[&'a str]) -> (Vec<&'a str>, Vec<&'a str>) {
    let mut it = lines.iter().copied();
    let head = it.by_ref().take_while(|l| !l.is_empty()).collect();
    (head, it.collect())
}

// ── Reversing ───────────────────────────────────────────────────────────────

pub fn reversed_words(s: &str) -> String {
    s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

// ── Nesting ─────────────────────────────────────────────────────────────────

pub fn expand(runs: &[(char, usize)]) -> String {
    runs.iter().flat_map(|&(c, n)| std::iter::repeat_n(c, n)).collect()
}

// ── State ───────────────────────────────────────────────────────────────────

pub fn running_total(xs: &[i32]) -> Vec<i32> {
    xs.iter()
        .scan(0, |sum, &x| {
            *sum += x;
            Some(*sum)
        })
        .collect()
}

pub fn run_lengths(s: &str) -> Vec<(char, usize)> {
    let mut chars = s.chars().peekable();
    std::iter::from_fn(|| {
        let c = chars.next()?;
        let more = std::iter::from_fn(|| chars.next_if_eq(&c)).count();
        Some((c, 1 + more))
    })
    .collect()
}

// ── Watching and repeating ──────────────────────────────────────────────────

pub fn doubled_with_log(xs: &[i32], mut log: impl FnMut(i32)) -> Vec<i32> {
    xs.iter().inspect(|&&x| log(x)).map(|x| x * 2).collect()
}

pub fn repeat_pattern(pattern: &[u8], len: usize) -> Vec<u8> {
    pattern.iter().cycle().take(len).copied().collect()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
