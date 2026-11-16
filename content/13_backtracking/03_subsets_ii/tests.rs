// Tests for Subsets II — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::collections::BTreeSet;

/// Sort each subset, then the list. Duplicates are kept so they fail.
fn normalize<T: Ord>(mut v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    for s in v.iter_mut() {
        s.sort();
    }
    v.sort();
    v
}

/// Bitmask over index subsets, deduplicated with a set.
fn brute_subsets_with_dup(nums: &[i32]) -> Vec<Vec<i32>> {
    let n = nums.len();
    let mut set = BTreeSet::new();
    for mask in 0..1u32 << n {
        let mut v: Vec<i32> = (0..n).filter(|&i| mask >> i & 1 == 1).map(|i| nums[i]).collect();
        v.sort_unstable();
        set.insert(v);
    }
    set.into_iter().collect()
}

fn check(nums: &[i32]) {
    let got = normalize(subsets_with_dup(nums));
    let exp = brute_subsets_with_dup(nums);
    assert_eq!(got.len(), exp.len(), "wrong number of subsets (duplicates?) for nums={nums:?}");
    assert_eq!(got, exp, "nums={nums:?}");
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

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

#[test]
fn example_1() {
    assert_eq!(
        normalize(subsets_with_dup(&[1, 2, 2])),
        vec![vec![], vec![1], vec![1, 2], vec![1, 2, 2], vec![2], vec![2, 2]]
    );
}

#[test]
fn example_2() {
    assert_eq!(normalize(subsets_with_dup(&[0])), vec![vec![], vec![0]]);
}

#[test]
fn empty_input_has_only_the_empty_subset() {
    assert_eq!(subsets_with_dup::<i32>(&[]), vec![Vec::<i32>::new()]);
}

#[test]
fn all_distinct_is_plain_power_set() {
    assert_eq!(subsets_with_dup(&[3, 1, 2]).len(), 8);
    check(&[3, 1, 2]);
}

#[test]
fn all_equal() {
    let got = normalize(subsets_with_dup(&[5; 4]));
    assert_eq!(got, vec![vec![], vec![5], vec![5, 5], vec![5, 5, 5], vec![5, 5, 5, 5]]);
}

#[test]
fn all_equal_max_size() {
    assert_eq!(subsets_with_dup(&[-10; 10]).len(), 11);
    check(&[-10; 10]);
}

#[test]
fn unsorted_duplicates_trap() {
    // Without sorting, [4,1] and [1,4] are produced as different subsets.
    check(&[4, 4, 4, 1, 4]);
    check(&[2, 1, 2, 1]);
}

#[test]
fn two_groups_of_duplicates() {
    // (2+1) * (3+1) = 12 distinct subsets.
    assert_eq!(subsets_with_dup(&[1, 1, 2, 2, 2]).len(), 12);
    check(&[1, 1, 2, 2, 2]);
}

#[test]
fn negatives_and_zero() {
    check(&[0, -1, 0, -1, 10]);
    check(&[i32::MIN, i32::MAX, i32::MIN]);
}

#[test]
fn randomized_against_bitmask() {
    let mut rng = Rng(0x0755_0755);
    for _ in 0..300 {
        let n = rng.below(11) as usize;
        let hi = rng.below(5) as i32;
        let nums: Vec<i32> = (0..n).map(|_| rng.below(2 * hi as u64 + 1) as i32 - hi).collect();
        check(&nums);
    }
}

#[test]
fn max_size_distinct() {
    let nums: Vec<i32> = (-5..5).rev().collect();
    assert_eq!(subsets_with_dup(&nums).len(), 1024);
    check(&nums);
}

#[test]
fn single_pair_of_duplicates() {
    assert_eq!(normalize(subsets_with_dup(&[3, 3])), vec![vec![], vec![3], vec![3, 3]]);
}

#[test]
fn count_is_product_of_multiplicities_plus_one() {
    // 1 x2, 2 x3, 3 x1, 4 x4 -> 3 * 4 * 2 * 5 = 120
    let nums = [4, 2, 1, 4, 3, 2, 4, 1, 2, 4];
    assert_eq!(subsets_with_dup(&nums).len(), 120);
    check(&nums);
}

#[test]
fn input_is_not_modified() {
    let nums = vec![2, 1, 2];
    let _ = subsets_with_dup(&nums);
    assert_eq!(nums, vec![2, 1, 2]);
}

#[test]
fn generic_over_strings() {
    let got = normalize(subsets_with_dup(&["b", "a", "b"]));
    assert_eq!(
        got,
        vec![vec![], vec!["a"], vec!["a", "b"], vec!["a", "b", "b"], vec!["b"], vec!["b", "b"]]
    );
    let owned: Vec<String> = ["ñ", "ñ", "ü"].iter().map(|s| s.to_string()).collect();
    assert_eq!(subsets_with_dup(&owned).len(), 6);
}

#[test]
fn generic_over_tuples() {
    // (1, 'x') twice and (1, 'y') once: 3 * 2 = 6 subsets.
    let got = normalize(subsets_with_dup(&[(1, 'x'), (1, 'y'), (1, 'x')]));
    assert_eq!(got.len(), 6);
    assert!(got.contains(&vec![(1, 'x'), (1, 'x'), (1, 'y')]), "got={got:?}");
    for w in got.windows(2) {
        assert_ne!(w[0], w[1], "duplicate subset {:?}", w[0]);
    }
}
