// Minimum Spanning Tree — reference solution.
// Kruskal: sort edges by weight, keep those joining two union-find components
// (path halving + union by size). Prim: lazy BinaryHeap of edges leaving the
// tree, skipping stale ones. Both O(m log m) time, O(n + m) space.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// An undirected weighted edge `(u, v, weight)`.
pub type Edge = (usize, usize, u64);

/// Disjoint sets with path halving and union by size.
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    /// Merge the sets of `a` and `b`; `false` if they were already one set.
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b {
            return false;
        }
        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.parent[b] = a;
        self.size[a] += self.size[b];
        true
    }
}

/// Total weight and chosen edges of a minimum spanning tree, or `None` if the
/// graph is disconnected.
pub fn kruskal(n: usize, edges: &[Edge]) -> Option<(u64, Vec<Edge>)> {
    let mut sorted = edges.to_vec();
    sorted.sort_unstable_by_key(|&(_, _, w)| w);
    let mut sets = UnionFind::new(n);
    let target = n.saturating_sub(1);
    let mut tree = Vec::with_capacity(target);
    for (u, v, w) in sorted {
        if tree.len() == target {
            break;
        }
        if sets.union(u, v) {
            tree.push((u, v, w));
        }
    }
    (tree.len() == target).then(|| (tree.iter().map(|&(_, _, w)| w).sum(), tree))
}

/// Total weight of a minimum spanning tree of the undirected graph `adj`, or
/// `None` if it is disconnected.
pub fn prim(adj: &[Vec<(usize, u64)>]) -> Option<u64> {
    let n = adj.len();
    if n == 0 {
        return Some(0);
    }
    let mut in_tree = vec![false; n];
    let mut heap = BinaryHeap::from([Reverse((0, 0))]);
    let (mut total, mut added) = (0, 0);
    while let Some(Reverse((w, u))) = heap.pop() {
        if in_tree[u] {
            continue; // a cheaper edge reached u first
        }
        in_tree[u] = true;
        total += w;
        added += 1;
        if added == n {
            return Some(total);
        }
        heap.extend(
            adj[u]
                .iter()
                .filter(|&&(v, _)| !in_tree[v])
                .map(|&(v, w)| Reverse((w, v))),
        );
    }
    None
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
