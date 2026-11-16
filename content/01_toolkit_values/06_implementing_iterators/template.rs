// Implementing Iterators — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: Countdown may not wrap a Range; ChunksOf may not use slice::chunks; Dedup may not collect.
#![allow(unused_imports, unused_variables, dead_code)]

use std::iter::{FusedIterator, Peekable, Rev};
use std::marker::PhantomData;
use std::{slice, vec};

// ── Countdown ───────────────────────────────────────────────────────────────

/// Counts down from `from` to 1. Front: `from, from-1, …, 1`; back: `1, 2, …`.
/// The two ends meet in the middle and never yield a number twice.
/// (`Iterator`, `size_hint`, `DoubleEndedIterator`, `ExactSizeIterator`)
#[derive(Debug, Clone)]
pub struct Countdown {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: PhantomData<u32>,
}

impl Countdown {
    pub fn new(from: u32) -> Self {
        todo!()
    }
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        todo!()
    }

    /// Exact: `(remaining, Some(remaining))`.
    fn size_hint(&self) -> (usize, Option<usize>) {
        todo!()
    }
}

impl DoubleEndedIterator for Countdown {
    fn next_back(&mut self) -> Option<u32> {
        todo!()
    }
}

/// Nothing to write: `len()` comes from an exact `size_hint`.
impl ExactSizeIterator for Countdown {}

// ── Fibonacci ───────────────────────────────────────────────────────────────

/// 0, 1, 1, 2, 3, 5, … as `u64`, stopping (for good) before the first value
/// that doesn't fit — so 94 values in all. (`Iterator`, `checked_add`)
#[derive(Debug, Clone)]
pub struct Fibonacci {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: PhantomData<u64>,
}

impl Fibonacci {
    pub fn new() -> Self {
        todo!()
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
        todo!()
    }
}

/// A promise that `next` keeps returning `None` once it has. Make it true.
impl FusedIterator for Fibonacci {}

// ── Pairs ───────────────────────────────────────────────────────────────────

/// Every pair `(&items[i], &items[j])` with `i < j`, in lexicographic order of
/// `(i, j)`: for `[a, b, c]`, `(a, b), (a, c), (b, c)`. The references borrow
/// from `items`, not from the iterator. (an iterator struct with a lifetime)
pub fn pairs<T>(items: &[T]) -> Pairs<'_, T> {
    todo!()
}

pub struct Pairs<'a, T> {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Pairs<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

// ── ChunksOf ────────────────────────────────────────────────────────────────

/// Consecutive sub-slices of `size` items; the last one may be shorter.
/// Panics with "chunk size must be non-zero" if `size == 0`.
/// (`Iterator<Item = &'a [T]>`, `split_at`, `size_hint`, `ExactSizeIterator`)
pub fn chunks_of<T>(items: &[T], size: usize) -> ChunksOf<'_, T> {
    todo!()
}

pub struct ChunksOf<'a, T> {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: PhantomData<&'a T>,
}

impl<'a, T> Iterator for ChunksOf<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<&'a [T]> {
        todo!()
    }

    /// Exact: the number of chunks still to come.
    fn size_hint(&self) -> (usize, Option<usize>) {
        todo!()
    }
}

impl<T> ExactSizeIterator for ChunksOf<'_, T> {}

// ── Dedup ───────────────────────────────────────────────────────────────────

/// Wraps an iterator, skipping each item equal to the one just yielded:
/// `1, 1, 2, 1` → `1, 2, 1`. Lazy: it must work on an endless input.
/// (a generic adapter, `Peekable::next_if_eq`)
pub fn dedup<I: Iterator>(iter: I) -> Dedup<I> {
    todo!()
}

pub struct Dedup<I: Iterator> {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: PhantomData<I>,
}

impl<I> Iterator for Dedup<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        todo!()
    }
}

// ── Closures as iterators ───────────────────────────────────────────────────

/// The decimal digits of `n`, least significant first. `0` gives `[0]`.
/// (`std::iter::from_fn` with a `move` closure)
pub fn digits_lsb(n: u64) -> Box<dyn Iterator<Item = u64>> {
    todo!()
}

/// `1, base, base², …` for as long as the powers fit in a `u64`. `base >= 2`.
/// (`std::iter::successors`, `checked_mul`)
pub fn powers_of(base: u64) -> Box<dyn Iterator<Item = u64>> {
    todo!()
}

// ── Stack: a collection you can iterate ─────────────────────────────────────

/// A last-in, first-out stack. Iterating yields the top (most recently
/// pushed) item first. The struct and its basic methods are given; write the
/// iteration traits.
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

    /// Borrow the items, top first. (`rev`)
    pub fn iter(&self) -> Rev<slice::Iter<'_, T>> {
        todo!()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// `for x in stack` — consumes the stack, top first.
impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = Rev<vec::IntoIter<T>>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

/// `for x in &stack` — borrows, top first.
impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = &'a T;
    type IntoIter = Rev<slice::Iter<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

/// `iter.collect::<Stack<_>>()` — items are pushed in order, so the last one
/// ends up on top.
impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        todo!()
    }
}

/// `stack.extend(iter)` — pushes each item in order.
impl<T> Extend<T> for Stack<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        todo!()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
