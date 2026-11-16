// Editing in Place — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: edit the buffer you're given; never build a new Vec and assign it back (`*v = ...collect()`).
// (clippy::ptr_arg is allowed only because `todo!()` bodies hide that the `&mut Vec` is needed.)
#![allow(unused_imports, unused_variables, dead_code, clippy::ptr_arg)]

use std::ops::Range;

// ── Removing ────────────────────────────────────────────────────────────────

/// Remove every negative number, keeping the order of the rest. (`retain`)
pub fn drop_negatives(v: &mut Vec<i32>) {
    todo!()
}

/// Decrement every timer by one (not below 0), then remove the ones that hit
/// 0 — in a single pass. (`retain_mut`)
pub fn tick_down(timers: &mut Vec<u32>) {
    todo!()
}

/// Collapse runs of equal neighbours into one: `[1, 1, 2, 1]` → `[1, 2, 1]`.
/// (`dedup`)
pub fn collapse_runs(v: &mut Vec<i32>) {
    todo!()
}

/// Of consecutive years in the same decade, keep only the first.
/// (`dedup_by_key`)
pub fn one_per_decade(years: &mut Vec<u32>) {
    todo!()
}

/// Remove the elements in `range` and return them. (`drain`)
pub fn cut_range(v: &mut Vec<i32>, range: Range<usize>) -> Vec<i32> {
    todo!()
}

/// Remove the even numbers and return them; both sides keep their order.
/// (`extract_if`)
pub fn extract_evens(v: &mut Vec<i32>) -> Vec<i32> {
    todo!()
}

/// Cut the vector at `at`: it keeps `[..at]`, you return `[at..]`.
/// (`split_off`)
pub fn split_tail(v: &mut Vec<i32>, at: usize) -> Vec<i32> {
    todo!()
}

/// Keep at most the first `n` elements. (`truncate`)
pub fn keep_first(v: &mut Vec<i32>, n: usize) {
    todo!()
}

/// Remove and return the element at `index` in O(1); the last element moves
/// into the hole. (`swap_remove`)
pub fn remove_unordered(v: &mut Vec<i32>, index: usize) -> i32 {
    todo!()
}

// ── Growing ─────────────────────────────────────────────────────────────────

/// Make the vector exactly `len` long: pad with `fill`, or cut. (`resize`)
pub fn fit_to(v: &mut Vec<i32>, len: usize, fill: i32) {
    todo!()
}

/// Append a copy of `xs`. (`extend_from_slice`)
pub fn append_all(v: &mut Vec<i32>, xs: &[i32]) {
    todo!()
}

/// Append a copy of the vector to itself: `[1, 2]` → `[1, 2, 1, 2]`.
/// (`extend_from_within`)
pub fn double_up(v: &mut Vec<i32>) {
    todo!()
}

/// Insert `item` into a vector sorted by key (the `i32`), keeping it sorted.
/// Only the key is compared: an item whose key is already present goes
/// after every existing item with that key, whatever its `char`.
/// (`partition_point`, `insert`)
pub fn insert_sorted(v: &mut Vec<(i32, char)>, item: (i32, char)) {
    todo!()
}

// ── Rearranging ─────────────────────────────────────────────────────────────

/// Rotate right by `k` (left if `k` is negative); `k` may exceed the length.
/// (`rotate_right`, `rotate_left`)
pub fn rotate(xs: &mut [i32], k: isize) {
    todo!()
}

/// Reverse each group of `size` elements (the last group may be shorter).
/// (`chunks_mut`, `reverse`)
pub fn reverse_each_chunk(xs: &mut [i32], size: usize) {
    todo!()
}

/// Swap the first and last elements; do nothing on an empty slice. (`swap`)
pub fn swap_ends(xs: &mut [i32]) {
    todo!()
}

/// Set every element to `value`. (`fill`)
pub fn reset(xs: &mut [i32], value: i32) {
    todo!()
}

/// Overwrite with `start, start + 1, start + 2, …`, wrapping round to 0 after
/// `u32::MAX`. (`fill_with`, `wrapping_add`)
pub fn number_from(xs: &mut [u32], start: u32) {
    todo!()
}

/// Clamp every element into `lo..=hi`. (`iter_mut`)
pub fn clamp_all(xs: &mut [i32], lo: i32, hi: i32) {
    todo!()
}

// ── Sorting ─────────────────────────────────────────────────────────────────

/// Sort by distance from `target`, closest first; equal distances put the
/// smaller number first. (`sort_unstable_by_key`)
pub fn sort_by_distance(xs: &mut [i32], target: i32) {
    todo!()
}

/// Sort by age, oldest first; equal ages by name, A to Z.
/// (`sort_by`, `Ordering::then_with`)
pub fn sort_people(people: &mut [(String, u32)]) {
    todo!()
}

/// Sort ignoring case; words equal ignoring case keep their order. The key
/// allocates, so compute it once per element. (`sort_by_cached_key`)
pub fn sort_case_insensitive(words: &mut [String]) {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
