// Tests for Singly Linked List — shared by template.rs, solution.rs and your attempt.
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

fn to_vec<T: Clone>(list: &LinkedList<T>) -> Vec<T> {
    list.iter().cloned().collect()
}

/// Runs `f` on a thread with a small stack (so anything that recurses once per
/// node overflows), re-raising its panic so "not yet implemented" surfaces.
fn on_small_stack<F: FnOnce() + Send + 'static>(f: F) {
    let handle = std::thread::Builder::new()
        .stack_size(512 * 1024)
        .spawn(f)
        .expect("spawn test thread");
    if let Err(panic) = handle.join() {
        std::panic::resume_unwind(panic);
    }
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

const DEEP: u32 = 100_000;

#[test]
fn readme_example() {
    let mut list = LinkedList::new();
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);
    assert_eq!(list.len(), 3);
    assert_eq!(list.peek_front(), Some(&1));
    list.reverse();
    assert_eq!(to_vec(&list), [3, 2, 1]);
    for x in &mut list {
        *x *= 10;
    }
    assert_eq!(list.pop_front(), Some(30));
    assert_eq!(list.into_iter().collect::<Vec<_>>(), [20, 10]);
    let words: LinkedList<&str> = ["a", "b", "c"].into_iter().collect();
    assert_eq!(words.peek_front(), Some(&"a"));
}

#[test]
fn empty_list_returns_none_everywhere() {
    let mut list: LinkedList<i32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
    assert_eq!(list.peek_front(), None);
    assert_eq!(list.peek_front_mut(), None);
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.iter().next(), None);
    assert_eq!(list.iter_mut().next(), None);
    list.reverse();
    assert!(list.is_empty());
    assert_eq!(list.into_iter().next(), None);
    let d: LinkedList<i32> = LinkedList::default();
    assert!(d.is_empty());
}

#[test]
fn push_and_pop_at_the_front_is_lifo() {
    let mut list = LinkedList::new();
    for i in 0..5 {
        list.push_front(i);
        assert_eq!(list.len(), i as usize + 1);
    }
    for i in (0..5).rev() {
        assert_eq!(list.pop_front(), Some(i));
    }
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.len(), 0, "pop_front on empty must not underflow len");
    list.push_front(9);
    assert_eq!(to_vec(&list), [9]);
}

#[test]
fn peek_front_mut_edits_in_place() {
    let mut list: LinkedList<String> = ["x", "y"].into_iter().map(String::from).collect();
    list.peek_front_mut().expect("non-empty").push('!');
    assert_eq!(list.peek_front().map(String::as_str), Some("x!"));
    assert_eq!(list.len(), 2);
}

#[test]
fn collect_keeps_iteration_order() {
    let list: LinkedList<i32> = (1..=5).collect();
    assert_eq!(list.len(), 5, "collect must count the nodes it builds");
    assert_eq!(to_vec(&list), [1, 2, 3, 4, 5]);
    let empty: LinkedList<i32> = std::iter::empty().collect();
    assert!(empty.is_empty());
}

#[test]
fn reverse_small_lists() {
    let mut one: LinkedList<i32> = std::iter::once(1).collect();
    one.reverse();
    assert_eq!(to_vec(&one), [1]);
    let mut two: LinkedList<i32> = (1..=2).collect();
    two.reverse();
    assert_eq!(to_vec(&two), [2, 1]);
    let mut five: LinkedList<i32> = (1..=5).collect();
    five.reverse();
    assert_eq!(to_vec(&five), [5, 4, 3, 2, 1]);
    assert_eq!(five.len(), 5, "reverse keeps the length");
    five.reverse();
    assert_eq!(to_vec(&five), [1, 2, 3, 4, 5]);
}

#[test]
fn list_is_usable_after_reverse() {
    let mut list: LinkedList<i32> = (1..=3).collect();
    list.reverse();
    list.push_front(4);
    assert_eq!(list.pop_front(), Some(4));
    assert_eq!(list.pop_front(), Some(3));
    list.push_front(0);
    assert_eq!(to_vec(&list), [0, 2, 1]);
}

#[test]
fn three_kinds_of_for_loop() {
    let mut list: LinkedList<u32> = (1..=4).collect();
    let mut sum = 0;
    for x in &list {
        sum += x;
    }
    assert_eq!(sum, 10);
    for x in &mut list {
        *x += 1;
    }
    let mut owned = Vec::new();
    for x in list {
        owned.push(x);
    }
    assert_eq!(owned, [2, 3, 4, 5]);
}

#[test]
fn iterators_are_independent() {
    let list: LinkedList<i32> = (0..4).collect();
    let mut a = list.iter();
    let b = list.iter();
    assert_eq!(a.next(), Some(&0));
    assert_eq!(b.collect::<Vec<_>>(), [&0, &1, &2, &3]);
    assert_eq!(a.collect::<Vec<_>>(), [&1, &2, &3]);
    assert_eq!(list.len(), 4, "iterating doesn't consume");
}

#[test]
fn iter_mut_visits_every_node_once() {
    let mut list: LinkedList<i32> = (1..=6).collect();
    list.iter_mut().for_each(|x| *x *= -1);
    assert_eq!(to_vec(&list), [-1, -2, -3, -4, -5, -6]);
    assert_eq!(list.iter_mut().count(), 6);
}

#[test]
fn works_without_clone_or_default() {
    let mut list = LinkedList::new();
    for i in 0..5 {
        list.push_front(Token(i));
    }
    list.reverse();
    assert_eq!(list.peek_front(), Some(&Token(0)));
    if let Some(t) = list.iter_mut().nth(2) {
        t.0 = 20;
    }
    assert_eq!(
        list.into_iter().collect::<Vec<_>>(),
        [Token(0), Token(1), Token(20), Token(3), Token(4)]
    );
}

#[test]
fn every_element_is_dropped_exactly_once() {
    let drops = Rc::new(Cell::new(0));
    {
        let mut list = LinkedList::new();
        for _ in 0..20 {
            list.push_front(Counted(Rc::clone(&drops)));
        }
        drop(list.pop_front());
        list.reverse();
        assert_eq!(drops.get(), 1, "reverse must not drop nodes");
        let mut it = list.into_iter();
        drop(it.next());
        drop(it.next());
        assert_eq!(drops.get(), 3);
        // `it` still owns 17 elements.
    }
    assert_eq!(drops.get(), 20, "dropping a half-used iterator drops the rest");
}

#[test]
fn len_asked_many_times() {
    // Counting the nodes in `len` makes this 1_000 * 600_000 = 6 * 10^8 node
    // visits; a stored length makes it 600_000 reads.
    let n = 1_000usize;
    let reps = 600_000;
    let list: LinkedList<usize> = (0..n).collect();
    let mut total = 0usize;
    for _ in 0..reps {
        total += list.len();
    }
    assert_eq!(total, n * reps, "n = {n}, reps = {reps}");
    assert!(!list.is_empty());
}

#[test]
fn collect_a_long_list() {
    // 62_000 items. Finding the tail from the head for every item is
    // n^2 / 2 = 1.9 * 10^9 node visits; keeping a tail pointer makes it n.
    let n = 62_000u32;
    let list: LinkedList<u32> = (0..n).collect();
    assert_eq!(list.len(), n as usize, "n = {n}");
    assert_eq!(list.peek_front(), Some(&0), "n = {n}");
    assert_eq!(list.iter().last(), Some(&(n - 1)), "n = {n}");
    assert!(list.iter().copied().eq(0..n), "n = {n}");
}

#[test]
fn random_operations_match_vecdeque() {
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    let mut list = LinkedList::new();
    let mut model: VecDeque<u64> = VecDeque::new();
    for step in 0..20_000 {
        match rng.below(8) {
            0..=2 => {
                let v = rng.next();
                list.push_front(v);
                model.push_front(v);
            }
            3 | 4 => assert_eq!(list.pop_front(), model.pop_front(), "step {step}"),
            5 => assert_eq!(list.peek_front(), model.front(), "step {step}"),
            6 => {
                if let Some(x) = list.peek_front_mut() {
                    *x = x.wrapping_mul(3);
                }
                if let Some(x) = model.front_mut() {
                    *x = x.wrapping_mul(3);
                }
            }
            _ => {
                if rng.below(10) == 0 {
                    list.reverse();
                    model.make_contiguous().reverse();
                }
            }
        }
        assert_eq!(list.len(), model.len(), "step {step}");
    }
    assert!(list.iter().eq(model.iter()));
}

#[test]
fn dropping_a_long_list_does_not_recurse() {
    on_small_stack(|| {
        let mut list = LinkedList::new();
        for i in 0..DEEP {
            list.push_front(i);
        }
        assert_eq!(list.len(), DEEP as usize);
        drop(list);
    });
}

#[test]
fn long_list_reverse_and_iterate() {
    on_small_stack(|| {
        let mut list = LinkedList::new();
        for i in (0..DEEP).rev() {
            list.push_front(i);
        }
        list.reverse();
        assert_eq!(list.peek_front(), Some(&(DEEP - 1)));
        for x in &mut list {
            *x += 1;
        }
        let sum: u64 = list.iter().map(|&x| u64::from(x)).sum();
        assert_eq!(sum, u64::from(DEEP) * u64::from(DEEP + 1) / 2);
        let mut it = list.into_iter();
        assert_eq!(it.next(), Some(DEEP));
        // Dropping the iterator drops the remaining nodes.
    });
}
