// Dijkstra's Algorithm — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: use BinaryHeap — no O(V) scan for the closest vertex, no Bellman-Ford relaxation.
#![allow(unused_imports, unused_variables, dead_code)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Shortest distance from `source` to every vertex; `None` if unreachable
/// (or reachable only by paths whose length overflows `u64`).
pub fn dijkstra(adj: &[Vec<(usize, u64)>], source: usize) -> Vec<Option<u64>> {
    todo!()
}

/// A shortest path `[s, …, t]` and its length, or `None` if unreachable.
pub fn shortest_path(adj: &[Vec<(usize, u64)>], s: usize, t: usize) -> Option<(u64, Vec<usize>)> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
