// Tests for Top K Frequent Elements — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::cell::Cell;
use std::hash::{Hash, Hasher};

fn check(nums: &[i32], k: usize, expected: &[i32]) {
    let mut got = top_k_frequent(nums, k);
    got.sort_unstable();
    let mut exp = expected.to_vec();
    exp.sort_unstable();
    if nums.len() <= 50 {
        assert_eq!(got, exp, "nums = {nums:?}, k = {k}");
    } else {
        assert_eq!(got, exp, "nums.len() = {}, k = {k}", nums.len());
    }
}

/// (value, count) pairs sorted by count descending — O(n²) counting.
fn brute_counts(nums: &[i32]) -> Vec<(i32, usize)> {
    let mut distinct: Vec<i32> = nums.to_vec();
    distinct.sort_unstable();
    distinct.dedup();
    let mut v: Vec<(i32, usize)> = distinct
        .into_iter()
        .map(|d| (d, nums.iter().filter(|&&x| x == d).count()))
        .collect();
    v.sort_by_key(|p| std::cmp::Reverse(p.1));
    v
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

    fn range(&mut self, lo: i64, hi: i64) -> i64 {
        lo + (self.next() % (hi - lo + 1) as u64) as i64
    }
}

thread_local! {
    static EQ_CALLS: Cell<usize> = const { Cell::new(0) };
}

/// A value that counts every `==` it takes part in.
#[derive(Debug, Clone)]
struct Probe(u32);

impl Hash for Probe {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl PartialEq for Probe {
    fn eq(&self, other: &Self) -> bool {
        EQ_CALLS.with(|c| c.set(c.get() + 1));
        self.0 == other.0
    }
}

impl Eq for Probe {}

#[test]
fn example_1() {
    check(&[1, 1, 1, 2, 2, 3], 2, &[1, 2]);
}

#[test]
fn example_2() {
    check(&[1], 1, &[1]);
}

#[test]
fn example_3() {
    check(&[1, 2, 1, 2, 1, 2, 3, 1, 3, 2], 2, &[1, 2]);
}

#[test]
fn example_words() {
    assert_eq!(
        top_k_frequent(&["to", "be", "or", "not", "to", "be", "to"], 1),
        vec!["to"]
    );
}

#[test]
fn example_k_larger_than_distinct() {
    check(&[7, 7, 8], 5, &[7, 8]);
}

#[test]
fn empty_input() {
    assert!(top_k_frequent::<i32>(&[], 3).is_empty());
    assert!(top_k_frequent::<i32>(&[], 0).is_empty());
}

#[test]
fn k_zero_returns_nothing() {
    assert!(top_k_frequent(&[1, 1, 2], 0).is_empty());
}

#[test]
fn k_equals_distinct_count() {
    check(&[4, 4, 5, 6, 6, 6], 3, &[4, 5, 6]);
}

#[test]
fn k_one_most_frequent_last() {
    check(&[1, 2, 3, 4, 5, 5], 1, &[5]);
}

#[test]
fn negatives_and_zero() {
    check(&[-1, -1, 0, 0, 0, -10_000, 10_000], 2, &[0, -1]);
}

#[test]
fn extreme_values() {
    check(&[-10_000, 10_000, 10_000, -10_000, -10_000], 1, &[-10_000]);
    check(&[i32::MIN, i32::MAX, i32::MAX], 1, &[i32::MAX]);
}

#[test]
fn frequency_not_value_order() {
    // Trap: returning the k largest/smallest *values*.
    check(&[9, 1, 1, 1, 2, 2, 7], 2, &[1, 2]);
}

#[test]
fn unsorted_runs() {
    // Trap: counting only adjacent runs.
    check(&[3, 1, 3, 2, 3, 1, 2, 3], 1, &[3]);
}

#[test]
fn returns_exactly_k_distinct() {
    let got = top_k_frequent(&[5, 5, 5, 6, 6, 7, 8, 9], 2);
    assert_eq!(got.len(), 2, "got {got:?}");
    // Trap: returning a value once per occurrence.
    let got = top_k_frequent(&[3, 3, 3, 3], 1);
    assert_eq!(got, vec![3]);
}

#[test]
fn generic_over_strings_and_tuples() {
    let words: Vec<String> = "a b a c b a".split(' ').map(String::from).collect();
    let mut got = top_k_frequent(&words, 2);
    got.sort_unstable();
    assert_eq!(got, vec!["a".to_string(), "b".to_string()]);

    let points = [(0, 0), (1, 2), (0, 0), (2, 1), (0, 0), (1, 2)];
    let mut got = top_k_frequent(&points, 2);
    got.sort_unstable();
    assert_eq!(got, vec![(0, 0), (1, 2)]);
}

#[test]
fn generic_over_chars() {
    let chars: Vec<char> = "mississippi".chars().collect();
    let mut got = top_k_frequent(&chars, 2);
    got.sort_unstable();
    assert_eq!(got, vec!['i', 's']);
}

#[test]
fn randomized_against_brute() {
    let mut rng = Rng(0xABCD_EF01_2345_6789);
    for _ in 0..300 {
        let n = rng.range(1, 30) as usize;
        let spread = rng.range(1, 8);
        let nums: Vec<i32> = (0..n).map(|_| rng.range(-spread, spread) as i32).collect();
        let counts = brute_counts(&nums);
        // Pick a k whose answer is unique (no tie across the k-th boundary);
        // k beyond the distinct count is fine too.
        let valid_ks: Vec<usize> = (0..=counts.len() + 2)
            .filter(|&k| k == 0 || k >= counts.len() || counts[k - 1].1 != counts[k].1)
            .collect();
        let k = valid_ks[rng.range(0, valid_ks.len() as i64 - 1) as usize];
        let exp: Vec<i32> = counts.iter().take(k).map(|p| p.0).collect();
        check(&nums, k, &exp);
    }
}

#[test]
fn large_input() {
    // Value v in 0..400 appears v+1 times -> ~80k elements; top 10 are 390..399.
    let mut nums = Vec::new();
    for v in 0..400 {
        for _ in 0..=v {
            nums.push(v - 200);
        }
    }
    // Interleave so equal values aren't adjacent.
    let n = nums.len();
    let mut shuffled = Vec::with_capacity(n);
    let step = 7919 % n;
    let mut i = 0;
    for _ in 0..n {
        shuffled.push(nums[i]);
        i = (i + step) % n;
    }
    let exp: Vec<i32> = (390..400).map(|v| v - 200).collect();
    check(&shuffled, 10, &exp);
}

#[test]
fn all_distinct_but_one() {
    let mut nums: Vec<i32> = (-1_000..=1_000).collect();
    nums.extend(std::iter::repeat_n(42, 5_000));
    check(&nums, 1, &[42]);
}

#[test]
fn counting_does_not_scan_the_values() {
    // Counts `==` calls rather than time. A hash map compares each item with
    // about one key; looking it up in a list of (value, count) pairs, or
    // counting each value by filtering the whole slice, compares it with
    // every distinct value.
    // 990 values occur 9 times each; values 1000..1010 occur 20..30 times.
    let mut values: Vec<u32> = (0..990).flat_map(|v| std::iter::repeat_n(v, 9)).collect();
    values.extend((0..10).flat_map(|i| std::iter::repeat_n(1000 + i, 20 + i as usize)));
    let n = values.len();
    let step = 7919 % n;
    let items: Vec<Probe> = (0..n).map(|i| Probe(values[i * step % n])).collect();
    let before = EQ_CALLS.with(Cell::get);
    let got = top_k_frequent(&items, 10);
    let calls = EQ_CALLS.with(Cell::get) - before;
    let mut got: Vec<u32> = got.into_iter().map(|p| p.0).collect();
    got.sort_unstable();
    assert_eq!(got, (1000..1010).collect::<Vec<_>>(), "n = {n}, k = 10");
    assert!(calls <= 3 * n, "{calls} comparisons for n = {n} items, 1000 distinct");
}
