// Implementing Iterators — reference solution.
// Each type keeps just enough state to produce the next item in O(1); the
// Stack delegates to Vec's own iterators, reversed so the top comes first.

use std::iter::{FusedIterator, Peekable, Rev};
use std::{slice, vec};

// ── Countdown ───────────────────────────────────────────────────────────────

/// Counts down from `from` to 1. Front: `from, from-1, …, 1`; back: `1, 2, …`.
#[derive(Debug, Clone)]
pub struct Countdown {
    /// Everything in `low + 1 ..= high` is still to come.
    low: u32,
    high: u32,
}

impl Countdown {
    pub fn new(from: u32) -> Self {
        Countdown { low: 0, high: from }
    }
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        (self.high > self.low).then(|| {
            self.high -= 1;
            self.high + 1
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = (self.high - self.low) as usize;
        (n, Some(n))
    }
}

impl DoubleEndedIterator for Countdown {
    fn next_back(&mut self) -> Option<u32> {
        (self.high > self.low).then(|| {
            self.low += 1;
            self.low
        })
    }
}

impl ExactSizeIterator for Countdown {}

// ── Fibonacci ───────────────────────────────────────────────────────────────

/// 0, 1, 1, 2, 3, 5, … as `u64`, stopping before the first value that
/// doesn't fit.
#[derive(Debug, Clone)]
pub struct Fibonacci {
    curr: Option<u64>,
    next: Option<u64>,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci { curr: Some(0), next: Some(1) }
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let c = self.curr?;
        let n = self.next;
        self.curr = n;
        self.next = n.and_then(|n| n.checked_add(c));
        Some(c)
    }
}

impl FusedIterator for Fibonacci {}

// ── Pairs ───────────────────────────────────────────────────────────────────

/// Every pair `(&items[i], &items[j])` with `i < j`, in lexicographic order of
/// `(i, j)`.
pub fn pairs<T>(items: &[T]) -> Pairs<'_, T> {
    Pairs { items, i: 0, j: 1 }
}

pub struct Pairs<'a, T> {
    items: &'a [T],
    i: usize,
    j: usize,
}

impl<'a, T> Iterator for Pairs<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.j >= self.items.len() {
            self.i += 1;
            self.j = self.i + 1;
        }
        let a = self.items.get(self.i)?;
        let b = self.items.get(self.j)?;
        self.j += 1;
        Some((a, b))
    }
}

// ── ChunksOf ────────────────────────────────────────────────────────────────

/// Consecutive sub-slices of `size` items; the last one may be shorter.
/// Panics with "chunk size must be non-zero" if `size == 0`.
pub fn chunks_of<T>(items: &[T], size: usize) -> ChunksOf<'_, T> {
    assert!(size != 0, "chunk size must be non-zero");
    ChunksOf { rest: items, size }
}

pub struct ChunksOf<'a, T> {
    rest: &'a [T],
    size: usize,
}

impl<'a, T> Iterator for ChunksOf<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<&'a [T]> {
        if self.rest.is_empty() {
            return None;
        }
        let (head, tail) = self.rest.split_at(self.size.min(self.rest.len()));
        self.rest = tail;
        Some(head)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.rest.len().div_ceil(self.size);
        (n, Some(n))
    }
}

impl<T> ExactSizeIterator for ChunksOf<'_, T> {}

// ── Dedup ───────────────────────────────────────────────────────────────────

/// Wraps an iterator, skipping each item equal to the one just yielded.
pub fn dedup<I: Iterator>(iter: I) -> Dedup<I> {
    Dedup { inner: iter.peekable() }
}

pub struct Dedup<I: Iterator> {
    inner: Peekable<I>,
}

impl<I> Iterator for Dedup<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        let x = self.inner.next()?;
        while self.inner.next_if_eq(&x).is_some() {}
        Some(x)
    }
}

// ── Closures as iterators ───────────────────────────────────────────────────

/// The decimal digits of `n`, least significant first. `0` gives `[0]`.
pub fn digits_lsb(n: u64) -> Box<dyn Iterator<Item = u64>> {
    let mut rest = Some(n);
    Box::new(std::iter::from_fn(move || {
        let x = rest?;
        rest = (x >= 10).then_some(x / 10);
        Some(x % 10)
    }))
}

/// `1, base, base², …` for as long as the powers fit in a `u64`.
pub fn powers_of(base: u64) -> Box<dyn Iterator<Item = u64>> {
    Box::new(std::iter::successors(Some(1u64), move |&p| p.checked_mul(base)))
}

// ── Stack: a collection you can iterate ─────────────────────────────────────

/// A last-in, first-out stack. Iterating yields the top (most recently
/// pushed) item first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> Rev<slice::Iter<'_, T>> {
        self.items.iter().rev()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = Rev<vec::IntoIter<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter().rev()
    }
}

impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = &'a T;
    type IntoIter = Rev<slice::Iter<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Stack { items: iter.into_iter().collect() }
    }
}

impl<T> Extend<T> for Stack<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.items.extend(iter);
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
