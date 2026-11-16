// BFS and DFS — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: dfs_order must not recurse — use a Vec as an explicit stack.
#![allow(unused_imports, unused_variables, dead_code)]

use std::collections::VecDeque;

/// Vertices reachable from `start`, in breadth-first order.
pub fn bfs_order(adj: &[Vec<usize>], start: usize) -> Vec<usize> {
    todo!()
}

/// Vertices reachable from `start`, in the preorder of a recursive DFS that
/// tries `adj[v]` in list order — computed without recursion.
pub fn dfs_order(adj: &[Vec<usize>], start: usize) -> Vec<usize> {
    todo!()
}

/// A path `[s, …, t]` with the fewest edges, or `None` if `t` is unreachable.
pub fn shortest_path_unweighted(adj: &[Vec<usize>], s: usize, t: usize) -> Option<Vec<usize>> {
    todo!()
}

/// Component id per vertex (undirected), numbered in order of lowest vertex.
pub fn connected_components(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    todo!()
}

/// Can the (undirected) graph be 2-coloured?
pub fn is_bipartite(n: usize, edges: &[(usize, usize)]) -> bool {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
