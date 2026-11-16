// Tests for Merge Sort — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::cell::Cell;

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

fn brute_sorted<T: Ord + Clone>(v: &[T]) -> Vec<T> {
    let mut w = v.to_vec();
    w.sort();
    w
}

/// `n * ceil(log2 n)`: the most comparisons a top-down merge sort may make.
fn comparison_budget(n: usize) -> usize {
    if n < 2 { 0 } else { n * (usize::BITS - (n - 1).leading_zeros()) as usize }
}

/// Sorts with `merge_sort_by(.., cmp)` and returns how many comparisons it
/// made. Panics as soon as it passes `comparison_budget`, so a quadratic sort
/// fails at once instead of running to the timeout.
fn counted_sort<T: Ord + Clone>(v: &mut [T]) -> usize {
    let n = v.len();
    let budget = comparison_budget(n);
    let calls = Cell::new(0usize);
    merge_sort_by(v, |a, b| {
        calls.set(calls.get() + 1);
        assert!(calls.get() <= budget, "more than {budget} comparisons for n = {n}");
        a.cmp(b)
    });
    calls.get()
}

/// Pairs sorted by `.0` only; check `.1` (the original position) still
/// increases within each run of equal keys.
fn assert_stably_sorted(v: &[(u32, usize)]) {
    for w in v.windows(2) {
        assert!(w[0] < w[1], "not stably sorted at {w:?}");
    }
}

#[test]
fn readme_example() {
    let mut v = vec![38, 27, 43, 3, 9, 82, 10];
    merge_sort(&mut v);
    assert_eq!(v, [3, 9, 10, 27, 38, 43, 82]);
    let mut people = vec![("ann", 31), ("bob", 25), ("cat", 31), ("dan", 25)];
    merge_sort_by(&mut people, |a, b| a.1.cmp(&b.1));
    assert_eq!(people, [("bob", 25), ("dan", 25), ("ann", 31), ("cat", 31)]);
    let mut words = vec!["a", "ccc", "bb"];
    merge_sort_by(&mut words, |a, b| b.len().cmp(&a.len()));
    assert_eq!(words, ["ccc", "bb", "a"]);
}

#[test]
fn empty_single_and_pair() {
    let mut e: Vec<i32> = vec![];
    merge_sort(&mut e);
    assert!(e.is_empty());
    let mut one = [1];
    merge_sort(&mut one);
    assert_eq!(one, [1]);
    let mut two = [2, 1];
    merge_sort(&mut two);
    assert_eq!(two, [1, 2]);
    let mut three = [3, 1, 2];
    merge_sort(&mut three);
    assert_eq!(three, [1, 2, 3]);
}

#[test]
fn sorted_reversed_and_all_equal() {
    let cases: [Vec<i32>; 3] = [(0..1001).collect(), (0..1001).rev().collect(), vec![4; 1001]];
    for case in cases {
        let mut v = case.clone();
        merge_sort(&mut v);
        assert_eq!(v, brute_sorted(&case));
    }
}

#[test]
fn negatives_and_extremes() {
    let mut v = [0, i32::MAX, -5, i32::MIN, 3, i32::MIN, -1, i32::MAX];
    merge_sort(&mut v);
    assert_eq!(v, [i32::MIN, i32::MIN, -5, -1, 0, 3, i32::MAX, i32::MAX]);
}

#[test]
fn odd_lengths_split_correctly() {
    for n in 0..70 {
        let input: Vec<u32> = (0..n).map(|i| (i * 37 + 11) % 23).collect();
        let mut v = input.clone();
        merge_sort(&mut v);
        assert_eq!(v, brute_sorted(&input), "length {n}: {input:?}");
    }
}

#[test]
fn stable_on_pairs_by_key() {
    let keys = [3, 1, 3, 2, 1, 3, 2, 1, 0, 3];
    let mut v: Vec<(u32, usize)> = keys.iter().enumerate().map(|(i, &k)| (k, i)).collect();
    merge_sort_by(&mut v, |a, b| a.0.cmp(&b.0));
    assert_stably_sorted(&v);
    assert_eq!(v.iter().map(|p| p.1).collect::<Vec<_>>(), [8, 1, 4, 7, 3, 6, 0, 2, 5, 9]);
}

#[test]
fn equal_keys_at_the_merge_boundary_take_left_first() {
    // Left half [ (1,0) (1,1) ], right half [ (1,2) (1,3) ]: every comparison
    // is a tie, and the left run must win each one.
    let mut v = vec![(1u32, 0usize), (1, 1), (1, 2), (1, 3)];
    merge_sort_by(&mut v, |a, b| a.0.cmp(&b.0));
    assert_eq!(v, [(1, 0), (1, 1), (1, 2), (1, 3)]);
    let mut w = vec![(2u32, 0usize), (1, 1), (2, 2), (1, 3)];
    merge_sort_by(&mut w, |a, b| a.0.cmp(&b.0));
    assert_eq!(w, [(1, 1), (1, 3), (2, 0), (2, 2)]);
}

#[test]
fn stable_random_records() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for _ in 0..30 {
        let n = rng.below(300) as usize;
        let mut v: Vec<(u32, usize)> = (0..n).map(|i| (rng.below(8) as u32, i)).collect();
        merge_sort_by(&mut v, |a, b| a.0.cmp(&b.0));
        assert_stably_sorted(&v);
    }
}

#[test]
fn descending_comparator() {
    let mut v: Vec<i32> = (0..100).map(|i| (i * 7) % 31).collect();
    let mut expected = brute_sorted(&v);
    expected.reverse();
    merge_sort_by(&mut v, |a, b| b.cmp(a));
    assert_eq!(v, expected);
}

#[test]
fn strings_and_sub_slices() {
    let mut s: Vec<String> = ["pear", "Fig", "apple", "", "fig"].map(String::from).to_vec();
    merge_sort(&mut s);
    assert_eq!(s, ["", "Fig", "apple", "fig", "pear"]);
    let mut v = [9, 8, 7, 6, 5, 4];
    merge_sort(&mut v[1..5]);
    assert_eq!(v, [9, 5, 6, 7, 8, 4], "only the sub-slice is touched");
}

#[test]
fn comparisons_stay_within_n_log_n() {
    let mut rng = Rng(0x0BAD_C0DE_1234_5678);
    for n in [1usize, 2, 3, 7, 64, 100, 1000, 4097] {
        let mut v: Vec<u64> = (0..n).map(|_| rng.next()).collect();
        let calls = counted_sort(&mut v);
        assert!(calls <= comparison_budget(n), "{calls} comparisons for n = {n}");
        assert!(v.windows(2).all(|w| w[0] <= w[1]));
    }
}

#[test]
fn reversed_input_is_not_quadratic() {
    // Insertion sort would make ~n^2/2 = 2 * 10^9 comparisons here.
    let n = 65_536;
    let mut v: Vec<u32> = (0..n as u32).rev().collect();
    let calls = counted_sort(&mut v);
    assert!(calls <= comparison_budget(n), "{calls} comparisons for n = {n}");
    assert!(v.iter().enumerate().all(|(i, &x)| x == i as u32));
}

thread_local! {
    static CLONES: Cell<usize> = const { Cell::new(0) };
}

/// An element that counts its clones (on this thread).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Tracked(u64);

impl Clone for Tracked {
    fn clone(&self) -> Self {
        CLONES.with(|c| c.set(c.get() + 1));
        Tracked(self.0)
    }
}

#[test]
fn scratch_buffer_is_cloned_once() {
    // One buffer of n / 2 clones, allocated up front, is all merge sort
    // needs. Cloning a fresh buffer for every merge costs ~(n / 2) log2 n.
    let mut rng = Rng(0x5EED_0FC1_0E5A_1234);
    for n in [0usize, 1, 2, 3, 10, 1000, 20_000] {
        let mut v: Vec<Tracked> = (0..n).map(|_| Tracked(rng.below(1000))).collect();
        let before = CLONES.with(Cell::get);
        merge_sort(&mut v);
        let clones = CLONES.with(Cell::get) - before;
        assert!(
            clones <= n,
            "{clones} clones to sort {n} elements: allocate the scratch buffer once, not per merge"
        );
        assert!(v.windows(2).all(|w| w[0] <= w[1]), "n = {n}: not sorted");
    }
}

#[test]
fn random_cross_check() {
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    for round in 0..300 {
        let n = rng.below(100) as usize;
        let range = [2, 10, 1 << 32][round % 3];
        let input: Vec<i64> = (0..n).map(|_| rng.below(range) as i64 - (range / 2) as i64).collect();
        let mut v = input.clone();
        merge_sort(&mut v);
        assert_eq!(v, brute_sorted(&input), "merge_sort({input:?})");
    }
}

#[test]
fn large_random_input_is_stable_and_fast() {
    let mut rng = Rng(0x1234_5678_9ABC_DEF1);
    let n = 200_000;
    let mut v: Vec<(u32, usize)> = (0..n).map(|i| (rng.below(1000) as u32, i)).collect();
    let budget = comparison_budget(n);
    let calls = Cell::new(0usize);
    merge_sort_by(&mut v, |a, b| {
        calls.set(calls.get() + 1);
        assert!(calls.get() <= budget, "more than {budget} comparisons for n = {n}");
        a.0.cmp(&b.0)
    });
    assert_stably_sorted(&v);
}

#[test]
fn random_input_with_ord() {
    let mut rng = Rng(0xFEED_FACE_DEAD_BEEF);
    let input: Vec<u64> = (0..10_000).map(|_| rng.next()).collect();
    let mut v = input.clone();
    merge_sort(&mut v);
    assert_eq!(v, brute_sorted(&input));
}
