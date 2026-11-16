// mem::take, replace and swap — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no clone(), to_owned() or to_string() on values you already own — move them.
// (clippy::ptr_arg is allowed only because `todo!()` bodies hide that the `&mut String`/`&mut Vec` is needed.)
#![allow(unused_imports, unused_variables, dead_code, clippy::ptr_arg)]

use std::collections::HashSet;
use std::mem::{self, Discriminant};
use std::num::NonZeroU32;

// ── Taking and replacing ────────────────────────────────────────────────────

/// Move the string out, leaving an empty one behind. (`mem::take`)
pub fn take_string(s: &mut String) -> String {
    todo!()
}

/// Put `new_name` in the slot and return the old name. (`mem::replace`)
pub fn replace_name(slot: &mut String, new_name: String) -> String {
    todo!()
}

/// If `buffer` isn't empty, move its contents to the end of `out`, leaving
/// `buffer` empty. An empty buffer pushes nothing. (`mem::take`)
pub fn flush(buffer: &mut Vec<u8>, out: &mut Vec<Vec<u8>>) {
    todo!()
}

// ── Swapping ────────────────────────────────────────────────────────────────

/// Exchange the two buffers (double buffering). (`mem::swap`)
pub fn swap_buffers(front: &mut Vec<u8>, back: &mut Vec<u8>) {
    todo!()
}

/// Rotate left: `a` gets `b`'s value, `b` gets `c`'s, `c` gets `a`'s.
/// (`mem::swap`, twice)
pub fn rotate_three(a: &mut String, b: &mut String, c: &mut String) {
    todo!()
}

/// A labelled range whose ends may have been given in either order.
#[derive(Debug, PartialEq, Eq)]
pub struct Span {
    pub start: String,
    pub end: String,
}

/// Make `start <= end` by swapping the two fields if they're out of order.
/// Two `&mut` to *different fields* of one struct are allowed at once.
/// (`mem::swap` on disjoint fields)
pub fn normalize(span: &mut Span) {
    todo!()
}

// ── Option slots ────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub next: Option<Box<Node>>,
}

/// Remove the first node of the list and return its value. (`Option::take`)
pub fn pop_front(head: &mut Option<Box<Node>>) -> Option<i32> {
    todo!()
}

/// Take the string out of the slot only if it's at least `min_len` bytes
/// long; otherwise leave it where it is. (`Option::take_if`)
pub fn take_if_long(slot: &mut Option<String>, min_len: usize) -> Option<String> {
    todo!()
}

// ── Draining ────────────────────────────────────────────────────────────────

/// Move every element out of `v` into a new `Vec`, but keep `v`'s
/// allocation so it can be refilled without reallocating — which
/// `mem::take` would not do. (`Vec::drain(..)`)
pub fn drain_keep_capacity(v: &mut Vec<String>) -> Vec<String> {
    todo!()
}

/// Move every element of `from` onto the end of `to`, in order. `from` is
/// left empty (but keeps its allocation); nothing is cloned. (`Vec::append`)
pub fn move_all(from: &mut Vec<String>, to: &mut Vec<String>) {
    todo!()
}

// ── A state machine ─────────────────────────────────────────────────────────

#[derive(Debug, Default, PartialEq, Eq)]
pub enum Conn {
    #[default]
    Idle,
    Connecting {
        attempts: u32,
    },
    Connected {
        session: String,
    },
    Closed {
        reason: String,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    Dial,
    Fail,
    Accept(String),
    Hangup(String),
}

pub const MAX_ATTEMPTS: u32 = 3;

#[derive(Debug, Default)]
pub struct Client {
    pub state: Conn,
    pub log: Vec<String>,
}

impl Client {
    /// A client in the `Idle` state with an empty log. (`Default`)
    pub fn new() -> Self {
        todo!()
    }

    /// Apply one event (see the table in the README). Owned data moves from
    /// the old state into the new one — no clones.
    /// (`self.state = match mem::take(&mut self.state) { … }`)
    pub fn handle(&mut self, event: Event) {
        todo!()
    }

    /// Hand over the log, leaving it empty. (`mem::take`)
    pub fn take_log(&mut self) -> Vec<String> {
        todo!()
    }
}

// ── Slices ──────────────────────────────────────────────────────────────────

/// Two mutable references into the same slice at once, `i < j < v.len()`.
/// If `i >= j`, panic with the message `pair_mut needs i < j` (check before
/// splitting). (`split_at_mut`, `assert!`)
pub fn pair_mut(v: &mut [i32], i: usize, j: usize) -> (&mut i32, &mut i32) {
    todo!()
}

/// Exchange the contents of two equally long slices. (`swap_with_slice`)
pub fn swap_contents(a: &mut [i32], b: &mut [i32]) {
    todo!()
}

// ── Sizes and discriminants ─────────────────────────────────────────────────

/// `(size of Box<u64>, size of Option<Box<u64>>)`. (`mem::size_of`)
pub fn box_sizes() -> (usize, usize) {
    todo!()
}

/// The sizes of `Option<u32>`, `Option<NonZeroU32>` and `Option<&u8>`.
/// (`mem::size_of`)
pub fn option_sizes() -> (usize, usize, usize) {
    todo!()
}

/// Are `a` and `b` the same variant, whatever their fields hold?
/// (`mem::discriminant`)
pub fn same_variant(a: &Conn, b: &Conn) -> bool {
    todo!()
}

/// How many different variants appear in `states`? (`mem::discriminant`,
/// `HashSet`)
pub fn count_variants(states: &[Conn]) -> usize {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
