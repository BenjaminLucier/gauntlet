// Tests for Combination Sum II — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::collections::{BTreeMap, BTreeSet};

/// Sort each combination, then the list. Duplicates are kept so they fail.
fn normalize(mut v: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    for s in v.iter_mut() {
        s.sort_unstable();
    }
    v.sort();
    v
}

/// Bitmask over all index subsets, deduplicated with a set.
fn brute_combination_sum_ii(c: &[u32], target: u32) -> Vec<Vec<u32>> {
    let n = c.len();
    let mut set = BTreeSet::new();
    for mask in 0..1u32 << n {
        let mut v: Vec<u32> = (0..n).filter(|&i| mask >> i & 1 == 1).map(|i| c[i]).collect();
        if v.iter().sum::<u32>() == target {
            v.sort_unstable();
            set.insert(v);
        }
    }
    set.into_iter().collect()
}

fn frequencies(c: &[u32]) -> BTreeMap<u32, usize> {
    let mut freq = BTreeMap::new();
    for &x in c {
        *freq.entry(x).or_default() += 1;
    }
    freq
}

/// Number of distinct multisets (respecting multiplicities) summing to target.
fn count_combinations(c: &[u32], target: u32) -> u64 {
    let t = target as usize;
    let mut ways = vec![0u64; t + 1];
    ways[0] = 1;
    for (&v, &cnt) in &frequencies(c) {
        let v = v as usize;
        let mut next = vec![0u64; t + 1];
        for s in 0..=t {
            for k in 0..=cnt {
                if s + k * v > t {
                    break;
                }
                next[s + k * v] += ways[s];
            }
        }
        ways = next;
    }
    ways[t]
}

/// Validates without enumerating: every combo is valid, none repeat, count matches.
fn validate(c: &[u32], target: u32, got: Vec<Vec<u32>>) {
    let got = normalize(got);
    let freq = frequencies(c);
    for combo in &got {
        assert_eq!(combo.iter().sum::<u32>(), target, "{combo:?} does not sum to {target}");
        for (x, k) in frequencies(combo) {
            assert!(
                k <= freq.get(&x).copied().unwrap_or(0),
                "{combo:?} uses {x} more often than available"
            );
        }
    }
    for w in got.windows(2) {
        assert_ne!(w[0], w[1], "duplicate combination {:?}", w[0]);
    }
    assert_eq!(got.len() as u64, count_combinations(c, target), "wrong number of combinations");
}

fn check(c: &[u32], target: u32) {
    let got = normalize(combination_sum_ii(c, target));
    assert_eq!(got, brute_combination_sum_ii(c, target), "candidates={c:?}, target={target}");
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

    /// Uniform-ish value in `lo..=hi`.
    fn range(&mut self, lo: u32, hi: u32) -> u32 {
        lo + (self.next() % u64::from(hi - lo + 1)) as u32
    }
}

#[test]
fn example_1() {
    assert_eq!(
        normalize(combination_sum_ii(&[10, 1, 2, 7, 6, 1, 5], 8)),
        vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
    );
}

#[test]
fn example_2() {
    assert_eq!(normalize(combination_sum_ii(&[2, 5, 2, 1, 2], 5)), vec![vec![1, 2, 2], vec![5]]);
}

#[test]
fn single_candidate_hit_and_miss() {
    assert_eq!(normalize(combination_sum_ii(&[7], 7)), vec![vec![7]]);
    assert!(combination_sum_ii(&[7], 3).is_empty());
}

#[test]
fn each_element_used_at_most_once() {
    // Reusing the single 2 would give [2,2].
    assert!(combination_sum_ii(&[2], 4).is_empty());
    assert_eq!(normalize(combination_sum_ii(&[2, 2], 4)), vec![vec![2, 2]]);
}

#[test]
fn duplicate_values_yield_one_combination() {
    assert_eq!(normalize(combination_sum_ii(&[1, 1, 2], 3)), vec![vec![1, 2]]);
    assert_eq!(normalize(combination_sum_ii(&[1, 1, 1, 1, 1], 2)), vec![vec![1, 1]]);
}

#[test]
fn duplicates_scattered_in_input() {
    check(&[3, 1, 3, 5, 1, 1], 8);
    check(&[4, 1, 1, 4, 4, 4, 1, 2, 2], 9);
}

#[test]
fn no_solution() {
    assert!(combination_sum_ii(&[5, 10, 20], 4).is_empty());
    assert!(combination_sum_ii(&[2, 4, 6], 13).is_empty());
}

#[test]
fn unsigned_underflow_trap() {
    // Subtracting before comparing panics on u32 underflow.
    assert!(combination_sum_ii(&[50, 40], 1).is_empty());
    assert_eq!(normalize(combination_sum_ii(&[50, 1, 49], 50)), vec![vec![1, 49], vec![50]]);
}

#[test]
fn whole_array_is_the_answer() {
    assert_eq!(normalize(combination_sum_ii(&[3, 1, 2, 4], 10)), vec![vec![1, 2, 3, 4]]);
}

#[test]
fn input_is_not_modified() {
    let c = vec![10, 1, 2, 7, 6, 1, 5];
    let _ = combination_sum_ii(&c, 8);
    assert_eq!(c, vec![10, 1, 2, 7, 6, 1, 5]);
}

#[test]
fn randomized_against_bitmask() {
    let mut rng = Rng(0x0733_0733);
    for _ in 0..300 {
        let n = rng.range(1, 12) as usize;
        let hi = rng.range(2, 10);
        let c: Vec<u32> = (0..n).map(|_| rng.range(1, hi)).collect();
        let target = rng.range(1, 30);
        check(&c, target);
    }
}

#[test]
fn large_hundred_ones() {
    // Generating index subsets then deduping is C(100, 30) ≈ 3 * 10^25 subsets:
    // exponential, so it never finishes.
    assert_eq!(combination_sum_ii(&[1; 100], 30), vec![vec![1; 30]]);
}

#[test]
fn large_ones_and_twos() {
    let mut c = vec![1; 50];
    c.extend([2; 50]);
    let got = combination_sum_ii(&c, 30);
    assert_eq!(got.len(), 16);
    validate(&c, 30, got);
}

#[test]
fn large_every_value_twice() {
    let c: Vec<u32> = (1..=50).chain(1..=50).collect();
    let got = combination_sum_ii(&c, 30);
    validate(&c, 30, got);
}

#[test]
fn large_random_max_size() {
    let mut rng = Rng(0xbead_0073);
    for _ in 0..3 {
        let c: Vec<u32> = (0..100).map(|_| rng.range(1, 50)).collect();
        let got = combination_sum_ii(&c, 30);
        validate(&c, 30, got);
    }
}
