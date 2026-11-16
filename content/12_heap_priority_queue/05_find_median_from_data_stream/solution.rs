// Find Median from Data Stream — reference solution.
// Max-heap for the lower half, min-heap for the upper half; the lower half
// holds the extra element on odd counts. add O(log n), median O(1), space O(n).

use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Default)]
pub struct MedianFinder {
    lower: BinaryHeap<i32>,
    upper: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, num: i32) {
        // Route through `lower` so its maximum moves up, then rebalance sizes.
        self.lower.push(num);
        let top = self.lower.pop().expect("just pushed");
        self.upper.push(Reverse(top));
        if self.upper.len() > self.lower.len() {
            let Reverse(x) = self.upper.pop().expect("upper is non-empty");
            self.lower.push(x);
        }
    }

    pub fn median(&self) -> Option<f64> {
        let low = f64::from(*self.lower.peek()?);
        if self.lower.len() > self.upper.len() {
            return Some(low);
        }
        let Reverse(high) = *self.upper.peek()?;
        Some((low + f64::from(high)) / 2.0)
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
