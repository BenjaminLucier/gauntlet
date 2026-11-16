// Tests for Kth Largest Element in an Array — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::cell::Cell;
use std::cmp::Ordering;

thread_local! {
    static COMPARISONS: Cell<usize> = const { Cell::new(0) };
}

/// A `Copy` + `Ord` wrapper that counts comparisons on this test's thread.
#[derive(Clone, Copy, Debug)]
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

/// Runs `kth_largest` on `items` and returns its answer and comparison count.
fn counted_kth_largest(items: &[i32], k: usize) -> (Option<i32>, usize) {
    let items: Vec<Counted> = items.iter().map(|&x| Counted(x)).collect();
    COMPARISONS.with(|c| c.set(0));
    let kth = kth_largest(&items, k).map(|c| c.0);
    (kth, COMPARISONS.with(Cell::get))
}

/// A budget that admits a heap of size `k` (`O(n log k)`) and even a full
/// sort, but not the `O(n^2)` of a degenerate quickselect.
fn comparison_budget(n: usize) -> usize {
    4 * n * n.ilog2() as usize
}

fn brute_kth_largest<T: Ord + Copy>(items: &[T], k: usize) -> Option<T> {
    let mut sorted = items.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    k.checked_sub(1).and_then(|i| sorted.get(i).copied())
}

/// Tiny deterministic PRNG (xorshift64) so random tests are reproducible.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        lo + (self.next() % (hi - lo + 1) as u64) as i32
    }
}

#[test]
fn example_1() {
    assert_eq!(kth_largest(&[3, 2, 1, 5, 6, 4], 2), Some(5));
}

#[test]
fn example_2() {
    assert_eq!(kth_largest(&[3, 2, 3, 1, 2, 4, 5, 5, 6], 4), Some(4));
}

#[test]
fn example_strings() {
    assert_eq!(kth_largest(&["b", "c", "a"], 1), Some("c"));
}

#[test]
fn example_out_of_range() {
    assert_eq!(kth_largest(&[1, 2], 3), None);
    assert_eq!(kth_largest(&[1, 2], 0), None);
}

#[test]
fn empty_is_none() {
    assert_eq!(kth_largest::<i32>(&[], 0), None);
    assert_eq!(kth_largest::<i32>(&[], 1), None);
}

#[test]
fn huge_k_does_not_underflow() {
    assert_eq!(kth_largest(&[1, 2, 3], usize::MAX), None);
    assert_eq!(kth_largest(&[1, 2, 3], 4), None);
}

#[test]
fn single_element() {
    assert_eq!(kth_largest(&[-7], 1), Some(-7));
}

#[test]
fn k_is_one_is_max() {
    assert_eq!(kth_largest(&[4, 9, -1, 9, 3], 1), Some(9));
}

#[test]
fn k_is_n_is_min() {
    assert_eq!(kth_largest(&[4, 9, -1, 9, 3], 5), Some(-1));
}

#[test]
fn duplicates_count_by_position_not_distinct_value() {
    assert_eq!(kth_largest(&[3, 3, 1], 2), Some(3));
    assert_eq!(kth_largest(&[2, 1, 2, 1, 2], 3), Some(2));
    assert_eq!(kth_largest(&[2, 1, 2, 1, 2], 4), Some(1));
}

#[test]
fn all_equal() {
    assert_eq!(kth_largest(&[5; 7], 4), Some(5));
}

#[test]
fn negatives_and_extremes() {
    assert_eq!(kth_largest(&[-10_000, 10_000, 0, -1], 2), Some(0));
    assert_eq!(kth_largest(&[-10_000, -10_000, -9_999], 1), Some(-9_999));
    assert_eq!(kth_largest(&[i32::MIN, i32::MAX, 0], 3), Some(i32::MIN));
}

#[test]
fn input_is_not_modified() {
    let items = [5, 1, 4, 2, 3];
    assert_eq!(kth_largest(&items, 2), Some(4));
    assert_eq!(items, [5, 1, 4, 2, 3]);
}

#[test]
fn generic_over_chars_and_tuples() {
    let letters: Vec<char> = "gauntlet".chars().collect();
    assert_eq!(kth_largest(&letters, 1), Some('u'));
    assert_eq!(kth_largest(&letters, 8), Some('a'));
    assert_eq!(kth_largest(&[(1, 'z'), (2, 'a'), (1, 'b')], 2), Some((1, 'z')));
}

#[test]
fn sorted_and_reverse_sorted() {
    let asc: Vec<i32> = (1..=100).collect();
    let desc: Vec<i32> = (1..=100).rev().collect();
    for k in [1, 2, 50, 99, 100] {
        let expected = Some(101 - k as i32);
        assert_eq!(kth_largest(&asc, k), expected);
        assert_eq!(kth_largest(&desc, k), expected);
    }
}

#[test]
fn randomized_against_sort() {
    let mut rng = Rng(0x0677_0677);
    for _ in 0..500 {
        let n = rng.range(0, 30) as usize;
        let c = if rng.range(0, 1) == 0 { 3 } else { 10_000 };
        let nums: Vec<i32> = (0..n).map(|_| rng.range(-c, c)).collect();
        let k = rng.range(0, n as i32 + 1) as usize;
        assert_eq!(kth_largest(&nums, k), brute_kth_largest(&nums, k), "nums={nums:?}, k={k}");
    }
}

#[test]
fn large_random() {
    let mut rng = Rng(0x1234_5678_9abc);
    let nums: Vec<i32> = (0..100_000).map(|_| rng.range(-10_000, 10_000)).collect();
    for k in [1, 50_000, 100_000] {
        assert_eq!(kth_largest(&nums, k), brute_kth_largest(&nums, k), "k={k}");
    }
}

#[test]
fn sorted_input_does_not_break_fixed_pivot_quickselect() {
    // A first- or last-element pivot on sorted input peels off one item per
    // partition: ~n^2/8 comparisons to reach the middle.
    let n = 10_000;
    let asc: Vec<i32> = (0..n as i32).collect();
    let desc: Vec<i32> = asc.iter().rev().copied().collect();
    for (name, items) in [("ascending", &asc), ("descending", &desc)] {
        for k in [n / 2, n / 2 + 1] {
            let (kth, count) = counted_kth_largest(items, k);
            assert_eq!(kth, Some((n - k) as i32), "{name} 0..{n}, k={k}");
            let budget = comparison_budget(n);
            assert!(count <= budget, "{count} comparisons (budget {budget}) on {name} 0..{n}, k={k}");
        }
    }
}

#[test]
fn all_equal_input_does_not_break_two_way_partition() {
    // A `<` / `>=` partition puts every equal item on one side, so even a
    // random pivot peels off one item per round: ~n^2/8 comparisons.
    let n = 10_000;
    for k in [1, n / 2, n] {
        let (kth, count) = counted_kth_largest(&vec![42; n], k);
        assert_eq!(kth, Some(42), "{n} equal items, k={k}");
        let budget = comparison_budget(n);
        assert!(count <= budget, "{count} comparisons (budget {budget}) on {n} equal items, k={k}");
    }
}
