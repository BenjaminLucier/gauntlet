// Tests for Stack — shared by template.rs, solution.rs and your attempt.
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
    let mut s = Stack::new();
    assert!(s.is_empty());
    s.push(1);
    s.push(2);
    s.push(3);
    assert_eq!(s.peek(), Some(&3));
    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.len(), 2);
    if let Some(top) = s.peek_mut() {
        *top *= 10;
    }
    assert_eq!(s.pop(), Some(20));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
    let s: Stack<char> = "abc".chars().collect();
    assert_eq!(s.into_iter().collect::<String>(), "cba");
}

#[test]
fn empty_stack_returns_none_everywhere() {
    let mut s: Stack<i32> = Stack::new();
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);
    assert_eq!(s.peek(), None);
    assert_eq!(s.peek_mut(), None);
    assert_eq!(s.pop(), None);
    assert_eq!(s.pop(), None, "popping twice on empty is still fine");
    assert_eq!(s.into_iter().next(), None);
}

#[test]
fn default_is_empty() {
    let s: Stack<String> = Stack::default();
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);
}

#[test]
fn single_element() {
    let mut s = Stack::new();
    s.push("only");
    assert_eq!(s.len(), 1);
    assert!(!s.is_empty());
    assert_eq!(s.peek(), Some(&"only"));
    assert_eq!(s.pop(), Some("only"));
    assert!(s.is_empty());
}

#[test]
fn last_in_first_out() {
    let mut s = Stack::new();
    for i in 0..10 {
        s.push(i);
    }
    for i in (0..10).rev() {
        assert_eq!(s.peek(), Some(&i));
        assert_eq!(s.pop(), Some(i));
    }
    assert_eq!(s.pop(), None);
}

#[test]
fn peek_does_not_remove() {
    let mut s = Stack::new();
    s.push(5);
    for _ in 0..3 {
        assert_eq!(s.peek(), Some(&5));
    }
    assert_eq!(s.len(), 1);
}

#[test]
fn peek_mut_changes_only_the_top() {
    let mut s: Stack<Vec<i32>> = Stack::new();
    s.push(vec![1]);
    s.push(vec![2]);
    s.peek_mut().expect("non-empty").push(3);
    assert_eq!(s.pop(), Some(vec![2, 3]));
    assert_eq!(s.pop(), Some(vec![1]));
}

#[test]
fn interleaved_pushes_and_pops() {
    let mut s = Stack::new();
    s.push('a');
    s.push('b');
    assert_eq!(s.pop(), Some('b'));
    s.push('c');
    s.push('d');
    assert_eq!(s.pop(), Some('d'));
    assert_eq!(s.pop(), Some('c'));
    s.push('e');
    assert_eq!(s.pop(), Some('e'));
    assert_eq!(s.pop(), Some('a'));
    assert!(s.is_empty());
}

#[test]
fn collect_puts_the_last_item_on_top() {
    let s: Stack<i32> = (1..=5).collect();
    assert_eq!(s.len(), 5);
    assert_eq!(s.peek(), Some(&5));
    let empty: Stack<i32> = std::iter::empty().collect();
    assert!(empty.is_empty());
}

#[test]
fn into_iter_pops_top_first() {
    let mut s: Stack<i32> = (1..=4).collect();
    s.push(10);
    assert_eq!(s.into_iter().collect::<Vec<_>>(), [10, 4, 3, 2, 1]);
}

#[test]
fn for_loop_consumes_the_stack() {
    let s: Stack<String> = ["x", "y", "z"].into_iter().map(String::from).collect();
    let mut seen = String::new();
    for word in s {
        seen.push_str(&word);
    }
    assert_eq!(seen, "zyx");
}

#[test]
fn iter_borrows_top_first_without_consuming() {
    let mut s: Stack<i32> = (1..=4).collect();
    assert_eq!(s.iter().copied().collect::<Vec<_>>(), [4, 3, 2, 1]);
    // Still intact, and still usable afterwards.
    assert_eq!(s.len(), 4);
    s.push(5);
    let mut seen = Vec::new();
    for x in &s {
        seen.push(*x);
    }
    assert_eq!(seen, [5, 4, 3, 2, 1]);
    assert_eq!(s.pop(), Some(5));
    let empty: Stack<i32> = Stack::new();
    assert_eq!(empty.iter().next(), None);
}

#[test]
fn collect_then_into_iter_reverses() {
    let input = [3, 1, 4, 1, 5, 9, 2, 6];
    let reversed: Vec<i32> = input.into_iter().collect::<Stack<_>>().into_iter().collect();
    let mut expected = input.to_vec();
    expected.reverse();
    assert_eq!(reversed, expected);
}

#[test]
fn works_without_clone_or_default() {
    let mut s = Stack::new();
    for i in 0..5 {
        s.push(Token(i));
    }
    s.peek_mut().expect("non-empty").0 = 40;
    assert_eq!(s.pop(), Some(Token(40)));
    assert_eq!(s.peek(), Some(&Token(3)));
    assert_eq!(s.into_iter().last(), Some(Token(0)));
}

#[test]
fn every_element_is_dropped_exactly_once() {
    let drops = Rc::new(Cell::new(0));
    {
        let mut s = Stack::new();
        for _ in 0..20 {
            s.push(Counted(Rc::clone(&drops)));
        }
        drop(s.pop());
        assert_eq!(drops.get(), 1);
        let mut it = s.into_iter();
        drop(it.next());
        drop(it.next());
        assert_eq!(drops.get(), 3);
        // `it` still owns 17 elements.
    }
    assert_eq!(drops.get(), 20, "dropping a half-used iterator drops the rest");
}

#[test]
fn random_operations_match_vec() {
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    let mut s = Stack::new();
    let mut model: Vec<u64> = Vec::new();
    for step in 0..20_000 {
        match rng.below(5) {
            0 | 1 => {
                let v = rng.next();
                s.push(v);
                model.push(v);
            }
            2 => assert_eq!(s.pop(), model.pop(), "step {step}"),
            3 => assert_eq!(s.peek(), model.last(), "step {step}"),
            _ => {
                if let Some(top) = s.peek_mut() {
                    *top = top.wrapping_add(1);
                }
                if let Some(top) = model.last_mut() {
                    *top = top.wrapping_add(1);
                }
            }
        }
        assert_eq!(s.len(), model.len(), "step {step}");
        assert_eq!(s.is_empty(), model.is_empty(), "step {step}");
    }
    model.reverse();
    assert_eq!(s.into_iter().collect::<Vec<_>>(), model);
}

#[test]
fn many_pushes_then_pop_everything() {
    // Pushing and popping at the front of a Vec instead of the back shifts
    // every element each time: about 5 * 10^9 moves of 32 bytes here, against
    // 1.4 * 10^5 at the back.
    let n = 70_000u64;
    let item = |i: u64| [i, !i, i ^ 0x5555, i.wrapping_mul(3)];
    let mut s = Stack::new();
    for i in 0..n {
        s.push(item(i));
    }
    assert_eq!(s.len(), n as usize, "n = {n}");
    assert_eq!(s.peek(), Some(&item(n - 1)), "n = {n}");
    for i in (0..n).rev() {
        assert_eq!(s.pop(), Some(item(i)), "n = {n}");
    }
    assert!(s.is_empty());
}
