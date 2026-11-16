// Segment Tree — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no O(n) scans per query or update — every operation but `new` is O(log n).
#![allow(unused_imports, unused_variables, dead_code)]

use std::marker::PhantomData;
use std::ops::Range;

/// An associative `combine` with an `identity` element.
pub trait Monoid {
    type Item: Copy;
    fn identity() -> Self::Item;
    fn combine(a: Self::Item, b: Self::Item) -> Self::Item;
}

/// Range sums.
pub struct Sum;

impl Monoid for Sum {
    type Item = i64;

    fn identity() -> i64 {
        todo!()
    }

    fn combine(a: i64, b: i64) -> i64 {
        todo!()
    }
}

/// Range minimums.
pub struct Min;

impl Monoid for Min {
    type Item = i64;

    fn identity() -> i64 {
        todo!()
    }

    fn combine(a: i64, b: i64) -> i64 {
        todo!()
    }
}

pub struct SegmentTree<M: Monoid> {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: PhantomData<M>,
}

impl<M: Monoid> SegmentTree<M> {
    /// Build over `values` in O(n).
    pub fn new(values: &[M::Item]) -> Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Replace element `index`. Panics if `index >= len`.
    pub fn set(&mut self, index: usize, value: M::Item) {
        todo!()
    }

    /// Combine the elements in `range`, left to right. An empty range gives
    /// `M::identity()`. Panics if `start > end` or `end > len`.
    pub fn query(&self, range: Range<usize>) -> M::Item {
        todo!()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
