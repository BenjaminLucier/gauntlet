// Redundant Connection — reference solution.
// Union-find: the first edge whose endpoints already share a root closes the
// cycle, and is its last edge in input order. Time O(n·α(n)), space O(n).

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    /// Merges the sets of `a` and `b`; returns false if they were already joined.
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (a, b) = (self.find(a), self.find(b));
        self.parent[a] = b;
        a != b
    }
}

pub fn find_redundant_connection(edges: &[(usize, usize)]) -> Option<(usize, usize)> {
    let max_label = edges.iter().map(|&(a, b)| a.max(b)).max()?;
    let mut uf = UnionFind::new(max_label + 1);
    edges.iter().copied().find(|&(a, b)| !uf.union(a, b))
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
