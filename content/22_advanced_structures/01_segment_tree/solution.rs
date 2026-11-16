// Segment Tree — reference solution.
// Bottom-up array layout: leaves at tree[n..2n], node i = combine(2i, 2i+1).
// Build O(n); set and query climb toward the root in O(log n), keeping a left
// and a right accumulator so non-commutative monoids combine in order.

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
        0
    }

    fn combine(a: i64, b: i64) -> i64 {
        a + b
    }
}

/// Range minimums.
pub struct Min;

impl Monoid for Min {
    type Item = i64;

    fn identity() -> i64 {
        i64::MAX
    }

    fn combine(a: i64, b: i64) -> i64 {
        a.min(b)
    }
}

pub struct SegmentTree<M: Monoid> {
    n: usize,
    /// `tree[1]` is the root; `tree[n + i]` is element `i`. `tree[0]` is unused.
    tree: Vec<M::Item>,
}

impl<M: Monoid> SegmentTree<M> {
    pub fn new(values: &[M::Item]) -> Self {
        let n = values.len();
        let mut tree = vec![M::identity(); n];
        tree.extend_from_slice(values);
        for i in (1..n).rev() {
            tree[i] = M::combine(tree[2 * i], tree[2 * i + 1]);
        }
        SegmentTree { n, tree }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn set(&mut self, index: usize, value: M::Item) {
        assert!(
            index < self.n,
            "index {index} out of bounds for length {}",
            self.n
        );
        let mut i = index + self.n;
        self.tree[i] = value;
        while i > 1 {
            i /= 2;
            self.tree[i] = M::combine(self.tree[2 * i], self.tree[2 * i + 1]);
        }
    }

    pub fn query(&self, range: Range<usize>) -> M::Item {
        let Range { start, end } = range;
        assert!(
            start <= end && end <= self.n,
            "range {start}..{end} out of bounds for length {}",
            self.n
        );
        let (mut left, mut right) = (M::identity(), M::identity());
        let (mut l, mut r) = (start + self.n, end + self.n);
        while l < r {
            if l % 2 == 1 {
                left = M::combine(left, self.tree[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                right = M::combine(self.tree[r], right);
            }
            l /= 2;
            r /= 2;
        }
        M::combine(left, right)
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
