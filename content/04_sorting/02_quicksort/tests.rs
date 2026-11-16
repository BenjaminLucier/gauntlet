// Tests for Quicksort — shared by template.rs, solution.rs and your attempt.
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

thread_local! {
    static COMPARISONS: Cell<usize> = const { Cell::new(0) };
    static BUDGET: Cell<(usize, &'static str)> = const { Cell::new((usize::MAX, "")) };
}

/// A `u32` that counts every comparison made on it (per test thread), and
/// panics as soon as the count passes the budget `sort_within_budget` set, so
/// a quadratic sort fails at once instead of running to the timeout (or
/// overflowing the stack).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Counted(u32);

impl Ord for Counted {
    fn cmp(&self, other: &Self) -> Ordering {
        let calls = COMPARISONS.with(|c| {
            c.set(c.get() + 1);
            c.get()
        });
        let (budget, input) = BUDGET.with(Cell::get);
        assert!(calls <= budget, "more than {budget} comparisons on {input}");
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Counted {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Quicksorts `values` as `Counted`, failing once it makes more than `budget`
/// comparisons, and checks the result.
fn sort_within_budget(values: &[u32], budget: usize, input: &'static str) {
    let mut v: Vec<Counted> = values.iter().map(|&x| Counted(x)).collect();
    COMPARISONS.with(|c| c.set(0));
    BUDGET.with(|b| b.set((budget, input)));
    quicksort(&mut v);
    BUDGET.with(|b| b.set((usize::MAX, "")));
    let got: Vec<u32> = v.iter().map(|c| c.0).collect();
    assert!(got == brute_sorted(values), "{input}: result is not sorted");
}

fn log2_ceil(n: usize) -> usize {
    (usize::BITS - n.saturating_sub(1).leading_zeros()) as usize
}

/// Checks `partition3`'s contract against the original contents.
fn check_partition(original: &[i32], pivot_index: usize) {
    let p = original[pivot_index];
    let mut v = original.to_vec();
    let (lt, gt) = partition3(&mut v, pivot_index);
    let ctx = format!("partition3({original:?}, {pivot_index}) = ({lt}, {gt}) -> {v:?}");
    assert!(lt < gt && gt <= v.len(), "{ctx}");
    assert!(v[..lt].iter().all(|&x| x < p), "{ctx}");
    assert!(v[lt..gt].iter().all(|&x| x == p), "{ctx}");
    assert!(v[gt..].iter().all(|&x| x > p), "{ctx}");
    assert_eq!(brute_sorted(&v), brute_sorted(original), "elements changed: {ctx}");
}

#[test]
fn readme_example() {
    let mut v = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    quicksort(&mut v);
    assert_eq!(v, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    let mut w = [5, 2, 8, 5, 1, 5, 9];
    let (lt, gt) = partition3(&mut w, 0);
    assert_eq!((lt, gt), (2, 5));
    assert!(w[..lt].iter().all(|&x| x < 5));
    assert_eq!(w[lt..gt], [5, 5, 5]);
    assert!(w[gt..].iter().all(|&x| x > 5));
    let mut words = vec!["kiwi", "apple", "fig"];
    quicksort(&mut words);
    assert_eq!(words, ["apple", "fig", "kiwi"]);
}

#[test]
fn quicksort_empty_single_and_pair() {
    let mut e: [u8; 0] = [];
    quicksort(&mut e);
    let mut one = ['x'];
    quicksort(&mut one);
    assert_eq!(one, ['x']);
    let mut two = [2, 1];
    quicksort(&mut two);
    assert_eq!(two, [1, 2]);
}

#[test]
fn partition3_single_element() {
    let mut v = [7];
    assert_eq!(partition3(&mut v, 0), (0, 1));
}

#[test]
fn partition3_pivot_is_min_max_or_middle() {
    let original = [4, 9, 1, 7, 1, 9, 3];
    for i in 0..original.len() {
        check_partition(&original, i);
    }
    let mut v = original;
    assert_eq!(partition3(&mut v, 2), (0, 2), "pivot 1 is the minimum");
    let mut v = original;
    assert_eq!(partition3(&mut v, 1), (5, 7), "pivot 9 is the maximum");
}

#[test]
fn partition3_all_equal_is_one_block() {
    let mut v = [3; 10];
    assert_eq!(partition3(&mut v, 6), (0, 10));
}

#[test]
#[should_panic(expected = "out of bounds")]
fn partition3_pivot_out_of_bounds_panics() {
    let mut v = [1, 2, 3];
    partition3(&mut v, 3);
}

#[test]
#[should_panic(expected = "out of bounds")]
fn partition3_on_empty_slice_panics() {
    let mut v: [i32; 0] = [];
    partition3(&mut v, 0);
}

#[test]
fn partition3_random_cross_check() {
    let mut rng = Rng(0xA5A5_5A5A_1234_4321);
    for round in 0..300 {
        let n = 1 + rng.below(40) as usize;
        let range = [2, 5, 1000][round % 3];
        let original: Vec<i32> = (0..n).map(|_| rng.below(range) as i32 - 2).collect();
        check_partition(&original, rng.below(n as u64) as usize);
    }
}

#[test]
fn partition3_makes_one_comparison_per_element() {
    let mut rng = Rng(0x1357_9BDF_2468_ACE0);
    let mut v: Vec<Counted> = (0..10_000).map(|_| Counted(rng.below(100) as u32)).collect();
    COMPARISONS.with(|c| c.set(0));
    let (lt, gt) = partition3(&mut v, 1234);
    let calls = COMPARISONS.with(Cell::get);
    assert!(
        calls <= v.len(),
        "{calls} comparisons to partition 10000 elements: one `cmp` per element is enough"
    );
    assert!(v[..lt].iter().all(|x| x.0 < v[lt].0) && v[gt..].iter().all(|x| x.0 > v[lt].0));
}

#[test]
fn sorted_and_reversed_inputs() {
    let mut a: Vec<i32> = (0..1000).collect();
    quicksort(&mut a);
    assert_eq!(a, (0..1000).collect::<Vec<_>>());
    let mut b: Vec<i32> = (0..1000).rev().collect();
    quicksort(&mut b);
    assert_eq!(b, (0..1000).collect::<Vec<_>>());
}

#[test]
fn negatives_extremes_and_strings() {
    let mut v = [0, i64::MAX, -1, i64::MIN, i64::MIN, 1, i64::MAX];
    quicksort(&mut v);
    assert_eq!(v, [i64::MIN, i64::MIN, -1, 0, 1, i64::MAX, i64::MAX]);
    let mut s: Vec<String> = ["b", "", "B", "ab", "a"].map(String::from).to_vec();
    quicksort(&mut s);
    assert_eq!(s, ["", "B", "a", "ab", "b"]);
}

#[test]
fn sorts_only_the_sub_slice() {
    let mut v = [9, 8, 7, 6, 5, 4];
    quicksort(&mut v[1..5]);
    assert_eq!(v, [9, 5, 6, 7, 8, 4]);
}

#[test]
fn random_cross_check() {
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    for round in 0..300 {
        let n = rng.below(120) as usize;
        let range = [2, 10, 1 << 32][round % 3];
        let input: Vec<i64> = (0..n).map(|_| rng.below(range) as i64 - (range / 2) as i64).collect();
        let mut v = input.clone();
        quicksort(&mut v);
        assert_eq!(v, brute_sorted(&input), "quicksort({input:?})");
    }
}

#[test]
fn all_equal_input_is_linear() {
    // A 2-way partition makes ~n^2/2 comparisons here; 3-way makes one pass.
    let n = 200_000;
    sort_within_budget(&vec![42; n], 2 * n + 10, "200000 equal elements");
}

#[test]
fn few_distinct_values_are_linear() {
    let mut rng = Rng(0x0F0F_F0F0_1111_2222);
    let n = 200_000;
    let values: Vec<u32> = (0..n).map(|_| rng.below(3) as u32).collect();
    sort_within_budget(&values, 10 * n, "200000 elements with 3 distinct values");
}

#[test]
fn sorted_input_is_not_quadratic() {
    // A first-element pivot makes ~n^2/2 comparisons and n stack frames here.
    let n = 200_000;
    let values: Vec<u32> = (0..n as u32).collect();
    sort_within_budget(&values, 3 * n * log2_ceil(n), "200000 sorted elements");
}

#[test]
fn reversed_input_is_not_quadratic() {
    let n = 200_000;
    let values: Vec<u32> = (0..n as u32).rev().collect();
    sort_within_budget(&values, 3 * n * log2_ceil(n), "200000 reversed elements");
}

#[test]
fn large_random_input() {
    let mut rng = Rng(0x1234_5678_9ABC_DEF1);
    let n = 200_000;
    let values: Vec<u32> = (0..n).map(|_| rng.next() as u32).collect();
    sort_within_budget(&values, 3 * n * log2_ceil(n), "200000 random elements");
}
