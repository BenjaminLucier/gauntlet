// Tests for 3Sum — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::collections::{BTreeSet, HashMap};

/// Sort the list of triplets (any order is accepted) — but keep duplicates so
/// they're caught. Triplets themselves must already be in non-decreasing order.
fn normalize(mut v: Vec<[i32; 3]>) -> Vec<[i32; 3]> {
    for t in &v {
        assert!(
            t[0] <= t[1] && t[1] <= t[2],
            "triplet {t:?} is not in non-decreasing order"
        );
    }
    v.sort_unstable();
    v
}

fn check(nums: &[i32], expected: &[[i32; 3]]) {
    let got = normalize(three_sum(nums));
    let mut exp = expected.to_vec();
    exp.sort_unstable();
    if nums.len() <= 50 {
        assert_eq!(got, exp, "nums = {nums:?}");
    } else {
        assert_eq!(
            got.len(),
            exp.len(),
            "wrong number of triplets; nums.len() = {}",
            nums.len()
        );
        assert!(got == exp, "wrong triplets; nums.len() = {}", nums.len());
    }
}

/// O(n^3) oracle with a set for deduplication.
fn brute_three_sum(nums: &[i32]) -> Vec<[i32; 3]> {
    let n = nums.len();
    let mut set = BTreeSet::new();
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if nums[i] + nums[j] + nums[k] == 0 {
                    let mut t = [nums[i], nums[j], nums[k]];
                    t.sort_unstable();
                    set.insert(t);
                }
            }
        }
    }
    set.into_iter().collect()
}

/// O(d^2) oracle over distinct values (fast enough for the large test).
fn oracle_by_counts(nums: &[i32]) -> Vec<[i32; 3]> {
    let mut count: HashMap<i32, usize> = HashMap::new();
    for &x in nums {
        *count.entry(x).or_insert(0) += 1;
    }
    let mut d: Vec<i32> = count.keys().copied().collect();
    d.sort_unstable();
    let mut out = Vec::new();
    for (ai, &a) in d.iter().enumerate() {
        for &b in &d[ai..] {
            let c = -(a + b);
            if c < b {
                break;
            }
            let Some(&cc) = count.get(&c) else { continue };
            let need_b = if b == a { 2 } else { 1 };
            let need_c = 1 + usize::from(c == a) + usize::from(c == b);
            if count[&b] >= need_b && cc >= need_c {
                out.push([a, b, c]);
            }
        }
    }
    out
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

#[test]
fn example_1() {
    check(&[-1, 0, 1, 2, -1, -4], &[[-1, -1, 2], [-1, 0, 1]]);
}

#[test]
fn example_2() {
    check(&[0, 1, 1], &[]);
}

#[test]
fn example_3() {
    check(&[0, 0, 0], &[[0, 0, 0]]);
    check(&[0, 0, 0, 0], &[[0, 0, 0]]);
}

#[test]
fn fewer_than_three_values() {
    // Trap: `n - 1` / `n - 2` underflow on short input.
    check(&[], &[]);
    check(&[0], &[]);
    check(&[1, -1], &[]);
    check(&[0, 0], &[]);
}

#[test]
fn many_zeros_one_triplet() {
    // Trap: emitting [0, 0, 0] once per choice of indices.
    check(&[0, 0, 0, 0, 0], &[[0, 0, 0]]);
}

#[test]
fn duplicate_first_element() {
    // Trap: not skipping repeated values for the fixed element.
    check(&[-2, -2, 1, 1, 0, 2], &[[-2, 0, 2], [-2, 1, 1]]);
}

#[test]
fn duplicate_inner_elements() {
    // Trap: not skipping repeats after a match moves the pointers.
    check(&[-2, 0, 0, 0, 2, 2, 2], &[[-2, 0, 2], [0, 0, 0]]);
}

#[test]
fn no_index_reuse() {
    // -2 + 1 + 1 == 0 needs two 1s; there's only one.
    check(&[-2, 1, 3], &[]);
    check(&[-4, 2, 3], &[]);
}

#[test]
fn all_positive_or_all_negative() {
    check(&[1, 2, 3, 4], &[]);
    check(&[-1, -2, -3, -4], &[]);
}

#[test]
fn extreme_values() {
    check(
        &[-100_000, 50_000, 50_000, 100_000, 0],
        &[[-100_000, 0, 100_000], [-100_000, 50_000, 50_000]],
    );
}

#[test]
fn many_triplets() {
    check(
        &[-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6],
        &[
            [-4, -2, 6],
            [-4, 0, 4],
            [-4, 1, 3],
            [-4, 2, 2],
            [-2, -2, 4],
            [-2, 0, 2],
        ],
    );
}

#[test]
fn input_is_not_modified() {
    let nums = vec![3, -1, -2, 0, 1];
    let _ = three_sum(&nums);
    assert_eq!(nums, vec![3, -1, -2, 0, 1]);
}

#[test]
fn randomized_against_brute() {
    let mut rng = Rng(0x3141_5926_5358_9793);
    for _ in 0..300 {
        let n = rng.range(0, 14) as usize;
        let spread = rng.range(1, 8);
        let nums: Vec<i32> = (0..n).map(|_| rng.range(-spread, spread) as i32).collect();
        check(&nums, &brute_three_sum(&nums));
    }
}

#[test]
fn many_zeros() {
    let nums = vec![0; 300];
    check(&nums, &[[0, 0, 0]]);
}

#[test]
fn random_with_many_duplicates() {
    // n = 500 with many triplets and duplicates.
    let mut rng = Rng(0x2718_2818_2845_9045);
    let nums: Vec<i32> = (0..500).map(|_| rng.range(-1000, 1000) as i32).collect();
    let exp = oracle_by_counts(&nums);
    check(&nums, &exp);
}

#[test]
fn large_no_answer() {
    // N odd values: odd + odd + odd is odd, never 0. A triple loop tries all
    // ~N^3 / 6 = 5.6 * 10^8 triples here; sort + two pointers is ~N^2 / 2 =
    // 1.1 * 10^6 steps.
    let n = 1500;
    let mut rng = Rng(0x6A09_E667_BB67_AE85);
    let nums: Vec<i32> = (0..n).map(|_| (rng.range(-50_000, 49_999) * 2 + 1) as i32).collect();
    let got = three_sum(&nums);
    assert!(got.is_empty(), "n = {n}: got {got:?}");
}
