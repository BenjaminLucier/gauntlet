// Tests for Min Stack — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::cell::Cell;
use std::cmp::Ordering;

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

#[test]
fn example_1() {
    let mut s = MinStack::new();
    s.push(-2);
    s.push(0);
    s.push(-3);
    assert_eq!(s.min(), Some(&-3));
    assert_eq!(s.pop(), Some(-3));
    assert_eq!(s.peek(), Some(&0));
    assert_eq!(s.min(), Some(&-2));
    assert_eq!(s.len(), 2);
}

#[test]
fn example_2_strings_via_default() {
    let mut words: MinStack<&str> = MinStack::default();
    assert_eq!(words.min(), None);
    words.push("pear");
    words.push("apple");
    assert_eq!(words.min(), Some(&"apple"));
}

#[test]
fn empty_stack_returns_none() {
    let mut s: MinStack<i32> = MinStack::new();
    assert_eq!(s.peek(), None);
    assert_eq!(s.min(), None);
    assert_eq!(s.pop(), None);
    assert_eq!(s.len(), 0);
    assert!(s.is_empty());
}

#[test]
fn pop_past_empty_stays_empty() {
    let mut s = MinStack::new();
    s.push(1);
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
    assert_eq!(s.pop(), None);
    assert_eq!(s.min(), None);
    s.push(2);
    assert_eq!(s.min(), Some(&2));
    assert_eq!(s.len(), 1);
}

#[test]
fn single_element() {
    let mut s = MinStack::new();
    s.push(42);
    assert_eq!(s.peek(), Some(&42));
    assert_eq!(s.min(), Some(&42));
    assert!(!s.is_empty());
}

#[test]
fn pop_returns_values_in_lifo_order() {
    let mut s = MinStack::new();
    for v in [3, 1, 4, 1, 5] {
        s.push(v);
    }
    let popped: Vec<i32> = std::iter::from_fn(|| s.pop()).collect();
    assert_eq!(popped, vec![5, 1, 4, 1, 3]);
    assert!(s.is_empty());
}

#[test]
fn duplicate_minimum_popped_once_trap() {
    // A "pop only removes the min if it equals it" helper stack that ignores
    // duplicates loses the second 0 here.
    let mut s = MinStack::new();
    s.push(0);
    s.push(1);
    s.push(0);
    assert_eq!(s.min(), Some(&0));
    s.pop();
    assert_eq!(s.min(), Some(&0));
    assert_eq!(s.peek(), Some(&1));
}

#[test]
fn popping_the_min_restores_previous_min() {
    let mut s = MinStack::new();
    s.push(5);
    s.push(3);
    s.push(7);
    s.push(1);
    assert_eq!(s.min(), Some(&1));
    s.pop();
    assert_eq!(s.min(), Some(&3));
    s.pop();
    assert_eq!(s.min(), Some(&3));
    s.pop();
    assert_eq!(s.min(), Some(&5));
    assert_eq!(s.peek(), Some(&5));
}

#[test]
fn increasing_pushes_min_stays_first() {
    let mut s = MinStack::new();
    for v in 1..=10 {
        s.push(v);
        assert_eq!(s.min(), Some(&1));
        assert_eq!(s.peek(), Some(&v));
    }
}

#[test]
fn decreasing_pushes_then_pops() {
    let mut s = MinStack::new();
    for v in (1..=10).rev() {
        s.push(v);
        assert_eq!(s.min(), Some(&v));
    }
    for v in 1..=9 {
        assert_eq!(s.peek(), Some(&v));
        assert_eq!(s.pop(), Some(v));
        assert_eq!(s.min(), Some(&(v + 1)));
    }
}

#[test]
fn extreme_values() {
    let mut s = MinStack::new();
    s.push(i32::MAX);
    assert_eq!(s.min(), Some(&i32::MAX));
    s.push(i32::MIN);
    assert_eq!(s.min(), Some(&i32::MIN));
    assert_eq!(s.peek(), Some(&i32::MIN));
    s.pop();
    assert_eq!(s.min(), Some(&i32::MAX));
    s.push(i32::MIN);
    s.push(i32::MAX);
    assert_eq!(s.min(), Some(&i32::MIN));
    assert_eq!(s.peek(), Some(&i32::MAX));
}

#[test]
fn empty_then_reuse() {
    // Forgetting to reset state when the stack empties out.
    let mut s = MinStack::new();
    s.push(-100);
    s.pop();
    s.push(50);
    assert_eq!(s.min(), Some(&50));
    assert_eq!(s.peek(), Some(&50));
    s.push(60);
    assert_eq!(s.min(), Some(&50));
}

#[test]
fn all_equal_values() {
    let mut s = MinStack::new();
    for _ in 0..5 {
        s.push(7);
    }
    for remaining in (1..5).rev() {
        s.pop();
        assert_eq!(s.min(), Some(&7));
        assert_eq!(s.peek(), Some(&7));
        assert_eq!(s.len(), remaining);
    }
}

#[test]
fn two_independent_instances() {
    let mut a = MinStack::new();
    let mut b = MinStack::new();
    a.push(1);
    b.push(2);
    assert_eq!(a.min(), Some(&1));
    assert_eq!(b.min(), Some(&2));
}

#[test]
fn generic_over_owned_strings() {
    let mut s = MinStack::new();
    for w in ["mango", "kiwi", "zucchini", "banana"] {
        s.push(w.to_string());
    }
    assert_eq!(s.min().map(String::as_str), Some("banana"));
    assert_eq!(s.pop().as_deref(), Some("banana"));
    assert_eq!(s.min().map(String::as_str), Some("kiwi"));
}

#[test]
fn works_without_clone() {
    // Ord but deliberately not Clone or Copy.
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Token(u32);
    let mut s = MinStack::new();
    s.push(Token(5));
    s.push(Token(2));
    s.push(Token(9));
    assert_eq!(s.min(), Some(&Token(2)));
    assert_eq!(s.pop(), Some(Token(9)));
    assert_eq!(s.pop(), Some(Token(2)));
    assert_eq!(s.min(), Some(&Token(5)));
}

#[test]
fn randomized_vs_model() {
    let mut rng = Rng(0x2201_1234_5678);
    for round in 0..40 {
        let mut s = MinStack::new();
        let mut model: Vec<i32> = Vec::new();
        let mut log = Vec::new();
        for _ in 0..200 {
            let op = rng.below(5);
            if op <= 1 {
                let v = rng.below(21) as i32 - 10;
                s.push(v);
                model.push(v);
                log.push(format!("push({v})"));
            } else if op == 2 {
                assert_eq!(s.pop(), model.pop(), "round {round}, ops: {log:?}");
                log.push("pop".to_string());
            }
            assert_eq!(s.peek(), model.last(), "round {round}, ops: {log:?}");
            assert_eq!(s.min(), model.iter().min(), "round {round}, ops: {log:?}");
            assert_eq!(s.len(), model.len(), "round {round}, ops: {log:?}");
            assert_eq!(s.is_empty(), model.is_empty(), "round {round}, ops: {log:?}");
        }
    }
}

#[test]
fn large_min_at_bottom() {
    let mut s = MinStack::new();
    s.push(i32::MIN);
    for i in 0..15_000 {
        s.push(i);
        assert_eq!(s.min(), Some(&i32::MIN));
    }
    for _ in 0..15_000 {
        s.pop();
        assert_eq!(s.min(), Some(&i32::MIN));
    }
    assert_eq!(s.peek(), Some(&i32::MIN));
}

thread_local! {
    static COMPARISONS: Cell<usize> = const { Cell::new(0) };
}

/// An `i32` that counts every comparison made on it (on this thread). Not
/// `Clone`, like any `T: Ord`.
#[derive(Debug)]
struct Counted(i32);

impl PartialEq for Counted {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Counted {}

impl PartialOrd for Counted {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Counted {
    fn cmp(&self, other: &Self) -> Ordering {
        COMPARISONS.with(|c| c.set(c.get() + 1));
        self.0.cmp(&other.0)
    }
}

/// Comparisons made so far on this thread.
fn comparisons() -> usize {
    COMPARISONS.with(Cell::get)
}

#[test]
fn min_does_not_scan_the_stack() {
    // 5000 random pushes then 5000 pops, asking for the min after each.
    // Scanning for it makes about 2.5 * 10^7 comparisons; keeping it costs at
    // most a couple per push or pop.
    let n = 5_000;
    let mut rng = Rng(0x2202_5eed_c0de);
    let values: Vec<i32> = (0..n).map(|_| rng.below(1_000_000) as i32).collect();
    let mut s = MinStack::new();
    let before = comparisons();
    let mut mins = Vec::with_capacity(n);
    for &v in &values {
        s.push(Counted(v));
        mins.push(s.min().map(|c| c.0));
    }
    for _ in 0..n {
        mins.push(s.min().map(|c| c.0));
        s.pop();
    }
    let calls = comparisons() - before;
    assert!(calls <= 4 * n, "{calls} comparisons for {n} pushes and {n} pops");
    // Model: the min of each prefix of `values`.
    let mut prefix_min: Vec<i32> = Vec::with_capacity(n);
    for &v in &values {
        prefix_min.push(prefix_min.last().map_or(v, |&m| m.min(v)));
    }
    let mut expected: Vec<Option<i32>> = prefix_min.iter().copied().map(Some).collect();
    expected.extend(prefix_min.iter().rev().copied().map(Some));
    assert_eq!(mins, expected, "random values, n = {n}");
    assert!(s.is_empty());
}

#[test]
fn popping_the_min_does_not_rescan() {
    // 9000 strictly decreasing pushes, so every pop removes the current min.
    // Remembering only the min and rescanning when it is popped makes about
    // 4 * 10^7 comparisons.
    let n = 9_000;
    let mut s = MinStack::new();
    let before = comparisons();
    for i in 0..n {
        s.push(Counted(n - i));
    }
    let mut mins = Vec::with_capacity(n as usize);
    while let Some(top) = s.pop() {
        mins.push((top.0, s.min().map(|c| c.0)));
    }
    let calls = comparisons() - before;
    assert!(
        calls <= 4 * n as usize,
        "{calls} comparisons for {n} pushes and {n} pops"
    );
    let expected: Vec<(i32, Option<i32>)> = (1..=n)
        .map(|v| (v, (v < n).then_some(v + 1)))
        .collect();
    assert_eq!(mins, expected, "decreasing pushes, n = {n}");
}
