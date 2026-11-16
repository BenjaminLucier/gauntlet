// Minimum Spanning Tree — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// An undirected weighted edge `(u, v, weight)`.
pub type Edge = (usize, usize, u64);

/// Total weight and chosen edges of a minimum spanning tree, or `None` if the
/// graph is disconnected.
pub fn kruskal(n: usize, edges: &[Edge]) -> Option<(u64, Vec<Edge>)> {
    todo!()
}

/// Total weight of a minimum spanning tree of the undirected graph `adj`, or
/// `None` if it is disconnected.
pub fn prim(adj: &[Vec<(usize, u64)>]) -> Option<u64> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
