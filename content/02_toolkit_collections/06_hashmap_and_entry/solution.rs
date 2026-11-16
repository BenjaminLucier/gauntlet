// HashMap and Entry — reference solution.
// Every insert-or-update goes through `entry`, so each key is hashed once;
// anything returned in order is sorted, since hash iteration order is unspecified.

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

// ── Counting with entry ─────────────────────────────────────────────────────

pub fn word_counts(text: &str) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

pub fn group_by_len<'a>(words: &[&'a str]) -> HashMap<usize, Vec<&'a str>> {
    let mut groups: HashMap<usize, Vec<&str>> = HashMap::new();
    for &word in words {
        groups.entry(word.len()).or_default().push(word);
    }
    groups
}

pub fn first_index<T: Hash + Eq + Copy>(items: &[T]) -> HashMap<T, usize> {
    let mut first = HashMap::new();
    for (i, &item) in items.iter().enumerate() {
        first.entry(item).or_insert(i);
    }
    first
}

pub fn bump(map: &mut HashMap<String, u32>, key: &str) {
    map.entry(key.to_string())
        .and_modify(|n| *n += 1)
        .or_insert(1);
}

pub fn cached_len(
    cache: &mut HashMap<String, usize>,
    key: &str,
    compute: impl FnOnce(&str) -> usize,
) -> usize {
    *cache
        .entry(key.to_string())
        .or_insert_with(|| compute(key))
}

pub fn merge_counts(total: &mut HashMap<String, u32>, other: HashMap<String, u32>) {
    for (key, n) in other {
        *total.entry(key).or_default() += n;
    }
}

pub fn take_one(stock: &mut HashMap<char, u32>, c: char) -> bool {
    match stock.entry(c) {
        Entry::Occupied(mut slot) => {
            *slot.get_mut() -= 1;
            if *slot.get() == 0 {
                slot.remove();
            }
            true
        }
        Entry::Vacant(_) => false,
    }
}

// ── Looking up ──────────────────────────────────────────────────────────────

pub fn age_of(ages: &HashMap<String, u32>, name: &str) -> Option<u32> {
    ages.get(name).copied()
}

pub fn add_if_present(map: &mut HashMap<String, i32>, key: &str, by: i32) -> bool {
    map.get_mut(key).map(|v| *v += by).is_some()
}

pub fn has_all(map: &HashMap<&str, i32>, keys: &[&str]) -> bool {
    keys.iter().all(|k| map.contains_key(k))
}

// ── Removing ────────────────────────────────────────────────────────────────

pub fn take_value(map: &mut HashMap<String, Vec<i32>>, key: &str) -> Option<Vec<i32>> {
    map.remove(key)
}

pub fn take_entry(map: &mut HashMap<String, u32>, key: &str) -> Option<(String, u32)> {
    map.remove_entry(key)
}

pub fn drop_below(map: &mut HashMap<String, u32>, min: u32) {
    map.retain(|_, v| *v >= min);
}

// ── Building ────────────────────────────────────────────────────────────────

pub fn roman_values() -> HashMap<char, u32> {
    HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ])
}

pub fn invert<K: Clone, V: Clone + Hash + Eq>(map: &HashMap<K, V>) -> HashMap<V, K> {
    map.iter().map(|(k, v)| (v.clone(), k.clone())).collect()
}

// ── Deterministic output ────────────────────────────────────────────────────

pub fn sorted_keys<K: Ord + Clone, V>(map: &HashMap<K, V>) -> Vec<K> {
    let mut keys: Vec<K> = map.keys().cloned().collect();
    keys.sort_unstable();
    keys
}

pub fn ranked<'a>(counts: &HashMap<&'a str, usize>) -> Vec<(&'a str, usize)> {
    let mut pairs: Vec<(&str, usize)> = counts.iter().map(|(&w, &n)| (w, n)).collect();
    pairs.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    pairs
}

// ── HashSet ─────────────────────────────────────────────────────────────────

fn sorted<'a>(it: impl Iterator<Item = &'a i32>) -> Vec<i32> {
    let mut v: Vec<i32> = it.copied().collect();
    v.sort_unstable();
    v
}

pub fn in_both(a: &HashSet<i32>, b: &HashSet<i32>) -> Vec<i32> {
    sorted(a.intersection(b))
}

pub fn in_either(a: &HashSet<i32>, b: &HashSet<i32>) -> Vec<i32> {
    sorted(a.union(b))
}

pub fn only_in_first(a: &HashSet<i32>, b: &HashSet<i32>) -> Vec<i32> {
    sorted(a.difference(b))
}

pub fn in_exactly_one(a: &HashSet<i32>, b: &HashSet<i32>) -> Vec<i32> {
    sorted(a.symmetric_difference(b))
}

pub fn no_overlap(a: &HashSet<i32>, b: &HashSet<i32>) -> bool {
    a.is_disjoint(b)
}

pub fn spellable(word: &str, tiles: &str) -> bool {
    let need: HashSet<char> = word.chars().collect();
    let have: HashSet<char> = tiles.chars().collect();
    need.is_subset(&have)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
