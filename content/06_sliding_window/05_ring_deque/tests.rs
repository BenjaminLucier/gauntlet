// Tests for Ring Buffer Deque — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::cell::Cell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Tiny deterministic PRNG (xorshift64) so random tests are reproducible.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

fn to_vec<T: Clone>(d: &RingDeque<T>) -> Vec<T> {
    d.iter().cloned().collect()
}

/// Counts its drops, to check every element is dropped exactly once.
#[derive(Debug)]
struct Counted(Rc<Cell<usize>>);

impl Drop for Counted {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

/// Deliberately not `Clone`, `Copy` or `Default`.
#[derive(Debug, PartialEq)]
struct Token(u32);

#[test]
fn readme_example() {
    let mut d = RingDeque::with_capacity(4);
    d.push_back(2);
    d.push_back(3);
    d.push_front(1);
    assert_eq!(d.front(), Some(&1));
    assert_eq!(d.back(), Some(&3));
    assert_eq!(d.get(1), Some(&2));
    assert_eq!(d.get(3), None);
    assert_eq!(d.pop_front(), Some(1));
    d.push_back(4);
    d.push_back(5);
    assert_eq!(d.capacity(), 4);
    d.push_front(0);
    assert!(d.capacity() >= 5);
    assert_eq!(to_vec(&d), [0, 2, 3, 4, 5]);
    assert_eq!(d.iter().rev().copied().collect::<Vec<_>>(), [5, 4, 3, 2, 0]);
    assert_eq!(d[1], 2);
    assert_eq!(d.pop_back(), Some(5));

    let mut e: RingDeque<char> = "bc".chars().collect();
    e.extend(['d', 'e']);
    e.push_front('a');
    assert_eq!(e.iter().collect::<String>(), "abcde");
}

#[test]
fn empty_deque_returns_none_everywhere() {
    let mut d: RingDeque<i32> = RingDeque::new();
    assert!(d.is_empty());
    assert_eq!(d.len(), 0);
    assert_eq!(d.front(), None);
    assert_eq!(d.back(), None);
    assert_eq!(d.get(0), None);
    assert_eq!(d.pop_front(), None);
    assert_eq!(d.pop_back(), None);
    assert_eq!(d.iter().next(), None);
    let e: RingDeque<i32> = RingDeque::default();
    assert!(e.is_empty());
}

#[test]
fn push_front_onto_a_fresh_deque() {
    // head - 1 underflows, and `% capacity` divides by zero, on a 0-slot buffer.
    let mut d = RingDeque::new();
    d.push_front('b');
    d.push_front('a');
    assert_eq!(to_vec(&d), ['a', 'b']);
    let mut z = RingDeque::with_capacity(0);
    z.push_front(1);
    assert_eq!(z.back(), Some(&1));
}

#[test]
fn pop_back_and_pop_front_on_zero_capacity() {
    let mut d: RingDeque<u8> = RingDeque::with_capacity(0);
    assert_eq!(d.pop_back(), None);
    assert_eq!(d.pop_front(), None);
    assert_eq!(d.front(), None);
    assert_eq!(d.iter().count(), 0);
}

#[test]
fn single_element_is_front_and_back() {
    let mut d = RingDeque::new();
    d.push_back(7);
    assert_eq!(d.front(), Some(&7));
    assert_eq!(d.back(), Some(&7));
    assert_eq!(d.pop_back(), Some(7));
    assert!(d.is_empty());
    d.push_front(8);
    assert_eq!(d.pop_front(), Some(8));
    assert!(d.is_empty());
}

#[test]
fn works_as_a_queue_and_as_a_stack_from_both_ends() {
    let mut d = RingDeque::new();
    for i in 0..5 {
        d.push_back(i);
    }
    assert_eq!(d.pop_front(), Some(0));
    assert_eq!(d.pop_back(), Some(4));
    for i in 10..13 {
        d.push_front(i);
    }
    assert_eq!(to_vec(&d), [12, 11, 10, 1, 2, 3]);
    assert_eq!(d.pop_front(), Some(12));
    assert_eq!(d.pop_front(), Some(11));
}

#[test]
fn wraps_around_without_growing() {
    let mut d = RingDeque::with_capacity(4);
    let cap = d.capacity();
    assert!(cap >= 4);
    for round in 0..50u32 {
        d.push_back(round);
        d.push_back(round + 1000);
        assert_eq!(d.pop_front(), Some(round));
        assert_eq!(d.pop_front(), Some(round + 1000));
    }
    assert_eq!(d.capacity(), cap, "a deque that never holds 3 items needs no growth");
    d.push_back(1);
    d.push_front(0);
    assert_eq!(to_vec(&d), [0, 1]);
}

#[test]
fn get_counts_from_the_front_across_the_wrap() {
    let mut d = RingDeque::with_capacity(4);
    d.push_back(1);
    d.push_back(2);
    d.push_back(3);
    d.pop_front();
    d.pop_front();
    d.push_back(4);
    d.push_back(5); // [3, 4, 5], physically split across the end of the buffer
    assert_eq!(d.get(0), Some(&3));
    assert_eq!(d.get(1), Some(&4));
    assert_eq!(d.get(2), Some(&5));
    assert_eq!(d.get(3), None, "slots past len hold nothing");
    assert_eq!(d.get(usize::MAX), None);
    assert_eq!(to_vec(&d), [3, 4, 5]);
}

#[test]
fn growing_a_wrapped_buffer_keeps_logical_order() {
    // Copying slot i to slot i when growing scrambles a wrapped deque.
    let mut d = RingDeque::with_capacity(4);
    let cap = d.capacity();
    for i in 0..cap as u32 {
        d.push_back(i);
    }
    d.pop_front();
    d.pop_front();
    d.push_back(100);
    d.push_back(101); // full and wrapped
    assert_eq!(d.capacity(), cap);
    d.push_back(102); // grow
    d.push_front(99);
    let mut expected: Vec<u32> = (2..cap as u32).collect();
    expected.insert(0, 99);
    expected.extend([100, 101, 102]);
    assert_eq!(to_vec(&d), expected);
    assert_eq!(d.front(), Some(&99));
    assert_eq!(d.back(), Some(&102));
}

#[test]
fn growing_through_push_front_keeps_order() {
    let mut d = RingDeque::new();
    for i in 0..100 {
        d.push_front(i);
    }
    assert_eq!(to_vec(&d), (0..100).rev().collect::<Vec<_>>());
    for i in 0..100 {
        assert_eq!(d.pop_back(), Some(i));
    }
}

#[test]
fn growth_is_geometric() {
    let mut d = RingDeque::new();
    let mut changes = 0;
    let mut cap = d.capacity();
    for i in 0..100_000u32 {
        if i % 2 == 0 {
            d.push_back(i);
        } else {
            d.push_front(i);
        }
        if d.capacity() != cap {
            changes += 1;
            cap = d.capacity();
        }
        assert!(d.capacity() >= d.len());
    }
    assert!(changes <= 40, "the buffer was reallocated {changes} times for 100k pushes");
    assert!(d.capacity() < 4 * d.len(), "capacity {} for len {}", d.capacity(), d.len());
}

#[test]
fn iterators_borrow_and_are_independent() {
    let mut d = RingDeque::with_capacity(3);
    d.push_back(2);
    d.push_back(3);
    d.push_front(1);
    let mut a = d.iter();
    let b = d.iter();
    assert_eq!(a.next(), Some(&1));
    assert_eq!(b.collect::<Vec<_>>(), [&1, &2, &3]);
    assert_eq!(a.collect::<Vec<_>>(), [&2, &3]);
    assert_eq!(d.len(), 3, "iterating doesn't consume");
}

#[test]
fn reverse_iteration_across_the_wrap() {
    // Capacity 4, head moved to slot 2, so the elements wrap: slots [3, 4, 1, 2].
    let mut d = RingDeque::with_capacity(4);
    d.push_back(0);
    d.push_back(0);
    d.push_back(1);
    d.push_back(2);
    d.pop_front();
    d.pop_front();
    d.push_back(3);
    d.push_back(4);
    assert_eq!(d.capacity(), 4);
    assert_eq!(d.iter().rev().copied().collect::<Vec<_>>(), [4, 3, 2, 1]);
    // Mixing both ends: they meet in the middle, never yielding twice.
    let mut it = d.iter();
    assert_eq!(it.next_back(), Some(&4));
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next_back(), Some(&3));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);
    let empty: RingDeque<u8> = RingDeque::new();
    assert_eq!(empty.iter().next_back(), None);
}

#[test]
fn collection_traits() {
    let mut d: RingDeque<u32> = (1..=3).collect();
    d.push_front(0);
    d.extend(4..6);
    assert_eq!(to_vec(&d), [0, 1, 2, 3, 4, 5]);
    let mut sum = 0;
    for x in &d {
        sum += x;
    }
    assert_eq!(sum, 15);
    assert_eq!((d[0], d[3], d[5]), (0, 3, 5));
    let empty: RingDeque<u8> = std::iter::empty().collect();
    assert!(empty.is_empty());
}

#[test]
#[should_panic(expected = "out of bounds")]
fn index_out_of_bounds_panics() {
    let d: RingDeque<i32> = (0..3).collect();
    let _ = d[3];
}

#[test]
fn works_without_clone_or_default() {
    let mut d = RingDeque::new();
    for i in 0..10 {
        d.push_front(Token(i));
    }
    assert_eq!(d.pop_back(), Some(Token(0)));
    assert_eq!(d.pop_front(), Some(Token(9)));
    assert_eq!(d.front(), Some(&Token(8)));
    assert_eq!(d.iter().map(|t| t.0).sum::<u32>(), (1..9).sum());
}

#[test]
fn every_element_is_dropped_exactly_once() {
    let drops = Rc::new(Cell::new(0));
    {
        let mut d = RingDeque::with_capacity(4);
        for i in 0..30 {
            if i % 3 == 0 {
                d.push_front(Counted(Rc::clone(&drops)));
            } else {
                d.push_back(Counted(Rc::clone(&drops)));
            }
        }
        drop(d.pop_front());
        drop(d.pop_back());
        assert_eq!(drops.get(), 2, "growing must move elements, not drop them");
    }
    assert_eq!(drops.get(), 30, "dropping the deque drops what's left");
}

#[test]
fn random_operations_match_vecdeque() {
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    let mut d = RingDeque::with_capacity(rng.below(5) as usize);
    let mut model: VecDeque<u64> = VecDeque::new();
    for step in 0..30_000 {
        let n = model.len() as u64;
        match rng.below(8) {
            0 => {
                let v = rng.next();
                d.push_front(v);
                model.push_front(v);
            }
            1 => {
                let v = rng.next();
                d.push_back(v);
                model.push_back(v);
            }
            2 => assert_eq!(d.pop_front(), model.pop_front(), "step {step}"),
            3 => assert_eq!(d.pop_back(), model.pop_back(), "step {step}"),
            4 => {
                assert_eq!(d.front(), model.front(), "step {step}");
                assert_eq!(d.back(), model.back(), "step {step}");
            }
            5 | 6 => {
                let i = rng.below(n + 2) as usize;
                assert_eq!(d.get(i), model.get(i), "step {step}: get({i})");
            }
            _ => {
                if rng.below(20) == 0 {
                    assert!(d.iter().eq(model.iter()), "step {step}: iter");
                }
            }
        }
        assert_eq!(d.len(), model.len(), "step {step}");
    }
    assert!(d.iter().eq(model.iter()));
}

#[test]
fn many_operations_at_the_front() {
    // 1.6 * 10^5 operations, all at the front. Shifting every element on
    // push_front or pop_front moves about 6.4 * 10^9 elements of 32 bytes;
    // a ring buffer does O(1) work per operation.
    let n = 80_000u64;
    let item = |i: u64| [i, !i, i ^ 0x5555, i.wrapping_mul(3)];
    let mut d = RingDeque::new();
    for i in 0..n {
        d.push_front(item(i));
    }
    assert_eq!(d.len(), n as usize, "n = {n}");
    assert_eq!(d.get(0), Some(&item(n - 1)), "n = {n}");
    assert_eq!(d.get(n as usize - 1), Some(&item(0)), "n = {n}");
    for i in (0..n).rev() {
        assert_eq!(d.pop_front(), Some(item(i)), "n = {n}");
    }
    assert!(d.is_empty());
}
