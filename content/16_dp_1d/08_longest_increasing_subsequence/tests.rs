// Tests for Longest Increasing Subsequence — shared by template.rs, solution.rs and your attempt.
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

    /// Uniform in `-m..=m`.
    fn signed(&mut self, m: i32) -> i32 {
        let span = 2 * u64::from(m.unsigned_abs()) + 1;
        i32::try_from(self.next() % span).expect("small") - m
    }
}

/// Brute force: check every subset (as a subsequence) for strict increase.
fn brute_lis<T: Ord>(items: &[T]) -> usize {
    let n = items.len();
    (0u32..1 << n)
        .filter(|mask| {
            let picked: Vec<&T> = (0..n).filter(|&i| mask >> i & 1 == 1).map(|i| &items[i]).collect();
            picked.windows(2).all(|w| w[0] < w[1])
        })
        .map(|mask| mask.count_ones() as usize)
        .max()
        .unwrap_or(0)
}

/// O(n^2) DP oracle.
fn dp_lis<T: Ord>(items: &[T]) -> usize {
    let mut lis = vec![1usize; items.len()];
    for i in 0..items.len() {
        for j in 0..i {
            if items[j] < items[i] {
                lis[i] = lis[i].max(lis[j] + 1);
            }
        }
    }
    lis.into_iter().max().unwrap_or(0)
}

#[test]
fn example_1() {
    assert_eq!(length_of_lis(&[10, 9, 2, 5, 3, 7, 101, 18]), 4);
}

#[test]
fn example_2() {
    assert_eq!(length_of_lis(&[0, 1, 0, 3, 2, 3]), 4);
}

#[test]
fn example_3() {
    assert_eq!(length_of_lis(&[7, 7, 7, 7, 7, 7, 7]), 1);
}

#[test]
fn empty() {
    assert_eq!(length_of_lis::<i32>(&[]), 0);
}

#[test]
fn single_element() {
    assert_eq!(length_of_lis(&[-10_000]), 1);
}

#[test]
fn strictly_increasing() {
    assert_eq!(length_of_lis(&[1, 2, 3, 4, 5]), 5);
}

#[test]
fn strictly_decreasing() {
    assert_eq!(length_of_lis(&[5, 4, 3, 2, 1]), 1);
}

#[test]
fn duplicates_are_not_increasing() {
    // Trap: using `<=` (non-decreasing) or lower/upper bound mix-ups.
    assert_eq!(length_of_lis(&[1, 2, 2, 3, 3, 3]), 3);
    assert_eq!(length_of_lis(&[4, 10, 4, 3, 8, 9]), 3);
}

#[test]
fn answer_is_not_the_longest_increasing_run() {
    // Contiguous runs have length 2 at most; the subsequence 1,2,3,4 has 4.
    assert_eq!(length_of_lis(&[1, 0, 2, 0, 3, 0, 4]), 4);
}

#[test]
fn greedy_from_first_element_fails() {
    // Starting at 100 gives 2; the best skips it.
    assert_eq!(length_of_lis(&[100, 1, 2, 3, 101]), 4);
}

#[test]
fn negatives_and_extremes() {
    assert_eq!(length_of_lis(&[i32::MIN, i32::MAX, -5, 0, 5, i32::MAX]), 5);
    assert_eq!(length_of_lis(&[i64::MAX, i64::MIN]), 1);
}

#[test]
fn best_ends_before_last_element() {
    assert_eq!(length_of_lis(&[3, 4, 5, 6, 1, 2]), 4);
}

#[test]
fn generic_over_strings_and_chars() {
    // Lexicographic order: apple < fig < kiwi < pear.
    assert_eq!(length_of_lis(&["pear", "apple", "fig", "kiwi"]), 3);
    assert_eq!(length_of_lis(&["b", "a", "b", "a"]), 2);
    let chars: Vec<char> = "gauntlet".chars().collect();
    // a < n < t (or a < l < t, a < u < …): length 3.
    assert_eq!(length_of_lis(&chars), 3);
}

#[test]
fn generic_over_tuples() {
    // Tuples compare lexicographically: (1, 5) < (2, 0) < (2, 1).
    assert_eq!(length_of_lis(&[(2, 1), (1, 5), (2, 0), (2, 1), (2, 1)]), 3);
}

#[test]
fn works_without_clone() {
    // Ord but deliberately neither Copy nor Clone.
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    struct Height(u32);
    let items = [Height(3), Height(1), Height(4), Height(1), Height(5), Height(9)];
    assert_eq!(length_of_lis(&items), 4);
}

#[test]
fn reverse_ordering_gives_longest_decreasing() {
    use std::cmp::Reverse;
    let items: Vec<Reverse<i32>> = [5, 1, 4, 3, 2, 6].into_iter().map(Reverse).collect();
    assert_eq!(length_of_lis(&items), 4); // 5, 4, 3, 2
}

#[test]
fn random_small_match_brute_force() {
    let mut rng = Rng(0x0F0F_F0F0_1234_4321);
    for _ in 0..400 {
        let n = (rng.next() % 15) as usize;
        let m = if rng.next() & 1 == 0 { 4 } else { 10_000 };
        let items: Vec<i32> = (0..n).map(|_| rng.signed(m)).collect();
        assert_eq!(length_of_lis(&items), brute_lis(&items), "items = {items:?}");
    }
}

#[test]
fn large_sorted_and_reverse_sorted() {
    let up: Vec<i32> = (0..5000).collect();
    assert_eq!(length_of_lis(&up), 5000);
    let down: Vec<i32> = up.into_iter().rev().collect();
    assert_eq!(length_of_lis(&down), 1);
}

#[test]
fn large_sawtooth() {
    // 70 blocks of 70 increasing values, each block entirely below the
    // previous one: the best is a single block.
    let items: Vec<i32> = (0..70)
        .flat_map(|b| (0..70).map(move |i| 1_000_000 - b * 1_000 + i))
        .collect();
    assert_eq!(length_of_lis(&items), 70);
    // 70 increasing blocks of 70 decreasing values: one per block.
    let items: Vec<i32> = (0..70)
        .flat_map(|b| (0..70).map(move |i| b * 1_000 + (69 - i)))
        .collect();
    assert_eq!(length_of_lis(&items), 70);
}

#[test]
fn large_random_matches_quadratic_dp() {
    let mut rng = Rng(271828);
    let items: Vec<i32> = (0..2500).map(|_| rng.signed(10_000)).collect();
    assert_eq!(length_of_lis(&items), dp_lis(&items));
}

thread_local! {
    static COMPARISONS: Cell<usize> = const { Cell::new(0) };
}

/// An item that counts every comparison made on it (on this thread).
#[derive(Debug)]
struct Counted(i64);

impl PartialEq for Counted {
    fn eq(&self, other: &Self) -> bool {
        COMPARISONS.with(|c| c.set(c.get() + 1));
        self.0 == other.0
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

/// Runs `length_of_lis` and returns its answer and the comparisons it made.
fn counted_lis(items: &[Counted]) -> (usize, usize) {
    let before = COMPARISONS.with(Cell::get);
    let len = length_of_lis(items);
    (len, COMPARISONS.with(Cell::get) - before)
}

#[test]
fn comparisons_stay_within_n_log_n() {
    // Patience sorting with a binary search makes ~n log2 n comparisons (a
    // BTreeSet of tails, ~5x that); the O(n^2) DP makes n^2 / 2 = 2 * 10^8.
    let n: i64 = 20_000;
    let budget = 8 * n as usize * (n.ilog2() as usize + 1);
    // Even positions increase; odd positions are all larger and decrease.
    let items: Vec<Counted> = (0..n)
        .map(|i| Counted(if i % 2 == 0 { i } else { 10_000_000 - i }))
        .collect();
    let (len, calls) = counted_lis(&items);
    // All 10_000 evens, then at most one odd after the last even.
    assert_eq!(len, 10_001, "interleaved, n = {n}");
    assert!(calls <= budget, "{calls} comparisons for interleaved n = {n} (budget {budget})");
    let mut rng = Rng(0x5EED_11D5);
    let items: Vec<Counted> = (0..n).map(|_| Counted(i64::from(rng.signed(1_000_000)))).collect();
    let plain: Vec<i64> = items.iter().map(|c| c.0).collect();
    let (len, calls) = counted_lis(&items);
    assert_eq!(len, length_of_lis(&plain), "random, n = {n}");
    assert!(calls <= budget, "{calls} comparisons for random n = {n} (budget {budget})");
}
