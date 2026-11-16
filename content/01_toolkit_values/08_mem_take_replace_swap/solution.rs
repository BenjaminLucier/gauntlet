// mem::take, replace and swap — reference solution.
// Every value is moved, never cloned: `mem::take` / `mem::replace` leave something valid
// behind in the `&mut` slot, so ownership can leave through a borrow.

use std::collections::HashSet;
use std::mem::{self, Discriminant};
use std::num::NonZeroU32;

// ── Taking and replacing ────────────────────────────────────────────────────

pub fn take_string(s: &mut String) -> String {
    mem::take(s)
}

pub fn replace_name(slot: &mut String, new_name: String) -> String {
    mem::replace(slot, new_name)
}

pub fn flush(buffer: &mut Vec<u8>, out: &mut Vec<Vec<u8>>) {
    if !buffer.is_empty() {
        out.push(mem::take(buffer));
    }
}

// ── Swapping ────────────────────────────────────────────────────────────────

pub fn swap_buffers(front: &mut Vec<u8>, back: &mut Vec<u8>) {
    mem::swap(front, back);
}

pub fn rotate_three(a: &mut String, b: &mut String, c: &mut String) {
    mem::swap(a, b);
    mem::swap(b, c);
}

#[derive(Debug, PartialEq, Eq)]
pub struct Span {
    pub start: String,
    pub end: String,
}

pub fn normalize(span: &mut Span) {
    if span.start > span.end {
        // Disjoint field borrows: the borrow checker sees `start` and `end` apart.
        mem::swap(&mut span.start, &mut span.end);
    }
}

// ── Option slots ────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub next: Option<Box<Node>>,
}

pub fn pop_front(head: &mut Option<Box<Node>>) -> Option<i32> {
    head.take().map(|node| {
        *head = node.next;
        node.val
    })
}

pub fn take_if_long(slot: &mut Option<String>, min_len: usize) -> Option<String> {
    slot.take_if(|s| s.len() >= min_len)
}

// ── Draining ────────────────────────────────────────────────────────────────

// clippy suggests `mem::take`, but that gives the allocation away: keeping it is the point here.
#[allow(clippy::drain_collect)]
pub fn drain_keep_capacity(v: &mut Vec<String>) -> Vec<String> {
    v.drain(..).collect()
}

pub fn move_all(from: &mut Vec<String>, to: &mut Vec<String>) {
    to.append(from);
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle(&mut self, event: Event) {
        self.state = match (mem::take(&mut self.state), event) {
            (Conn::Idle, Event::Dial) => Conn::Connecting { attempts: 1 },
            (Conn::Connecting { attempts }, Event::Fail) if attempts >= MAX_ATTEMPTS => {
                Conn::Closed { reason: "gave up".to_string() }
            }
            (Conn::Connecting { attempts }, Event::Fail) => Conn::Connecting { attempts: attempts + 1 },
            (Conn::Connecting { .. }, Event::Accept(session)) => Conn::Connected { session },
            (Conn::Connected { session }, Event::Hangup(why)) => {
                self.log.push(session);
                Conn::Closed { reason: why }
            }
            (state, _) => state,
        };
    }

    pub fn take_log(&mut self) -> Vec<String> {
        mem::take(&mut self.log)
    }
}

// ── Slices ──────────────────────────────────────────────────────────────────

pub fn pair_mut(v: &mut [i32], i: usize, j: usize) -> (&mut i32, &mut i32) {
    assert!(i < j, "pair_mut needs i < j");
    let (left, right) = v.split_at_mut(j);
    (&mut left[i], &mut right[0])
}

pub fn swap_contents(a: &mut [i32], b: &mut [i32]) {
    a.swap_with_slice(b);
}

// ── Sizes and discriminants ─────────────────────────────────────────────────

pub fn box_sizes() -> (usize, usize) {
    (mem::size_of::<Box<u64>>(), mem::size_of::<Option<Box<u64>>>())
}

pub fn option_sizes() -> (usize, usize, usize) {
    (
        mem::size_of::<Option<u32>>(),
        mem::size_of::<Option<NonZeroU32>>(),
        mem::size_of::<Option<&u8>>(),
    )
}

pub fn same_variant(a: &Conn, b: &Conn) -> bool {
    mem::discriminant(a) == mem::discriminant(b)
}

pub fn count_variants(states: &[Conn]) -> usize {
    states.iter().map(mem::discriminant).collect::<HashSet<Discriminant<Conn>>>().len()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
