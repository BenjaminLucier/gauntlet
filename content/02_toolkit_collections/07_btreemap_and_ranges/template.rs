// BTreeMap and Ranges — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no sorting, and no linear scans where a `range` will do.
#![allow(unused_imports, unused_variables, dead_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::ops::Bound;

// ── Ordered iteration ───────────────────────────────────────────────────────

/// The distinct values, ascending. (`collect` into a `BTreeSet`)
pub fn sorted_unique(xs: &[i32]) -> Vec<i32> {
    todo!()
}

/// The values in key order, consuming the map — no keys cloned or dropped by
/// hand. (`into_values`)
pub fn values_by_key(map: BTreeMap<String, u32>) -> Vec<u32> {
    todo!()
}

/// The keys from largest to smallest. (`keys`, `rev`)
pub fn descending_keys(map: &BTreeMap<i32, &str>) -> Vec<i32> {
    todo!()
}

/// The `n`-th smallest element (0-based). (`iter`, `nth`)
pub fn nth_smallest(set: &BTreeSet<i32>, n: usize) -> Option<i32> {
    todo!()
}

// ── Ranges ──────────────────────────────────────────────────────────────────

/// The values whose keys are in `lo..=hi`, in key order. Empty if `lo > hi`.
/// (`range` with `..=` — and mind what `range` does when start > end)
pub fn values_between<'a>(map: &BTreeMap<u32, &'a str>, lo: u32, hi: u32) -> Vec<&'a str> {
    todo!()
}

/// The sum of the values whose keys are in the half-open `lo..hi`. 0 if the
/// range is empty or backwards. (`range`, `values`-style mapping, `sum`)
pub fn range_sum(map: &BTreeMap<i32, i64>, lo: i32, hi: i32) -> i64 {
    todo!()
}

/// Add `by` to every value whose key is in `lo..=hi`; do nothing if `lo > hi`.
/// (`range_mut`)
pub fn raise_between(map: &mut BTreeMap<u32, i64>, lo: u32, hi: u32, by: i64) {
    todo!()
}

/// How many elements are strictly below `x`. (`range(..x)`, `count`)
pub fn count_below(set: &BTreeSet<i32>, x: i32) -> usize {
    todo!()
}

/// The first `n` events at or after time `t`. (`range(t..)`, `take`)
pub fn events_from(log: &BTreeMap<u64, String>, t: u64, n: usize) -> Vec<&str> {
    todo!()
}

/// Every word that starts with `prefix`, in order. (`range` over `str` with
/// `Bound`, `take_while`)
pub fn with_prefix<'a>(words: &'a BTreeSet<String>, prefix: &str) -> Vec<&'a str> {
    todo!()
}

// ── Floor and ceiling ───────────────────────────────────────────────────────

/// The largest element `<= x`. (`range(..=x)`, `next_back`)
pub fn floor(set: &BTreeSet<i32>, x: i32) -> Option<i32> {
    todo!()
}

/// The smallest element `>= x`. (`range(x..)`, `next`)
pub fn ceiling(set: &BTreeSet<i32>, x: i32) -> Option<i32> {
    todo!()
}

/// The smallest element strictly greater than `x`.
/// (`range` with `Bound::Excluded`)
pub fn strictly_after(set: &BTreeSet<i32>, x: i32) -> Option<i32> {
    todo!()
}

// ── Ends ────────────────────────────────────────────────────────────────────

/// The alphabetically first and last names. (`first_key_value`,
/// `last_key_value`)
pub fn first_and_last(scores: &BTreeMap<String, u32>) -> Option<(&str, &str)> {
    todo!()
}

/// Remove and return the `k` smallest elements, ascending (fewer if the set
/// runs out). (`pop_first`)
pub fn pop_smallest(set: &mut BTreeSet<i32>, k: usize) -> Vec<i32> {
    todo!()
}

/// `stock` maps a price to how many items are on offer at it (always at least
/// 1). Sell one item at the lowest price: decrement that count, removing the
/// price once it reaches 0, and return the price. `None` if nothing is left.
/// (`first_entry`, `OccupiedEntry::remove`)
pub fn sell_cheapest(stock: &mut BTreeMap<u32, u32>) -> Option<u32> {
    todo!()
}

/// Remove the latest entry and return its value. (`pop_last`)
pub fn pop_latest(log: &mut BTreeMap<u64, String>) -> Option<String> {
    todo!()
}

// ── Splitting and editing ───────────────────────────────────────────────────

/// Move every entry with key `>= key` out of `map` and return them.
/// (`split_off`)
pub fn split_at_key(map: &mut BTreeMap<i32, char>, key: i32) -> BTreeMap<i32, char> {
    todo!()
}

/// Keep only the entries with an even key. (`retain`)
pub fn keep_even_keys(map: &mut BTreeMap<i32, char>) {
    todo!()
}

/// Move everything from `other` into `set`, leaving `other` empty. (`append`)
pub fn absorb(set: &mut BTreeSet<i32>, other: &mut BTreeSet<i32>) {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
