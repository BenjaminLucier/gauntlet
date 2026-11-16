// VecDeque and BinaryHeap — reference solution.
// VecDeque: O(1) at both ends, O(min(k, n - k)) rotations. BinaryHeap: O(log n)
// push/pop, O(1) peek, O(n) `from(Vec)`; `Reverse` turns the max-heap into a min-heap.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

// ── VecDeque: both ends ─────────────────────────────────────────────────────

pub fn push_both(front: &[i32], back: &[i32]) -> VecDeque<i32> {
    let mut d = VecDeque::with_capacity(front.len() + back.len());
    for &x in front {
        d.push_front(x);
    }
    for &x in back {
        d.push_back(x);
    }
    d
}

pub fn is_palindrome(s: &str) -> bool {
    let mut d: VecDeque<char> = s.chars().collect();
    while let (Some(a), Some(b)) = (d.pop_front(), d.pop_back()) {
        if a != b {
            return false;
        }
    }
    true
}

pub fn rotate(d: &mut VecDeque<i32>, k: isize) {
    if d.is_empty() {
        return;
    }
    let steps = k.unsigned_abs() % d.len();
    if k >= 0 {
        d.rotate_right(steps);
    } else {
        d.rotate_left(steps);
    }
}

pub fn elimination_order(n: usize, k: usize) -> Vec<usize> {
    let mut circle: VecDeque<usize> = (1..=n).collect();
    let mut order = Vec::with_capacity(n);
    while !circle.is_empty() {
        let skip = (k - 1) % circle.len();
        circle.rotate_left(skip);
        order.extend(circle.pop_front());
    }
    order
}

// ── VecDeque: sliding buffers ───────────────────────────────────────────────

pub fn last_k(stream: &[i32], k: usize) -> Vec<i32> {
    let mut buf = VecDeque::with_capacity(k + 1);
    for &x in stream {
        buf.push_back(x);
        if buf.len() > k {
            buf.pop_front();
        }
    }
    buf.into()
}

pub fn recent_counts(pings: &[u32], window: u32) -> Vec<usize> {
    let mut recent = VecDeque::new();
    pings
        .iter()
        .map(|&t| {
            recent.push_back(t);
            let oldest = t.saturating_sub(window);
            while recent.front().is_some_and(|&p| p < oldest) {
                recent.pop_front();
            }
            recent.len()
        })
        .collect()
}

// ── VecDeque: as slices ─────────────────────────────────────────────────────

pub fn sort_deque(d: &mut VecDeque<i32>) {
    d.make_contiguous().sort_unstable();
}

pub fn total(d: &VecDeque<i64>) -> i64 {
    let (a, b) = d.as_slices();
    a.iter().sum::<i64>() + b.iter().sum::<i64>()
}

pub fn middle(d: &VecDeque<char>, start: usize, end: usize) -> String {
    d.range(start..end).collect()
}

// ── BinaryHeap ──────────────────────────────────────────────────────────────

pub fn peek_max(heap: &BinaryHeap<i32>) -> Option<i32> {
    heap.peek().copied()
}

pub fn largest_k(xs: &[i32], k: usize) -> Vec<i32> {
    let mut heap = BinaryHeap::from(xs.to_vec());
    std::iter::from_fn(|| heap.pop()).take(k).collect()
}

pub fn smallest_k(xs: &[i32], k: usize) -> Vec<i32> {
    let mut heap: BinaryHeap<Reverse<i32>> = xs.iter().copied().map(Reverse).collect();
    std::iter::from_fn(|| heap.pop().map(|Reverse(x)| x))
        .take(k)
        .collect()
}

pub fn heap_sorted(xs: Vec<i32>) -> Vec<i32> {
    BinaryHeap::from(xs).into_sorted_vec()
}

pub fn merge_into(heap: &mut BinaryHeap<i32>, other: &mut BinaryHeap<i32>) {
    heap.append(other);
}

pub fn drain_total(heap: &mut BinaryHeap<u64>) -> u64 {
    heap.drain().sum()
}

pub fn halve_largest(heap: &mut BinaryHeap<u32>) {
    if let Some(mut top) = heap.peek_mut() {
        *top /= 2;
    }
}

pub fn drop_odds(heap: &mut BinaryHeap<i32>) {
    heap.retain(|x| x % 2 == 0);
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
