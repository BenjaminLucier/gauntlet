// VecDeque and BinaryHeap — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no sort/sort_unstable (except via `make_contiguous` in `sort_deque`);
// no Vec::remove(0) or Vec::insert(0, _) — that's what a deque is for.
#![allow(unused_imports, unused_variables, dead_code)]

use std::cmp::Reverse;
use std::collections::binary_heap::PeekMut;
use std::collections::{BinaryHeap, VecDeque};

// ── VecDeque: both ends ─────────────────────────────────────────────────────

/// Build a deque by pushing each of `front` onto the front (in order — so the
/// last one ends up first), then each of `back` onto the back.
/// (`push_front`, `push_back`)
pub fn push_both(front: &[i32], back: &[i32]) -> VecDeque<i32> {
    todo!()
}

/// Does `s` read the same forwards and backwards (by `char`)? Pop from both
/// ends until they meet. (`pop_front`, `pop_back`)
pub fn is_palindrome(s: &str) -> bool {
    todo!()
}

/// Rotate by `k`: positive moves elements toward the back (the last `k` come to
/// the front), negative toward the front. `k` may exceed the length.
/// (`rotate_right`, `rotate_left`)
pub fn rotate(d: &mut VecDeque<i32>, k: isize) {
    todo!()
}

/// The order in which people `1..=n` standing in a circle are removed when
/// every `k`-th one is counted out (`k >= 1`), starting the count at person 1.
/// (`rotate_left`, `pop_front`)
pub fn elimination_order(n: usize, k: usize) -> Vec<usize> {
    todo!()
}

// ── VecDeque: sliding buffers ───────────────────────────────────────────────

/// The last `k` items of the stream, oldest first, keeping at most `k` in
/// memory at a time. (`push_back`, `pop_front`)
pub fn last_k(stream: &[i32], k: usize) -> Vec<i32> {
    todo!()
}

/// For each ping time (non-decreasing), how many pings so far fall in
/// `[t - window, t]`, the current one included. (`front`, `pop_front`)
pub fn recent_counts(pings: &[u32], window: u32) -> Vec<usize> {
    todo!()
}

// ── VecDeque: as slices ─────────────────────────────────────────────────────

/// Sort the deque in place. A ring buffer may be split in two; make it one
/// slice first. (`make_contiguous`)
pub fn sort_deque(d: &mut VecDeque<i32>) {
    todo!()
}

/// The sum of every element, reading the deque as its (up to) two slices.
/// (`as_slices`)
pub fn total(d: &VecDeque<i64>) -> i64 {
    todo!()
}

/// The elements at positions `start..end` as a string. Panics if
/// `start > end` or `end > d.len()`, as `range` itself does. (`range`)
pub fn middle(d: &VecDeque<char>, start: usize, end: usize) -> String {
    todo!()
}

// ── BinaryHeap ──────────────────────────────────────────────────────────────

/// The largest element, without removing it. (`peek`)
pub fn peek_max(heap: &BinaryHeap<i32>) -> Option<i32> {
    todo!()
}

/// The `k` largest values, largest first. (`BinaryHeap::from(Vec)`, `pop`)
pub fn largest_k(xs: &[i32], k: usize) -> Vec<i32> {
    todo!()
}

/// The `k` smallest values, smallest first, using a min-heap.
/// (`BinaryHeap<Reverse<T>>`)
pub fn smallest_k(xs: &[i32], k: usize) -> Vec<i32> {
    todo!()
}

/// All values, ascending. (`into_sorted_vec`)
pub fn heap_sorted(xs: Vec<i32>) -> Vec<i32> {
    todo!()
}

/// Move every element of `other` into `heap`, leaving `other` empty.
/// (`BinaryHeap::append`)
pub fn merge_into(heap: &mut BinaryHeap<i32>, other: &mut BinaryHeap<i32>) {
    todo!()
}

/// Empty the heap and return the sum of what it held. The heap keeps its
/// allocation, ready for reuse. (`BinaryHeap::drain`)
pub fn drain_total(heap: &mut BinaryHeap<u64>) -> u64 {
    todo!()
}

/// Halve the largest element in place (rounding down); the heap reorders
/// itself when the guard drops. Does nothing on an empty heap. (`peek_mut`)
pub fn halve_largest(heap: &mut BinaryHeap<u32>) {
    todo!()
}

/// Remove every odd value. (`retain`)
pub fn drop_odds(heap: &mut BinaryHeap<i32>) {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
