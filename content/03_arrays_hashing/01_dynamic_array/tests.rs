// Tests for Dynamic Array — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::cell::Cell;
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

fn to_vec<T: Clone>(a: &DynArray<T>) -> Vec<T> {
    a.iter().cloned().collect()
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
    let mut a = DynArray::new();
    assert!(a.is_empty());
    a.push(10);
    a.push(20);
    a.push(30);
    assert_eq!(a.len(), 3);
    assert_eq!(a[1], 20);
    assert_eq!(a.get(3), None);
    assert!(a.capacity() >= 3);
    a.insert(0, 5).unwrap();
    assert_eq!(a.insert(9, 99), Err(99));
    assert_eq!(a.remove(2), Some(20));
    assert_eq!(a.pop(), Some(30));
    a[1] += 1;
    assert_eq!(to_vec(&a), [5, 11]);
    let b: DynArray<String> = ["x", "y"].into_iter().map(String::from).collect();
    assert_eq!(b.len(), 2);
}

#[test]
fn new_is_empty() {
    let mut a: DynArray<i32> = DynArray::new();
    assert_eq!(a.len(), 0);
    assert!(a.is_empty());
    assert_eq!(a.pop(), None);
    assert_eq!(a.get(0), None);
    assert_eq!(a.remove(0), None);
    assert_eq!(a.iter().next(), None);
    let d: DynArray<i32> = DynArray::default();
    assert!(d.is_empty());
}

#[test]
fn push_then_pop_is_last_in_first_out() {
    let mut a = DynArray::new();
    for i in 0..10 {
        a.push(i);
    }
    for i in (0..10).rev() {
        assert_eq!(a.pop(), Some(i));
    }
    assert_eq!(a.pop(), None);
    assert!(a.is_empty());
}

#[test]
fn get_and_get_mut_past_len_are_none() {
    let mut a = DynArray::with_capacity(8);
    a.push('a');
    a.push('b');
    assert_eq!(a.get(1), Some(&'b'));
    assert_eq!(a.get(2), None, "slot 2 exists in the buffer but isn't in use");
    assert_eq!(a.get_mut(7), None);
    assert_eq!(a.get(usize::MAX), None);
    *a.get_mut(0).unwrap() = 'z';
    assert_eq!(a[0], 'z');
}

#[test]
#[should_panic(expected = "out of bounds")]
fn index_past_len_panics() {
    let mut a = DynArray::with_capacity(8);
    a.push(1);
    let _ = a[1];
}

#[test]
#[should_panic(expected = "out of bounds")]
fn index_mut_past_len_panics() {
    let mut a = DynArray::new();
    a.push(1);
    a[3] = 5;
}

#[test]
fn insert_at_front_middle_and_end() {
    let mut a: DynArray<i32> = [2, 4].into_iter().collect();
    a.insert(0, 1).unwrap();
    a.insert(2, 3).unwrap();
    a.insert(4, 5).unwrap(); // index == len appends
    assert_eq!(to_vec(&a), [1, 2, 3, 4, 5]);
}

#[test]
fn insert_past_len_hands_the_value_back() {
    let mut a = DynArray::new();
    a.push(Token(1));
    assert_eq!(a.insert(2, Token(7)), Err(Token(7)));
    assert_eq!(a.len(), 1);
}

#[test]
fn insert_into_empty_and_full_buffers() {
    let mut a = DynArray::new();
    a.insert(0, 'x').unwrap();
    assert_eq!(to_vec(&a), ['x']);
    let mut b = DynArray::with_capacity(2);
    b.push(1);
    b.push(2);
    assert_eq!(b.capacity(), 2);
    b.insert(1, 9).unwrap(); // must grow first
    assert_eq!(to_vec(&b), [1, 9, 2]);
}

#[test]
fn remove_shifts_later_elements_left() {
    let mut a: DynArray<i32> = (0..6).collect();
    assert_eq!(a.remove(0), Some(0));
    assert_eq!(a.remove(2), Some(3));
    assert_eq!(a.remove(3), Some(5)); // last
    assert_eq!(a.remove(3), None);
    assert_eq!(to_vec(&a), [1, 2, 4]);
}

#[test]
fn clear_drops_everything_and_keeps_capacity() {
    let mut a: DynArray<i32> = (0..100).collect();
    let cap = a.capacity();
    a.clear();
    assert!(a.is_empty());
    assert_eq!(a.capacity(), cap);
    assert_eq!(a.get(0), None);
    a.push(7);
    assert_eq!(to_vec(&a), [7]);
}

#[test]
fn with_capacity_doesnt_grow_until_full() {
    let mut a = DynArray::with_capacity(100);
    let cap = a.capacity();
    assert!(cap >= 100);
    for i in 0..cap {
        a.push(i);
        assert_eq!(a.capacity(), cap, "grew early, at len {}", i + 1);
    }
    a.push(0);
    assert!(a.capacity() > cap);
}

#[test]
fn growth_is_geometric() {
    let mut a = DynArray::new();
    let mut changes = 0;
    let mut cap = a.capacity();
    for i in 0..100_000u32 {
        a.push(i);
        if a.capacity() != cap {
            changes += 1;
            cap = a.capacity();
        }
        assert!(a.capacity() >= a.len());
        // Checked as we go, so growing by a constant fails fast instead of
        // spending minutes copying.
        assert!(changes <= 40, "the buffer was reallocated {changes} times in {} pushes", i + 1);
    }
    assert!(a.capacity() < 4 * a.len(), "capacity {} for len {}", a.capacity(), a.len());
}

#[test]
fn works_without_clone_or_default() {
    let mut a = DynArray::new();
    for i in 0..20 {
        a.push(Token(i));
    }
    assert_eq!(a.remove(5), Some(Token(5)));
    a.insert(0, Token(99)).unwrap();
    assert_eq!(a[0], Token(99));
    assert_eq!(a.pop(), Some(Token(19)));
    assert_eq!(a.len(), 19);
}

#[test]
fn every_element_is_dropped_exactly_once() {
    let drops = Rc::new(Cell::new(0));
    {
        let mut a = DynArray::new();
        for _ in 0..50 {
            a.push(Counted(Rc::clone(&drops)));
        }
        drop(a.pop());
        drop(a.remove(10));
        assert_eq!(drops.get(), 2);
        a.insert(3, Counted(Rc::clone(&drops))).unwrap();
        a.clear();
        assert_eq!(drops.get(), 51, "clear drops the elements");
        for _ in 0..7 {
            a.push(Counted(Rc::clone(&drops)));
        }
    }
    assert_eq!(drops.get(), 58, "dropping the array drops what's left");
}

#[test]
fn for_loops_borrow_the_array() {
    let a: DynArray<u64> = (1..=10).collect();
    let mut sum = 0;
    for x in &a {
        sum += x;
    }
    assert_eq!(sum, 55);
    assert_eq!(a.iter().count(), 10);
    assert_eq!(a.len(), 10, "iterating doesn't consume");
}

#[test]
fn into_iter_moves_the_elements_out_in_order() {
    let a: DynArray<String> = ["a", "b", "c"].into_iter().map(String::from).collect();
    let mut joined = String::new();
    for s in a {
        joined.push_str(&s);
    }
    assert_eq!(joined, "abc");
    let b: DynArray<i32> = (0..100).collect();
    assert_eq!(b.into_iter().collect::<Vec<_>>(), (0..100).collect::<Vec<_>>());
    assert_eq!(DynArray::<u8>::new().into_iter().next(), None);
}

#[test]
fn into_iter_drops_each_element_once_even_when_stopped_early() {
    let drops = Rc::new(Cell::new(0));
    let a: DynArray<Counted> = (0..10).map(|_| Counted(Rc::clone(&drops))).collect();
    {
        let mut it = a.into_iter();
        drop(it.next());
        drop(it.next());
        assert_eq!(drops.get(), 2);
    } // `it` goes out of scope here.
    assert_eq!(drops.get(), 10, "the 8 elements never yielded are dropped with the iterator");
}

#[test]
fn iterators_are_independent() {
    let a: DynArray<i32> = (0..5).collect();
    let mut it1 = a.iter();
    let mut it2 = a.iter();
    assert_eq!(it1.next(), Some(&0));
    assert_eq!(it1.next(), Some(&1));
    assert_eq!(it2.next(), Some(&0));
    assert_eq!(it1.collect::<Vec<_>>(), [&2, &3, &4]);
}

#[test]
fn extend_appends() {
    let mut a: DynArray<String> = DynArray::new();
    a.push("a".to_string());
    a.extend(["b", "c"].map(String::from));
    a.extend(Vec::<String>::new());
    assert_eq!(to_vec(&a), ["a", "b", "c"]);
}

#[test]
fn random_operations_match_vec() {
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    let mut a = DynArray::new();
    let mut model: Vec<u64> = Vec::new();
    for step in 0..20_000 {
        let n = model.len() as u64;
        match rng.below(7) {
            0 | 1 => {
                let v = rng.next();
                a.push(v);
                model.push(v);
            }
            2 => assert_eq!(a.pop(), model.pop(), "step {step}"),
            3 => {
                let i = rng.below(n + 2) as usize;
                let v = rng.next();
                let r = a.insert(i, v);
                if i <= model.len() {
                    model.insert(i, v);
                    assert_eq!(r, Ok(()), "step {step}");
                } else {
                    assert_eq!(r, Err(v), "step {step}");
                }
            }
            4 => {
                let i = rng.below(n + 2) as usize;
                let expected = (i < model.len()).then(|| model.remove(i));
                assert_eq!(a.remove(i), expected, "step {step}: remove({i})");
            }
            5 => {
                let i = rng.below(n + 2) as usize;
                assert_eq!(a.get(i), model.get(i), "step {step}: get({i})");
                if i < model.len() {
                    a[i] = a[i].wrapping_add(1);
                    model[i] = model[i].wrapping_add(1);
                }
            }
            _ => {
                if rng.below(50) == 0 {
                    a.clear();
                    model.clear();
                }
            }
        }
        assert_eq!(a.len(), model.len(), "step {step}");
    }
    assert_eq!(to_vec(&a), model);
}

#[test]
fn popping_does_not_reallocate_every_time() {
    // Shrinking the buffer on every pop moves the whole array each time.
    // Preallocated, so a slow `push` is left to `growth_is_geometric`.
    let mut a = DynArray::with_capacity(300_000);
    for i in 0..300_000u32 {
        a.push(i);
    }
    assert_eq!(a.len(), 300_000);
    assert_eq!(a[299_999], 299_999);
    let mut cap = a.capacity();
    let mut changes = 0;
    let mut total = 0u64;
    while let Some(x) = a.pop() {
        total += u64::from(x);
        if a.capacity() != cap {
            changes += 1;
            cap = a.capacity();
        }
        assert!(changes <= 40, "the buffer was reallocated {changes} times by pops, len now {}", a.len());
    }
    assert_eq!(total, 299_999 * 300_000 / 2);
}
