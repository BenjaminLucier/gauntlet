// BTreeMap and Ranges — reference solution.
// Ordered maps answer "what's near x" with `range` in O(log n) plus the items
// visited; the ends are O(log n) through first/last/pop.

use std::collections::{BTreeMap, BTreeSet};
use std::ops::Bound;

// ── Ordered iteration ───────────────────────────────────────────────────────

pub fn sorted_unique(xs: &[i32]) -> Vec<i32> {
    xs.iter()
        .copied()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub fn values_by_key(map: BTreeMap<String, u32>) -> Vec<u32> {
    map.into_values().collect()
}

pub fn descending_keys(map: &BTreeMap<i32, &str>) -> Vec<i32> {
    map.keys().rev().copied().collect()
}

pub fn nth_smallest(set: &BTreeSet<i32>, n: usize) -> Option<i32> {
    set.iter().nth(n).copied()
}

// ── Ranges ──────────────────────────────────────────────────────────────────

pub fn values_between<'a>(map: &BTreeMap<u32, &'a str>, lo: u32, hi: u32) -> Vec<&'a str> {
    if lo > hi {
        return Vec::new(); // `range` panics when start > end
    }
    map.range(lo..=hi).map(|(_, &v)| v).collect()
}

pub fn range_sum(map: &BTreeMap<i32, i64>, lo: i32, hi: i32) -> i64 {
    if lo >= hi {
        return 0;
    }
    map.range(lo..hi).map(|(_, v)| v).sum()
}

pub fn raise_between(map: &mut BTreeMap<u32, i64>, lo: u32, hi: u32, by: i64) {
    if lo > hi {
        return; // `range_mut` panics when start > end
    }
    for (_, v) in map.range_mut(lo..=hi) {
        *v += by;
    }
}

pub fn count_below(set: &BTreeSet<i32>, x: i32) -> usize {
    set.range(..x).count()
}

pub fn events_from(log: &BTreeMap<u64, String>, t: u64, n: usize) -> Vec<&str> {
    log.range(t..).take(n).map(|(_, e)| e.as_str()).collect()
}

pub fn with_prefix<'a>(words: &'a BTreeSet<String>, prefix: &str) -> Vec<&'a str> {
    words
        .range::<str, _>((Bound::Included(prefix), Bound::Unbounded))
        .take_while(|w| w.starts_with(prefix))
        .map(String::as_str)
        .collect()
}

// ── Floor and ceiling ───────────────────────────────────────────────────────

pub fn floor(set: &BTreeSet<i32>, x: i32) -> Option<i32> {
    set.range(..=x).next_back().copied()
}

pub fn ceiling(set: &BTreeSet<i32>, x: i32) -> Option<i32> {
    set.range(x..).next().copied()
}

pub fn strictly_after(set: &BTreeSet<i32>, x: i32) -> Option<i32> {
    set.range((Bound::Excluded(x), Bound::Unbounded))
        .next()
        .copied()
}

// ── Ends ────────────────────────────────────────────────────────────────────

pub fn first_and_last(scores: &BTreeMap<String, u32>) -> Option<(&str, &str)> {
    let (first, _) = scores.first_key_value()?;
    let (last, _) = scores.last_key_value()?;
    Some((first, last))
}

pub fn pop_smallest(set: &mut BTreeSet<i32>, k: usize) -> Vec<i32> {
    std::iter::from_fn(|| set.pop_first()).take(k).collect()
}

pub fn sell_cheapest(stock: &mut BTreeMap<u32, u32>) -> Option<u32> {
    let mut cheapest = stock.first_entry()?;
    let price = *cheapest.key();
    *cheapest.get_mut() -= 1;
    if *cheapest.get() == 0 {
        cheapest.remove();
    }
    Some(price)
}

pub fn pop_latest(log: &mut BTreeMap<u64, String>) -> Option<String> {
    log.pop_last().map(|(_, e)| e)
}

// ── Splitting and editing ───────────────────────────────────────────────────

pub fn split_at_key(map: &mut BTreeMap<i32, char>, key: i32) -> BTreeMap<i32, char> {
    map.split_off(&key)
}

pub fn keep_even_keys(map: &mut BTreeMap<i32, char>) {
    map.retain(|k, _| k % 2 == 0);
}

pub fn absorb(set: &mut BTreeSet<i32>, other: &mut BTreeSet<i32>) {
    set.append(other);
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
