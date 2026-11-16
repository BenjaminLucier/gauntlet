// Topological Sort — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// A directed cycle: edges `v0 → v1 → … → vk → v0`, all vertices distinct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cycle {
    pub vertices: Vec<usize>,
}

/// Kahn's algorithm, always taking the smallest ready vertex: the
/// lexicographically smallest topological order, or a cycle.
pub fn topo_sort_kahn(n: usize, edges: &[(usize, usize)]) -> Result<Vec<usize>, Cycle> {
    todo!()
}

/// DFS reverse-postorder: any topological order, or a cycle.
pub fn topo_sort_dfs(n: usize, edges: &[(usize, usize)]) -> Result<Vec<usize>, Cycle> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
