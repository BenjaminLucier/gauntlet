// Tests for Doubly Linked List (Arena) — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::cell::Cell;
use std::collections::HashSet;
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

fn to_vec<T: Clone>(list: &DList<T>) -> Vec<T> {
    list.iter().cloned().collect()
}

/// Empties the list from the back, checking the `prev` links end to end.
fn drain_back<T>(list: &mut DList<T>) -> Vec<T> {
    let mut out: Vec<T> = std::iter::from_fn(|| list.pop_back()).collect();
    out.reverse();
    out
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
    let mut list = DList::new();
    let b = list.push_back("b");
    let d = list.push_back("d");
    let a = list.push_front("a");
    let c = list.insert_after(b, "c").unwrap();
    assert_eq!(to_vec(&list), ["a", "b", "c", "d"]);
    assert_eq!(list.remove(b), Some("b"));
    assert_eq!(list.get(b), None);
    assert_eq!(list.insert_after(b, "x"), Err("x"));
    assert_eq!(list.get(c), Some(&"c"));
    *list.get_mut(a).unwrap() = "A";
    assert_eq!(list.pop_back(), Some("d"));
    assert_eq!(list.get(d), None);
    assert_eq!(list.pop_front(), Some("A"));
    assert_eq!(list.len(), 1);
}

#[test]
fn empty_list_returns_none_everywhere() {
    let mut list: DList<i32> = DList::new();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.pop_back(), None);
    assert_eq!(list.iter().next(), None);
    assert_eq!(list.get(Handle(0)), None, "a handle that was never issued");
    assert_eq!(list.remove(Handle(0)), None);
    assert_eq!(list.insert_after(Handle(0), 5), Err(5));
    let d: DList<i32> = DList::default();
    assert!(d.is_empty());
}

#[test]
fn pushes_at_both_ends() {
    let mut list = DList::new();
    list.push_back(3);
    list.push_front(2);
    list.push_back(4);
    list.push_front(1);
    assert_eq!(to_vec(&list), [1, 2, 3, 4]);
    assert_eq!(list.len(), 4);
    assert_eq!(drain_back(&mut list), [1, 2, 3, 4], "prev links agree with next links");
    assert!(list.is_empty());
}

#[test]
fn pops_at_both_ends_down_to_empty_and_back() {
    let mut list = DList::new();
    for i in 0..4 {
        list.push_back(i);
    }
    assert_eq!(list.pop_front(), Some(0));
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.pop_back(), None);
    // Both head and tail must be reset, or the next push links to a ghost.
    list.push_front(7);
    list.push_back(8);
    assert_eq!(to_vec(&list), [7, 8]);
    assert_eq!(drain_back(&mut list), [7, 8]);
}

#[test]
fn insert_after_the_tail_becomes_the_new_tail() {
    let mut list = DList::new();
    let a = list.push_back('a');
    let b = list.insert_after(a, 'b').unwrap();
    let c = list.insert_after(b, 'c').unwrap();
    assert_eq!(list.pop_back(), Some('c'), "tail must move to the inserted node");
    assert_eq!(list.get(c), None);
    list.push_back('d');
    assert_eq!(to_vec(&list), ['a', 'b', 'd']);
}

#[test]
fn insert_after_in_the_middle_links_both_ways() {
    let mut list = DList::new();
    let one = list.push_back(1);
    list.push_back(4);
    let two = list.insert_after(one, 2).unwrap();
    list.insert_after(two, 3).unwrap();
    assert_eq!(to_vec(&list), [1, 2, 3, 4]);
    assert_eq!(drain_back(&mut list), [1, 2, 3, 4]);
}

#[test]
fn remove_head_middle_tail_and_only_element() {
    let mut list = DList::new();
    let h: Vec<Handle> = (0..5).map(|i| list.push_back(i)).collect();
    assert_eq!(list.remove(h[2]), Some(2));
    assert_eq!(list.remove(h[0]), Some(0));
    assert_eq!(list.remove(h[4]), Some(4));
    assert_eq!(to_vec(&list), [1, 3]);
    assert_eq!(list.len(), 2);
    assert_eq!(list.remove(h[1]), Some(1));
    assert_eq!(list.remove(h[3]), Some(3));
    assert!(list.is_empty());
    assert_eq!(list.iter().count(), 0);
    list.push_back(9);
    assert_eq!(to_vec(&list), [9]);
}

#[test]
fn removed_handles_are_invalid_until_reused() {
    let mut list = DList::new();
    let a = list.push_back(10);
    let b = list.push_back(20);
    assert_eq!(list.remove(a), Some(10));
    assert_eq!(list.remove(a), None, "removing twice");
    assert_eq!(list.get(a), None);
    assert_eq!(list.get_mut(a), None);
    assert_eq!(list.insert_after(a, 30), Err(30));
    assert_eq!(list.len(), 1);
    assert_eq!(list.pop_front(), Some(20));
    assert_eq!(list.get(b), None, "popped elements' handles are invalid too");
    assert_eq!(list.get(Handle(1_000)), None);
}

#[test]
fn handles_are_stable_across_other_changes() {
    let mut list = DList::new();
    let handles: Vec<Handle> = (0..10).map(|i| list.push_back(i * 10)).collect();
    for &h in handles.iter().step_by(2) {
        list.remove(h);
    }
    list.push_front(-1);
    list.insert_after(handles[5], 55).unwrap();
    for (i, &h) in handles.iter().enumerate().skip(1).step_by(2) {
        assert_eq!(list.get(h), Some(&(i as i32 * 10)), "handle {i}");
    }
    assert_eq!(to_vec(&list), [-1, 10, 30, 50, 55, 70, 90]);
}

#[test]
fn get_mut_edits_in_place() {
    let mut list = DList::new();
    let a = list.push_back(String::from("a"));
    list.push_back(String::from("b"));
    list.get_mut(a).expect("live handle").push_str("aa");
    assert_eq!(to_vec(&list), ["aaa", "b"]);
}

#[test]
fn freed_slots_are_reused() {
    let mut list = DList::new();
    let handles: Vec<Handle> = (0..10).map(|i| list.push_back(i)).collect();
    let removed: HashSet<Handle> = [handles[2], handles[5], handles[9]].into();
    for &h in &removed {
        list.remove(h);
    }
    let fresh: HashSet<Handle> = (0..3).map(|i| list.push_front(100 + i)).collect();
    assert_eq!(fresh, removed, "new elements must take the freed slots");
    let next = list.push_back(200);
    assert!(!handles.contains(&next), "no free slot left: a new one is needed");
}

#[test]
fn arena_does_not_grow_while_the_list_stays_short() {
    // A list that never holds more than 3 elements needs at most 3 slots.
    let mut list = DList::new();
    let mut seen = HashSet::new();
    let mut a = list.push_back(0u32);
    seen.insert(a);
    for i in 1..50_000u32 {
        let b = list.insert_after(a, i).expect("a is live");
        let c = list.push_front(i);
        seen.insert(b);
        seen.insert(c);
        assert_eq!(list.remove(a), Some(i - 1));
        assert_eq!(list.pop_front(), Some(i));
        a = b;
    }
    assert_eq!(list.len(), 1);
    assert!(seen.len() <= 3, "{} distinct slots used", seen.len());
}

#[test]
fn works_without_clone_or_default() {
    let mut list = DList::new();
    let h = list.push_back(Token(1));
    list.push_front(Token(0));
    list.insert_after(h, Token(2)).expect("live handle");
    assert_eq!(list.remove(h), Some(Token(1)));
    assert_eq!(list.pop_back(), Some(Token(2)));
    assert_eq!(list.iter().collect::<Vec<_>>(), [&Token(0)]);
}

#[test]
fn every_element_is_dropped_exactly_once() {
    let drops = Rc::new(Cell::new(0));
    {
        let mut list = DList::new();
        let handles: Vec<Handle> = (0..20)
            .map(|_| list.push_back(Counted(Rc::clone(&drops))))
            .collect();
        drop(list.remove(handles[7]));
        drop(list.pop_front());
        drop(list.pop_back());
        assert_eq!(drops.get(), 3);
        // These reuse freed slots, whose old occupants are already gone.
        list.push_back(Counted(Rc::clone(&drops)));
        assert!(list.insert_after(handles[1], Counted(Rc::clone(&drops))).is_ok());
        assert_eq!(drops.get(), 3, "reusing a slot must not drop anything");
    }
    assert_eq!(drops.get(), 22, "dropping the list drops what's left");
}

#[test]
fn iterator_borrows_and_knows_its_length() {
    let mut list = DList::new();
    for i in 0..5 {
        list.push_front(i);
    }
    let mut it = list.iter();
    assert_eq!(it.next(), Some(&4));
    assert_eq!(it.collect::<Vec<_>>(), [&3, &2, &1, &0]);
    assert_eq!(list.iter().count(), 5);
    assert_eq!(list.len(), 5, "iterating doesn't consume");
}

#[test]
fn rev_walks_the_prev_links() {
    let mut list = DList::new();
    let b = list.push_back('b');
    list.push_back('d');
    list.push_front('a');
    list.insert_after(b, 'c').expect("live handle");
    assert_eq!(list.iter().rev().collect::<String>(), "dcba");
    list.remove(b);
    assert_eq!(list.iter().rev().collect::<String>(), "dca");
    let empty: DList<char> = DList::new();
    assert_eq!(empty.iter().next_back(), None);
}

#[test]
fn both_ends_meet_in_the_middle() {
    let mut list = DList::new();
    for i in 0..5 {
        list.push_back(i);
    }
    let mut it = list.iter();
    assert_eq!(it.next(), Some(&0));
    assert_eq!(it.next_back(), Some(&4));
    assert_eq!(it.next_back(), Some(&3));
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), None, "the front must stop where the back left off");
    assert_eq!(it.next_back(), None);
    let mut it = list.iter();
    assert_eq!(it.next_back(), Some(&4));
    assert_eq!(it.by_ref().count(), 4);
    assert_eq!(it.next_back(), None);
}

#[test]
fn for_loops_borrow_the_list() {
    let list: DList<u32> = {
        let mut l = DList::new();
        for i in 1..=4 {
            l.push_back(i);
        }
        l
    };
    let mut sum = 0;
    for x in &list {
        sum += x;
    }
    assert_eq!(sum, 10);
    assert_eq!(list.len(), 4);
}

#[test]
fn random_operations_match_a_model() {
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    let mut list = DList::new();
    // The model: (handle, value) pairs in list order.
    let mut model: Vec<(Handle, u64)> = Vec::new();
    let mut issued: Vec<Handle> = Vec::new();
    for step in 0..20_000 {
        let n = model.len() as u64;
        match rng.below(9) {
            0 => {
                let v = rng.next();
                let h = list.push_front(v);
                model.insert(0, (h, v));
                issued.push(h);
            }
            1 => {
                let v = rng.next();
                let h = list.push_back(v);
                model.push((h, v));
                issued.push(h);
            }
            2 => {
                let expected = (!model.is_empty()).then(|| model.remove(0).1);
                assert_eq!(list.pop_front(), expected, "step {step}");
            }
            3 => assert_eq!(list.pop_back(), model.pop().map(|p| p.1), "step {step}"),
            4 | 5 if n > 0 => {
                let k = rng.below(n) as usize;
                let v = rng.next();
                let h = list.insert_after(model[k].0, v).expect("live handle");
                model.insert(k + 1, (h, v));
                issued.push(h);
            }
            6 if n > 0 => {
                let k = rng.below(n) as usize;
                let (h, v) = model.remove(k);
                assert_eq!(list.remove(h), Some(v), "step {step}");
            }
            _ if !issued.is_empty() => {
                let h = issued[rng.below(issued.len() as u64) as usize];
                let expected = model.iter().find(|p| p.0 == h).map(|p| &p.1);
                assert_eq!(list.get(h), expected, "step {step}: get({h:?})");
                if expected.is_none() {
                    assert_eq!(list.insert_after(h, 0), Err(0), "step {step}");
                }
            }
            _ => {}
        }
        assert_eq!(list.len(), model.len(), "step {step}");
        if step % 500 == 0 {
            assert!(list.iter().eq(model.iter().map(|p| &p.1)), "step {step}");
            assert!(list.iter().rev().eq(model.iter().rev().map(|p| &p.1)), "step {step}");
        }
    }
    let live: HashSet<Handle> = model.iter().map(|p| p.0).collect();
    assert_eq!(live.len(), model.len(), "live handles are distinct");
    let values: Vec<u64> = model.iter().map(|p| p.1).collect();
    assert_eq!(drain_back(&mut list), values);
}

#[test]
fn many_inserts_and_removes_in_the_middle() {
    // Two rounds (the second reusing the freed slots) of 14_000 inserts after
    // a random live element, then removal of every element in random order:
    // 56_000 operations. Finding a handle by walking the list, or keeping the
    // order in a Vec that is searched and shifted, costs ~len / 2 per
    // operation: 10^8 to 2 * 10^8 steps in all, against one step each with links.
    let n = 14_000u32;
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let mut list = DList::new();
    for round in 0..2 {
        let mut handles = vec![(list.push_back(0u32), 0)];
        for i in 1..n {
            let (at, _) = handles[rng.below(u64::from(i)) as usize];
            handles.push((list.insert_after(at, i).expect("handle is live"), i));
        }
        assert_eq!(list.len(), n as usize, "round {round}");
        for i in (1..handles.len()).rev() {
            handles.swap(i, rng.below(i as u64 + 1) as usize);
        }
        for &(h, i) in &handles {
            assert_eq!(list.remove(h), Some(i), "round {round}, n = {n}");
        }
        assert!(list.is_empty(), "round {round}");
    }
}
