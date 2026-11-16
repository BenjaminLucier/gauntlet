// Tests for Binary Search & Bounds — shared by template.rs, solution.rs and your attempt.
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

    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        lo + self.below((hi - lo + 1) as u64) as i32
    }
}

fn brute_lower<T: Ord>(items: &[T], target: &T) -> usize {
    items.iter().position(|x| x >= target).unwrap_or(items.len())
}

fn brute_upper<T: Ord>(items: &[T], target: &T) -> usize {
    items.iter().position(|x| x > target).unwrap_or(items.len())
}

fn brute_search<T: Ord>(items: &[T], target: &T) -> Option<usize> {
    items.iter().position(|x| x == target)
}

/// Checks every function against the brute-force versions.
fn check_all(items: &[i32], target: i32) {
    let ctx = format!("items = {items:?}, target = {target}");
    let lo = brute_lower(items, &target);
    let hi = brute_upper(items, &target);
    assert_eq!(lower_bound(items, &target), lo, "lower_bound: {ctx}");
    assert_eq!(upper_bound(items, &target), hi, "upper_bound: {ctx}");
    assert_eq!(equal_range(items, &target), lo..hi, "equal_range: {ctx}");
    assert_eq!(binary_search(items, &target), brute_search(items, &target), "binary_search: {ctx}");
    assert_eq!(partition_point(items, |&x| x < target), lo, "partition_point: {ctx}");
}

/// An `Ord` wrapper that counts comparisons and panics if there are too many —
/// so a loop that never shrinks its window fails instead of hanging.
struct Counted<'a>(i64, &'a Cell<usize>);

const MAX_COMPARISONS: usize = 200;

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
        let count = self.1.get();
        assert!(count <= MAX_COMPARISONS, "over {MAX_COMPARISONS} comparisons: a linear scan or an infinite loop?");
        self.0.cmp(&other.0)
    }
}

#[test]
fn readme_examples() {
    let v = [1, 2, 2, 2, 5, 8];
    assert_eq!(lower_bound(&v, &2), 1);
    assert_eq!(upper_bound(&v, &2), 4);
    assert_eq!(equal_range(&v, &2), 1..4);
    assert_eq!(equal_range(&v, &3), 4..4);
    assert_eq!(lower_bound(&v, &0), 0);
    assert_eq!(upper_bound(&v, &9), 6);
    assert_eq!(binary_search(&v, &2), Some(1));
    assert_eq!(binary_search(&v, &7), None);
    assert_eq!(partition_point(&v, |&x| x < 5), 4);
    assert_eq!(binary_search(&["ant", "bee", "cat"], &"cat"), Some(2));
}

#[test]
fn empty_slice() {
    let e: [i32; 0] = [];
    assert_eq!(lower_bound(&e, &7), 0);
    assert_eq!(upper_bound(&e, &7), 0);
    assert_eq!(equal_range(&e, &7), 0..0);
    assert_eq!(binary_search(&e, &7), None);
    assert_eq!(partition_point(&e, |_| true), 0);
}

#[test]
fn single_element_below_at_and_above() {
    // Below the only element: a closed window's `hi = mid - 1` underflows.
    for t in [4, 5, 6] {
        check_all(&[5], t);
    }
    assert_eq!(equal_range(&[5], &5), 0..1);
}

#[test]
fn two_elements_infinite_loop_trap() {
    // `lo = mid` instead of `lo = mid + 1` never shrinks a window of two.
    for t in 0..=4 {
        check_all(&[1, 3], t);
        check_all(&[2, 2], t);
    }
}

#[test]
fn target_below_and_above_everything() {
    let v = [10, 20, 20, 30];
    assert_eq!(equal_range(&v, &-1_000), 0..0);
    assert_eq!(equal_range(&v, &1_000), 4..4);
    assert_eq!(binary_search(&v, &-1_000), None);
    assert_eq!(binary_search(&v, &1_000), None);
    check_all(&v, i32::MIN);
    check_all(&v, i32::MAX);
}

#[test]
fn all_equal() {
    let v = [7; 9];
    assert_eq!(equal_range(&v, &7), 0..9);
    assert_eq!(binary_search(&v, &7), Some(0), "the first occurrence, not any");
    assert_eq!(equal_range(&v, &6), 0..0);
    assert_eq!(equal_range(&v, &8), 9..9);
}

#[test]
fn duplicates_at_the_ends() {
    let v = [1, 1, 1, 4, 9, 9, 9];
    assert_eq!(equal_range(&v, &1), 0..3);
    assert_eq!(equal_range(&v, &9), 4..7);
    assert_eq!(binary_search(&v, &9), Some(4));
    assert_eq!(equal_range(&v, &4), 3..4);
}

#[test]
fn bounds_are_insertion_points() {
    let v = [2, 4, 4, 6];
    for t in 0..8 {
        let mut w = v.to_vec();
        w.insert(lower_bound(&v, &t), t);
        assert!(w.windows(2).all(|p| p[0] <= p[1]), "lower_bound({t}) breaks order: {w:?}");
        let mut w = v.to_vec();
        w.insert(upper_bound(&v, &t), t);
        assert!(w.windows(2).all(|p| p[0] <= p[1]), "upper_bound({t}) breaks order: {w:?}");
    }
}

#[test]
fn every_position_in_odd_and_even_lengths() {
    for n in 0..20 {
        let v: Vec<i32> = (0..n).map(|i| i * 2).collect();
        for t in -1..=2 * n {
            check_all(&v, t);
        }
    }
}

#[test]
fn extreme_values() {
    let v = [i32::MIN, i32::MIN, -1, 0, i32::MAX];
    for t in [i32::MIN, -1, 0, 1, i32::MAX - 1, i32::MAX] {
        check_all(&v, t);
    }
}

#[test]
fn partition_point_with_any_predicate() {
    let v = [3, 5, 7, 2, 4, 6]; // odd numbers first, then even: partitioned by "is odd"
    assert_eq!(partition_point(&v, |x| x % 2 != 0), 3);
    assert_eq!(partition_point(&v, |_| true), 6);
    assert_eq!(partition_point(&v, |_| false), 0);
    let words = ["fig", "kiwi", "apple", "banana", "cherry"];
    assert_eq!(partition_point(&words, |w| w.len() < 5), 2);
}

#[test]
fn equal_range_slices_the_run() {
    let v = [1, 3, 3, 3, 3, 8];
    assert_eq!(&v[equal_range(&v, &3)], &[3, 3, 3, 3]);
    assert!(v[equal_range(&v, &2)].is_empty());
    assert_eq!(equal_range(&v, &2).start, 1, "an empty range still sits at the insertion point");
}

#[test]
fn generic_over_strings_tuples_and_non_clone_types() {
    let words: Vec<String> = ["apple", "banana", "banana", "date"].map(String::from).to_vec();
    assert_eq!(equal_range(&words, &"banana".to_string()), 1..3);
    assert_eq!(binary_search(&words, &"cherry".to_string()), None);
    let pairs = [(1, 'z'), (2, 'a'), (2, 'b'), (5, 'c')];
    assert_eq!(lower_bound(&pairs, &(2, 'b')), 2);
    assert_eq!(upper_bound(&pairs, &(2, 'z')), 3);

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct Id(u64);
    let ids = [Id(3), Id(10), Id(10), Id(1000)];
    assert_eq!(equal_range(&ids, &Id(10)), 1..3);
    assert_eq!(binary_search(&ids, &Id(11)), None);
}

#[test]
fn partition_point_terminates_on_tiny_slices() {
    // Counts predicate calls; a window that stops shrinking trips the limit.
    let calls = Cell::new(0usize);
    let pred = |x: &i32, cut: i32| {
        calls.set(calls.get() + 1);
        assert!(calls.get() <= 64, "over 64 predicate calls on a tiny slice: infinite loop?");
        *x < cut
    };
    for v in [&[][..], &[1], &[1, 2], &[1, 2, 3]] {
        for cut in 0..=4 {
            calls.set(0);
            let expected = v.iter().filter(|&&x| x < cut).count();
            assert_eq!(partition_point(v, |x| pred(x, cut)), expected, "v = {v:?}, cut = {cut}");
        }
    }
}

#[test]
fn uses_logarithmic_comparisons() {
    let count = Cell::new(0);
    // 100_000 items: each value repeated 4 times, except one run of 50_000
    // copies of 30_000, so expanding outwards from a match is also linear.
    let value = |i: i64| if (20_000..70_000).contains(&i) { 30_000 } else { i / 4 * 2 };
    let items: Vec<Counted> = (0..100_000).map(|i| Counted(value(i), &count)).collect();
    for target in [-5, 0, 1, 30_000, 77_776, 49_998, 49_999, 50_000] {
        let t = Counted(target, &count);
        let first = (0..100_000).filter(|&i| value(i) < target).count();
        let last = (0..100_000).filter(|&i| value(i) <= target).count();
        let in_range = first < last;
        let expected = first..last;
        count.set(0);
        assert_eq!(lower_bound(&items, &t), first, "lower_bound, target = {target}");
        // floor(log2(100_000)) + 1 = 17 comparisons; a linear scan needs thousands.
        assert!(count.get() <= 20, "lower_bound: {} comparisons for target {target}", count.get());
        count.set(0);
        assert_eq!(upper_bound(&items, &t), last, "upper_bound, target = {target}");
        assert!(count.get() <= 20, "upper_bound: {} comparisons for target {target}", count.get());
        count.set(0);
        assert_eq!(equal_range(&items, &t), expected, "target = {target}");
        // Two bounds at ~17 comparisons each.
        assert!(count.get() <= 40, "equal_range: {} comparisons for target {target}", count.get());
        count.set(0);
        assert_eq!(binary_search(&items, &t), in_range.then_some(first), "target = {target}");
        assert!(count.get() <= 22, "binary_search: {} comparisons for target {target}", count.get());
    }
}

#[test]
fn partition_point_calls_the_predicate_logarithmically() {
    // A linear scan in partition_point itself would make ~cut calls here.
    let items: Vec<u32> = (0..100_000).collect();
    for cut in [0, 1, 2, 777, 50_000, 99_998, 99_999, 100_000] {
        let mut calls = 0;
        let got = partition_point(&items, |&x| {
            calls += 1;
            x < cut
        });
        assert_eq!(got, cut as usize, "cut = {cut}");
        // floor(log2(100_000)) + 1 = 17.
        assert!(calls <= 17, "{calls} predicate calls for cut = {cut}");
    }
}

#[test]
fn randomized_vs_brute() {
    let mut rng = Rng(0x2801_aaaa_5555);
    for _ in 0..1_000 {
        let n = rng.below(25) as usize;
        let mut items: Vec<i32> = (0..n).map(|_| rng.range(-10, 10)).collect();
        items.sort_unstable();
        let target = rng.range(-12, 12);
        check_all(&items, target);
    }
}

#[test]
fn large_with_long_runs() {
    let n = 1_000_000;
    let items: Vec<u32> = (0..n).map(|i| i / 1_000).collect();
    for v in (0..1_000).step_by(37) {
        assert_eq!(equal_range(&items, &v), (v * 1_000) as usize..((v + 1) * 1_000) as usize);
        assert_eq!(binary_search(&items, &v), Some((v * 1_000) as usize));
    }
    assert_eq!(equal_range(&items, &1_000), 1_000_000..1_000_000);
    assert_eq!(partition_point(&items, |&x| x < 999), 999_000);
}
