// Editing in Place — reference solution.
// Every function edits the buffer it's given: `&mut Vec<T>` when the length
// changes, `&mut [T]` when it doesn't. No new Vec is built and assigned back.

use std::ops::Range;

// ── Removing ────────────────────────────────────────────────────────────────

pub fn drop_negatives(v: &mut Vec<i32>) {
    v.retain(|&x| x >= 0);
}

pub fn tick_down(timers: &mut Vec<u32>) {
    timers.retain_mut(|t| {
        *t = t.saturating_sub(1);
        *t > 0
    });
}

pub fn collapse_runs(v: &mut Vec<i32>) {
    v.dedup();
}

pub fn one_per_decade(years: &mut Vec<u32>) {
    years.dedup_by_key(|y| *y / 10);
}

pub fn cut_range(v: &mut Vec<i32>, range: Range<usize>) -> Vec<i32> {
    v.drain(range).collect()
}

pub fn extract_evens(v: &mut Vec<i32>) -> Vec<i32> {
    v.extract_if(.., |x| *x % 2 == 0).collect()
}

pub fn split_tail(v: &mut Vec<i32>, at: usize) -> Vec<i32> {
    v.split_off(at)
}

pub fn keep_first(v: &mut Vec<i32>, n: usize) {
    v.truncate(n);
}

pub fn remove_unordered(v: &mut Vec<i32>, index: usize) -> i32 {
    v.swap_remove(index)
}

// ── Growing ─────────────────────────────────────────────────────────────────

pub fn fit_to(v: &mut Vec<i32>, len: usize, fill: i32) {
    v.resize(len, fill);
}

pub fn append_all(v: &mut Vec<i32>, xs: &[i32]) {
    v.extend_from_slice(xs);
}

pub fn double_up(v: &mut Vec<i32>) {
    v.extend_from_within(..);
}

pub fn insert_sorted(v: &mut Vec<(i32, char)>, item: (i32, char)) {
    let at = v.partition_point(|&(key, _)| key <= item.0);
    v.insert(at, item);
}

// ── Rearranging ─────────────────────────────────────────────────────────────

pub fn rotate(xs: &mut [i32], k: isize) {
    if xs.is_empty() {
        return;
    }
    let steps = k.unsigned_abs() % xs.len();
    if k >= 0 {
        xs.rotate_right(steps);
    } else {
        xs.rotate_left(steps);
    }
}

pub fn reverse_each_chunk(xs: &mut [i32], size: usize) {
    xs.chunks_mut(size).for_each(<[i32]>::reverse);
}

pub fn swap_ends(xs: &mut [i32]) {
    if let Some(last) = xs.len().checked_sub(1) {
        xs.swap(0, last);
    }
}

pub fn reset(xs: &mut [i32], value: i32) {
    xs.fill(value);
}

pub fn number_from(xs: &mut [u32], start: u32) {
    let mut next = start;
    xs.fill_with(|| {
        let current = next;
        next = next.wrapping_add(1);
        current
    });
}

pub fn clamp_all(xs: &mut [i32], lo: i32, hi: i32) {
    xs.iter_mut().for_each(|x| *x = (*x).clamp(lo, hi));
}

// ── Sorting ─────────────────────────────────────────────────────────────────

pub fn sort_by_distance(xs: &mut [i32], target: i32) {
    xs.sort_unstable_by_key(|&x| (x.abs_diff(target), x));
}

pub fn sort_people(people: &mut [(String, u32)]) {
    people.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
}

pub fn sort_case_insensitive(words: &mut [String]) {
    words.sort_by_cached_key(|w| w.to_lowercase());
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
