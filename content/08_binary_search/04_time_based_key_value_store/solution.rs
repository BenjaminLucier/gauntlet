// Time Based Key-Value Store — reference solution.
// Per-key Vec of (timestamp, value), sorted because timestamps only grow; `get`
// binary-searches for the last time <= t with `partition_point`.
// set O(1) amortized, get O(log k), space O(total sets).

use std::collections::HashMap;

pub struct TimeMap<V> {
    versions: HashMap<String, Vec<(u32, V)>>,
}

impl<V> TimeMap<V> {
    pub fn new() -> Self {
        Self { versions: HashMap::new() }
    }

    pub fn set(&mut self, key: &str, value: V, timestamp: u32) {
        match self.versions.get_mut(key) {
            Some(history) => history.push((timestamp, value)),
            None => {
                self.versions.insert(key.to_owned(), vec![(timestamp, value)]);
            }
        }
    }

    pub fn get(&self, key: &str, timestamp: u32) -> Option<&V> {
        let history = self.versions.get(key)?;
        let at_or_before = history.partition_point(|&(t, _)| t <= timestamp);
        let (_, value) = &history[at_or_before.checked_sub(1)?];
        Some(value)
    }
}

impl<V> Default for TimeMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
