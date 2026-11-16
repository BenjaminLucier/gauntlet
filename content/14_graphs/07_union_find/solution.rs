// Union-Find — reference solution.
// Parent array with union by size (the smaller root goes under the larger) and
// path halving in find (each visited node skips to its grandparent). Together:
// O(α(n)) amortized per operation, iterative, O(n) space.

use std::mem;

/// Disjoint sets over `0..n`, with union by size and path compression.
pub struct UnionFind {
    parent: Vec<usize>,
    /// Only meaningful at roots: the number of elements in that root's set.
    size: Vec<usize>,
    sets: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            sets: n,
        }
    }

    pub fn len(&self) -> usize {
        self.parent.len()
    }

    pub fn is_empty(&self) -> bool {
        self.parent.is_empty()
    }

    pub fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            // Path halving: point x at its grandparent, then move there.
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let mut large = self.find(a);
        let mut small = self.find(b);
        if large == small {
            return false;
        }
        if self.size[large] < self.size[small] {
            mem::swap(&mut large, &mut small);
        }
        self.parent[small] = large;
        self.size[large] += self.size[small];
        self.sets -= 1;
        true
    }

    pub fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    pub fn component_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }

    pub fn count(&self) -> usize {
        self.sets
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
