// Collecting Options and Results — reference solution.
// Each function is one iterator chain; collecting into Result/Option, Sum,
// try_fold and find_map all stop at the first failure. All O(n).

use std::collections::{BTreeSet, HashMap};
use std::num::ParseIntError;
use std::ops::ControlFlow;

// ── Collecting ──────────────────────────────────────────────────────────────

pub fn parse_all(words: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    words.iter().map(|w| w.parse()).collect()
}

pub fn all_present(xs: &[Option<u8>]) -> Option<Vec<u8>> {
    xs.iter().copied().collect()
}

pub fn lookup_all(map: &HashMap<&str, i32>, keys: &[&str]) -> Option<Vec<i32>> {
    keys.iter().map(|k| map.get(k).copied()).collect()
}

pub fn parse_unique(words: &[&str]) -> Result<BTreeSet<u32>, ParseIntError> {
    words.iter().map(|w| w.parse()).collect()
}

pub fn collect_string(chars: &[Option<char>]) -> Option<String> {
    chars.iter().copied().collect()
}

pub fn try_map<T, U, E>(items: &[T], f: impl FnMut(&T) -> Result<U, E>) -> Result<Vec<U>, E> {
    items.iter().map(f).collect()
}

pub fn parse_lines(text: &str) -> Result<Vec<u32>, ParseIntError> {
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::parse)
        .collect()
}

// ── Summing ─────────────────────────────────────────────────────────────────

pub fn sum_parsed(words: &[&str]) -> Result<i64, ParseIntError> {
    words.iter().map(|w| w.parse::<i64>()).sum()
}

pub fn product_present(xs: &[Option<u32>]) -> Option<u32> {
    xs.iter().copied().product()
}

// ── Folding with an exit ────────────────────────────────────────────────────

pub fn sum_until_negative(xs: &[i32]) -> i32 {
    let flow = xs.iter().try_fold(0, |acc, &x| {
        if x < 0 {
            ControlFlow::Break(acc)
        } else {
            ControlFlow::Continue(acc + x)
        }
    });
    match flow {
        ControlFlow::Break(sum) | ControlFlow::Continue(sum) => sum,
    }
}

pub fn running_balance(start: i64, txns: &[i64]) -> Result<i64, usize> {
    txns.iter().enumerate().try_fold(start, |balance, (i, &t)| {
        let next = balance + t;
        if next < 0 { Err(i) } else { Ok(next) }
    })
}

pub fn all_non_negative(xs: &[i32]) -> Result<(), i32> {
    xs.iter()
        .try_for_each(|&x| if x < 0 { Err(x) } else { Ok(()) })
}

// ── Keeping the good ones ───────────────────────────────────────────────────

pub fn valid_numbers(words: &[&str]) -> Vec<i32> {
    words.iter().filter_map(|w| w.parse().ok()).collect()
}

pub fn present_values(xs: &[Option<i32>]) -> Vec<i32> {
    xs.iter().flatten().copied().collect()
}

pub fn ok_values(rs: Vec<Result<i32, String>>) -> Vec<i32> {
    rs.into_iter().flatten().collect()
}

// ── Finding ─────────────────────────────────────────────────────────────────

pub fn first_number(words: &[&str]) -> Option<i32> {
    words.iter().find_map(|w| w.parse().ok())
}

pub fn first_error(rs: &[Result<i32, String>]) -> Option<&str> {
    rs.iter().find_map(|r| r.as_ref().err()).map(String::as_str)
}

// ── Splitting ───────────────────────────────────────────────────────────────

pub fn split_parsed<'a>(words: &[&'a str]) -> (Vec<i32>, Vec<&'a str>) {
    let (oks, errs): (Vec<_>, Vec<_>) = words
        .iter()
        .map(|&w| w.parse::<i32>().map_err(|_| w))
        .partition(Result::is_ok);
    (
        oks.into_iter().flatten().collect(),
        errs.into_iter().filter_map(Result::err).collect(),
    )
}

// ── Nesting ─────────────────────────────────────────────────────────────────

pub fn parse_optional_fields(fields: &[Option<&str>]) -> Result<Vec<Option<i32>>, ParseIntError> {
    fields.iter().map(|f| f.map(str::parse).transpose()).collect()
}

pub fn max_parsed(words: &[&str]) -> Result<Option<i32>, ParseIntError> {
    parse_all(words).map(|v| v.into_iter().max())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
