// Tests for Partition Equal Subset Sum — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Tiny deterministic PRNG (xorshift64) so random tests are reproducible.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }

    /// Uniform in `lo..=hi`.
    fn range(&mut self, lo: u32, hi: u32) -> u32 {
        lo + u32::try_from(self.next() % (u64::from(hi - lo) + 1)).expect("below span")
    }
}

/// Brute force: try every assignment of elements to the two groups.
fn brute_partition(nums: &[u32]) -> bool {
    let n = nums.len();
    let total: u64 = nums.iter().map(|&x| u64::from(x)).sum();
    (0u32..1 << n).any(|mask| {
        let a: u64 = (0..n)
            .filter(|&i| mask >> i & 1 == 1)
            .map(|i| u64::from(nums[i]))
            .sum();
        2 * a == total
    })
}

/// Independent oracle: the set of all reachable subset sums.
fn set_partition(nums: &[u32]) -> bool {
    let total: u32 = nums.iter().sum();
    let mut sums = std::collections::BTreeSet::from([0u32]);
    for &x in nums {
        let shifted: Vec<u32> = sums.iter().map(|s| s + x).collect();
        sums.extend(shifted);
    }
    total.is_multiple_of(2) && sums.contains(&(total / 2))
}

#[test]
fn example_1() {
    assert!(can_partition(&[1, 5, 11, 5]));
}

#[test]
fn example_2() {
    assert!(!can_partition(&[1, 2, 3, 5]));
}

#[test]
fn empty_splits_into_two_empty_groups() {
    assert!(can_partition(&[]));
}

#[test]
fn zeros_split_evenly() {
    assert!(can_partition(&[0]));
    assert!(can_partition(&[0, 0, 0]));
    assert!(can_partition(&[0, 4, 0, 4]));
    assert!(!can_partition(&[0, 3]));
}

#[test]
fn single_element_never_splits() {
    // Even if the value is even, one group would be empty and the other not.
    assert!(!can_partition(&[1]));
    assert!(!can_partition(&[2]));
    assert!(!can_partition(&[100]));
}

#[test]
fn two_elements() {
    assert!(can_partition(&[7, 7]));
    assert!(!can_partition(&[7, 9]));
}

#[test]
fn even_total_but_impossible() {
    // Total 12, half 6 — no subset reaches it.
    assert!(!can_partition(&[2, 2, 3, 5]));
    // One element larger than half the total.
    assert!(!can_partition(&[1, 1, 10]));
}

#[test]
fn element_reuse_trap() {
    // Unbounded-knapsack bug (iterating sums upwards) lets an element be used
    // repeatedly. Total 6, half 3: only reachable as 1 + 1 + 1.
    assert!(!can_partition(&[1, 5]));
    // Total 2 + 6 = 8, half 4 = 2 + 2 only with reuse.
    assert!(!can_partition(&[2, 6]));
}

#[test]
fn greedy_balancing_fails() {
    // Sorted descending, "put the next number in the lighter group" gives
    // {3, 2, 2} vs {3, 2} = 7 vs 5. Real answer: {3, 3} vs {2, 2, 2}.
    assert!(can_partition(&[3, 3, 2, 2, 2]));
}

#[test]
fn all_equal() {
    assert!(can_partition(&[5; 4]));
    assert!(!can_partition(&[5; 3]));
}

#[test]
fn needs_many_elements_on_one_side() {
    assert!(can_partition(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 10]));
    assert!(can_partition(&[1, 2, 3, 4, 5, 6, 7]));
}

#[test]
fn random_small_match_brute_force() {
    let mut rng = Rng(0x7777_1111_3333_9999);
    for _ in 0..400 {
        let n = rng.below(15);
        let (lo, hi) = [(0, 6), (1, 6), (1, 100)][rng.below(3)];
        let nums: Vec<u32> = (0..n).map(|_| rng.range(lo, hi)).collect();
        assert_eq!(can_partition(&nums), brute_partition(&nums), "nums = {nums:?}");
    }
}

#[test]
fn large_even_total_unreachable_half() {
    // 199 sixes and one 2: total 1196, half 598. Reachable sums are 6k and
    // 6k + 2, and 598 = 6 * 99 + 4 — impossible. Exponential search explodes.
    let mut nums = vec![6; 199];
    nums.push(2);
    assert!(!can_partition(&nums));
}

#[test]
fn large_odd_values_even_count() {
    // 200 odd values: total even. Pair them up to make a valid split.
    let mut rng = Rng(1618);
    let half: Vec<u32> = (0..100).map(|_| rng.range(0, 49) * 2 + 1).collect();
    let mut nums = half.clone();
    let mut other = half;
    // Shuffle the second copy so the split isn't simply "first half / second half".
    for i in (1..other.len()).rev() {
        let j = rng.below(i + 1);
        other.swap(i, j);
    }
    nums.extend(other);
    assert!(can_partition(&nums));
}

#[test]
fn large_odd_count_of_odd_values() {
    // An odd number of odd values has an odd total: false straight away.
    let mut nums = vec![99; 199];
    nums.push(100);
    assert!(!can_partition(&nums));
}

#[test]
fn large_all_max() {
    assert!(can_partition(&[100; 200]));
    // 199 hundreds + one 1: total 19901 is odd.
    let mut nums = vec![100; 199];
    nums.push(1);
    assert!(!can_partition(&nums));
}

#[test]
fn large_one_big_rest_small() {
    // Total 198, half 99: the 100 overshoots and the ones only reach 98.
    let mut nums = vec![1; 98];
    nums.push(100);
    assert!(!can_partition(&nums));
    // With 100 ones it splits as {100} vs {1 x 100}.
    let mut nums = vec![1; 100];
    nums.push(100);
    assert!(can_partition(&nums));
}

#[test]
fn large_random_matches_set_oracle() {
    // Mostly-even inputs with a few odd values make both answers common.
    let mut rng = Rng(99);
    for _ in 0..6 {
        let n = rng.range(150, 200) as usize;
        let mut nums: Vec<u32> = (0..n).map(|_| rng.range(0, 50) * 2).collect();
        for _ in 0..rng.below(4) {
            let i = rng.below(n);
            nums[i] = rng.range(0, 49) * 2 + 1;
        }
        assert_eq!(can_partition(&nums), set_partition(&nums), "nums = {nums:?}");
    }
    // Constructed true case: a random half, duplicated, plus a pair of 1s.
    let half: Vec<u32> = (0..99).map(|_| rng.range(0, 100)).collect();
    let mut nums: Vec<u32> = half.iter().chain(&half).copied().collect();
    nums.extend([1, 1]);
    assert!(can_partition(&nums));
}
