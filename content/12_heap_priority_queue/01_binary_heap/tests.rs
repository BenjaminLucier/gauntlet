// Tests for Binary Heap — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::cell::Cell;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

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

fn drain<T: Ord>(h: &mut MaxHeap<T>) -> Vec<T> {
    std::iter::from_fn(|| h.pop()).collect()
}

/// Asserts the array layout is a max-heap: every parent >= its children.
fn assert_heap_order<T: Ord + std::fmt::Debug>(v: &[T]) {
    for i in 1..v.len() {
        let parent = (i - 1) / 2;
        assert!(v[parent] >= v[i], "v[{parent}] = {:?} < v[{i}] = {:?}", v[parent], v[i]);
    }
}

fn sorted<T: Ord>(mut v: Vec<T>) -> Vec<T> {
    v.sort_unstable();
    v
}

/// An `Ord` wrapper that counts comparisons.
struct Counted<'a>(u32, &'a Cell<usize>);

impl PartialEq for Counted<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Counted<'_> {}
impl PartialOrd for Counted<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Counted<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.set(self.1.get() + 1);
        self.0.cmp(&other.0)
    }
}

#[test]
fn readme_examples() {
    let mut h = MaxHeap::new();
    h.push(3);
    h.push(10);
    h.push(7);
    assert_eq!(h.peek(), Some(&10));
    assert_eq!(h.pop(), Some(10));
    assert_eq!(h.pop(), Some(7));
    assert_eq!(h.len(), 1);
    let mut h = MaxHeap::from(vec![4, 1, 8, 1, 5]);
    assert_eq!(h.peek(), Some(&8));
    assert_eq!(h.iter().count(), 5);
    h.extend([9, 2]);
    assert_eq!(h.pop(), Some(9));
    let v = h.into_vec();
    assert_eq!(v.len(), 6);
    assert_eq!(v[0], 8);
    let mut min: MaxHeap<Reverse<i32>> = [5, -2, 9].into_iter().map(Reverse).collect();
    assert_eq!(min.pop(), Some(Reverse(-2)));
}

#[test]
fn empty_heap() {
    let mut h: MaxHeap<i32> = MaxHeap::new();
    assert!(h.is_empty());
    assert_eq!(h.len(), 0);
    assert_eq!(h.peek(), None);
    assert_eq!(h.pop(), None);
    assert_eq!(h.iter().next(), None);
    assert!(h.into_vec().is_empty());
    let d: MaxHeap<String> = MaxHeap::default();
    assert!(d.is_empty());
    assert!(MaxHeap::<u8>::from(Vec::new()).is_empty());
}

#[test]
fn single_item() {
    let mut h = MaxHeap::new();
    h.push('x');
    assert_eq!(h.peek(), Some(&'x'));
    assert_eq!(h.len(), 1);
    assert_eq!(h.pop(), Some('x'));
    assert_eq!(h.pop(), None);
    assert!(h.is_empty());
}

#[test]
fn peek_does_not_remove() {
    let mut h: MaxHeap<i32> = [2, 9, 4].into_iter().collect();
    assert_eq!(h.peek(), Some(&9));
    assert_eq!(h.peek(), Some(&9));
    assert_eq!(h.len(), 3);
    h.push(11);
    assert_eq!(h.peek(), Some(&11));
}

#[test]
fn pops_in_descending_order() {
    let mut h = MaxHeap::new();
    for x in [5, 1, 8, 3, 9, 2, 7, 4, 6, 0] {
        h.push(x);
    }
    assert_eq!(drain(&mut h), [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
}

#[test]
fn duplicates_and_negatives() {
    let mut h = MaxHeap::from(vec![3, -1, 3, 3, -7, 0, -1]);
    assert_eq!(drain(&mut h), [3, 3, 3, 0, -1, -1, -7]);
}

#[test]
fn missing_right_child_trap() {
    // Sizes 2 and 3 leave a node with only a left child after a pop.
    for input in [vec![1, 2], vec![2, 1], vec![1, 2, 3], vec![3, 1, 2], vec![2, 3, 1, 4]] {
        let mut expected = input.clone();
        expected.sort_unstable_by(|a, b| b.cmp(a));
        let mut h = MaxHeap::new();
        for &x in &input {
            h.push(x);
        }
        assert_eq!(drain(&mut h), expected, "pushed {input:?}");
        assert_eq!(drain(&mut MaxHeap::from(input.clone())), expected, "from {input:?}");
    }
}

#[test]
fn sift_down_must_pick_the_larger_child() {
    // After popping 10, the new root must swap with 9, not 8.
    let mut h = MaxHeap::from(vec![10, 8, 9, 1, 2, 3, 4]);
    assert_eq!(h.pop(), Some(10));
    assert_eq!(h.peek(), Some(&9));
    assert_eq!(drain(&mut h), [9, 8, 4, 3, 2, 1]);
}

#[test]
fn iter_sees_every_item_in_heap_order() {
    let mut h: MaxHeap<i64> = [7, -3, 12, 0, 7, 5].into_iter().collect();
    h.push(1);
    let seen: Vec<i64> = h.iter().copied().collect();
    assert_eq!(seen.first(), Some(&12), "array order starts at the root");
    assert_heap_order(&seen);
    assert_eq!(sorted(seen), [-3, 0, 1, 5, 7, 7, 12]);
    let mut total = 0;
    for x in &h {
        total += x;
    }
    assert_eq!(total, 29);
    assert_eq!(h.len(), 7, "iterating doesn't consume");
    assert_eq!(h.pop(), Some(12));
}

#[test]
fn from_vec_and_into_vec_reuse_the_buffer() {
    let mut v = Vec::with_capacity(64);
    v.extend([5, 3, 8, 1, 9, 2]);
    let ptr = v.as_ptr();
    let mut h = MaxHeap::from(v);
    h.push(4);
    assert_eq!(h.pop(), Some(9));
    let back = h.into_vec();
    assert_eq!(back.as_ptr(), ptr, "the same allocation, no copying");
    assert_heap_order(&back);
    assert_eq!(sorted(back), [1, 2, 3, 4, 5, 8]);
}

#[test]
fn extend_pushes_everything() {
    let mut h: MaxHeap<u32> = MaxHeap::new();
    h.extend([3, 1, 4]);
    h.extend(Vec::new());
    h.extend((10..13).rev());
    assert_eq!(h.len(), 6);
    assert_heap_order(&h.iter().copied().collect::<Vec<_>>());
    assert_eq!(drain(&mut h), [12, 11, 10, 4, 3, 1]);
}

#[test]
fn reverse_makes_a_min_heap() {
    let mut h: MaxHeap<Reverse<i32>> = MaxHeap::new();
    for x in [4, -10, 7, 0, 3] {
        h.push(Reverse(x));
    }
    assert_eq!(h.peek(), Some(&Reverse(-10)));
    let order: Vec<i32> = std::iter::from_fn(|| h.pop()).map(|Reverse(x)| x).collect();
    assert_eq!(order, [-10, 0, 3, 4, 7]);
    let mut h = MaxHeap::from(vec![Reverse(1), Reverse(3), Reverse(2)]);
    assert_eq!(drain(&mut h), [Reverse(1), Reverse(2), Reverse(3)], "smallest inner value first");
}

#[test]
fn works_without_clone() {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Task {
        priority: u8,
        name: String,
    }
    let task = |priority, name: &str| Task { priority, name: name.to_string() };
    let mut h: MaxHeap<Task> = [task(2, "b"), task(5, "e"), task(1, "a")].into_iter().collect();
    h.push(task(5, "f"));
    assert_eq!(h.pop().map(|t| t.name), Some("f".to_string()));
    assert_eq!(h.peek().map(|t| t.priority), Some(5));
    let names: Vec<String> = drain(&mut h).into_iter().map(|t| t.name).collect();
    assert_eq!(names, ["e", "b", "a"]);
}

#[test]
fn collect_from_any_iterator() {
    let h: MaxHeap<u32> = (1..=100).filter(|x| x % 7 == 0).collect();
    assert_eq!(h.len(), 14);
    assert_eq!(h.peek(), Some(&98));
    let mut words: MaxHeap<&str> = "the quick brown fox".split(' ').collect();
    assert_eq!(drain(&mut words), ["the", "quick", "fox", "brown"]);
}

#[test]
fn heapify_is_linear_not_n_pushes() {
    // Ascending input is the worst case for n pushes: every push sifts all the
    // way to the root, ~n log n comparisons. Bottom-up heapify stays under 2n.
    let count = Cell::new(0);
    let n = 1 << 16;
    let items: Vec<Counted> = (0..n as u32).map(|i| Counted(i, &count)).collect();
    let mut h = MaxHeap::from(items);
    assert!(count.get() <= 2 * n, "{} comparisons to heapify {n} items", count.get());
    assert_eq!(h.len(), n);
    assert_eq!(h.pop().map(|c| c.0), Some(n as u32 - 1));
    assert_eq!(h.pop().map(|c| c.0), Some(n as u32 - 2));
    // `collect` is O(n) the same way, not a loop of pushes.
    count.set(0);
    let mut h: MaxHeap<Counted> = (0..n as u32).map(|i| Counted(i, &count)).collect();
    assert!(count.get() <= 2 * n, "{} comparisons to collect {n} items", count.get());
    assert_eq!(h.pop().map(|c| c.0), Some(n as u32 - 1));
}

#[test]
fn pop_is_logarithmic() {
    let count = Cell::new(0);
    let n = 1 << 16;
    let mut h: MaxHeap<Counted> = (0..n as u32).map(|i| Counted(i * 7 % n as u32, &count)).collect();
    for _ in 0..100 {
        count.set(0);
        h.pop();
        assert!(count.get() <= 40, "{} comparisons for one pop", count.get());
        count.set(0);
        h.push(Counted(n as u32 + 1, &count));
        assert!(count.get() <= 20, "{} comparisons for one push", count.get());
    }
}

#[test]
fn random_operations_match_std() {
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    let mut h = MaxHeap::new();
    let mut model = BinaryHeap::new();
    for step in 0..20_000 {
        match rng.below(5) {
            0..=2 => {
                let x = rng.below(100) as i32 - 50;
                h.push(x);
                model.push(x);
            }
            3 => assert_eq!(h.pop(), model.pop(), "step {step}"),
            _ => assert_eq!(h.peek(), model.peek(), "step {step}"),
        }
        assert_eq!(h.len(), model.len(), "step {step}");
    }
    assert_eq!(sorted(h.into_vec()), model.into_sorted_vec());
}

#[test]
fn random_heapify_gives_a_valid_heap() {
    let mut rng = Rng(0x0123_4567_89AB_CDEF);
    for _ in 0..300 {
        let n = rng.below(40) as usize;
        let input: Vec<u8> = (0..n).map(|_| rng.below(10) as u8).collect();
        let heap = MaxHeap::from(input.clone()).into_vec();
        assert_heap_order(&heap);
        assert_eq!(sorted(heap), sorted(input.clone()), "input = {input:?}");
    }
}

#[test]
fn large_push_pop() {
    let mut rng = Rng(0xFEED_FACE_CAFE_BEEF);
    let n = 20_000;
    let mut h = MaxHeap::new();
    for _ in 0..n {
        h.push(rng.next());
    }
    let mut prev = u64::MAX;
    let mut popped = 0;
    while let Some(x) = h.pop() {
        assert!(x <= prev, "popped {x} after {prev}");
        prev = x;
        popped += 1;
    }
    assert_eq!(popped, n);
}

#[test]
fn push_and_pop_fat_items() {
    // 6 * 10^4 pushes, then as many pops, of 64-byte items. A sorted array kept
    // with `Vec::insert` / `Vec::remove(0)` (valid heap order, O(n) per
    // operation) moves ~2.7 * 10^9 items here; a heap swaps ~2 * 10^6 times.
    // The comparison budgets can't tell the two apart.
    let mut rng = Rng(0xDEAD_BEEF_0BAD_F00D);
    let n = 60_000;
    let mut h: MaxHeap<[u64; 8]> = MaxHeap::new();
    for _ in 0..n {
        let mut item = [0; 8];
        item[0] = rng.next();
        h.push(item);
    }
    let mut prev = [u64::MAX; 8];
    let mut popped = 0;
    while let Some(x) = h.pop() {
        assert!(x <= prev, "popped {x:?} after {prev:?}");
        prev = x;
        popped += 1;
    }
    assert_eq!(popped, n);
}
