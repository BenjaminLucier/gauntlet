// Union-Find — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no HashMap/HashSet/BTreeMap — Vec<usize> arrays. find must not recurse.
#![allow(unused_imports, unused_variables, dead_code)]

use std::mem;

/// Disjoint sets over `0..n`, with union by size and path compression.
pub struct UnionFind {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: (),
}

impl UnionFind {
    /// `n` singleton sets.
    pub fn new(n: usize) -> Self {
        todo!()
    }

    /// Number of elements.
    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Representative of `x`'s set; compresses the path it walks.
    pub fn find(&mut self, x: usize) -> usize {
        todo!()
    }

    /// Merge the sets of `a` and `b`. `false` if they were already one set.
    pub fn union(&mut self, a: usize, b: usize) -> bool {
        todo!()
    }

    pub fn connected(&mut self, a: usize, b: usize) -> bool {
        todo!()
    }

    /// Number of elements in `x`'s set.
    pub fn component_size(&mut self, x: usize) -> usize {
        todo!()
    }

    /// Number of disjoint sets.
    pub fn count(&self) -> usize {
        todo!()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
