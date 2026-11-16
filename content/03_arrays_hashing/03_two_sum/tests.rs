// Tests for Two Sum — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Inputs here have a unique answer.
fn check(nums: &[i32], target: i32, expected: (usize, usize)) {
    assert_eq!(
        two_sum(nums, target),
        Some(expected),
        "nums = {nums:?}, target = {target}"
    );
}

/// Validate by property: two in-range indices i < j whose values sum to target.
fn check_valid(nums: &[i32], target: i32) {
    let Some((i, j)) = two_sum(nums, target) else {
        panic!("expected a pair, got None; nums = {nums:?}, target = {target}");
    };
    assert!(
        i < j && j < nums.len(),
        "invalid indices ({i}, {j}); nums = {nums:?}, target = {target}"
    );
    assert_eq!(
        i64::from(nums[i]) + i64::from(nums[j]),
        i64::from(target),
        "indices ({i}, {j}) don't sum to target; nums = {nums:?}, target = {target}"
    );
}

/// Obviously-correct O(n²) oracle: does any pair exist?
fn brute_has_pair(nums: &[i32], target: i32) -> bool {
    (0..nums.len()).any(|i| {
        (i + 1..nums.len()).any(|j| i64::from(nums[i]) + i64::from(nums[j]) == i64::from(target))
    })
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
    check(&[2, 7, 11, 15], 9, (0, 1));
}

#[test]
fn example_2() {
    check(&[3, 2, 4], 6, (1, 2));
}

#[test]
fn example_3() {
    check(&[3, 3], 6, (0, 1));
}

#[test]
fn example_no_pair() {
    assert_eq!(two_sum(&[1, 2, 3], 7), None);
}

#[test]
fn example_empty() {
    assert_eq!(two_sum(&[], 0), None);
}

#[test]
fn single_element_is_none() {
    // Trap: pairing the only element with itself.
    assert_eq!(two_sum(&[4], 8), None);
    assert_eq!(two_sum(&[0], 0), None);
}

#[test]
fn must_not_reuse_same_index() {
    // 4 + 4 == 8 but there's only one 4.
    check(&[4, 1, 7], 8, (1, 2));
}

#[test]
fn smaller_index_first() {
    // The pair comes back as (i, j) with i < j.
    check(&[9, 1, 5], 6, (1, 2));
    check(&[1, 8, 3, 4], 9, (0, 1));
}

#[test]
fn negatives() {
    check(&[-1, -2, -3, -4, -5], -8, (2, 4));
}

#[test]
fn zeros() {
    check(&[0, 4, 3, 0], 0, (0, 3));
}

#[test]
fn answer_at_ends() {
    check(&[5, 1, 2, 3, 9], 14, (0, 4));
}

#[test]
fn complement_out_of_value_range() {
    // target - nums[0] == 2 * 10^9: outside the value range, still fits in i32.
    check(&[-1_000_000_000, 3, 1_000_000_000, 0], 1_000_000_000, (2, 3));
    // target - nums[0] == -2 * 10^9.
    check(&[1_000_000_000, 7, -1_000_000_000, 0], -1_000_000_000, (2, 3));
}

#[test]
fn extreme_values_sum() {
    check(&[500_000_000, 5, 500_000_000], 1_000_000_000, (0, 2));
    check(&[-1_000_000_000, 5, 0], -1_000_000_000, (0, 2));
}

#[test]
fn extreme_values_no_pair() {
    assert_eq!(two_sum(&[1_000_000_000, 1_000_000_000], -1_000_000_000), None);
    assert_eq!(two_sum(&[-1_000_000_000, -1_000_000_000], 1_000_000_000), None);
}

#[test]
fn several_answers_any_is_accepted() {
    check_valid(&[1, 2, 3, 4, 5, 6], 7);
    check_valid(&[2, 2, 2, 2], 4);
}

#[test]
fn randomized_property() {
    let mut rng = Rng(0xDEAD_BEEF_1234_5678);
    for _ in 0..400 {
        let n = rng.range(2, 12) as usize;
        let lim = if rng.range(0, 1) == 0 { 10 } else { 1_000_000_000 };
        let nums: Vec<i32> = (0..n).map(|_| rng.range(-lim / 2, lim / 2) as i32).collect();
        let a = rng.range(0, n as i64 - 1) as usize;
        let mut b = rng.range(0, n as i64 - 2) as usize;
        if b >= a {
            b += 1;
        }
        let target = nums[a] + nums[b];
        check_valid(&nums, target);
    }
}

#[test]
fn randomized_against_brute_force() {
    // Targets are arbitrary, so both Some and None come up.
    let mut rng = Rng(0x0BAD_CAFE_F00D_0001);
    for _ in 0..500 {
        let n = rng.range(0, 10) as usize;
        let nums: Vec<i32> = (0..n).map(|_| rng.range(-10, 10) as i32).collect();
        let target = rng.range(-25, 25) as i32;
        let got = two_sum(&nums, target);
        assert_eq!(
            got.is_some(),
            brute_has_pair(&nums, target),
            "got {got:?}; nums = {nums:?}, target = {target}"
        );
        if got.is_some() {
            check_valid(&nums, target);
        }
    }
}

#[test]
fn large_duplicates_pair() {
    let n = 10_000;
    let mut nums = vec![1; n];
    nums[0] = 500;
    nums[n - 1] = 500;
    check(&nums, 1000, (0, n - 1));
}

#[test]
fn large_inputs_with_no_pair_or_pair_at_the_end() {
    // 14 inputs of 10^4 elements: half have no pair, half have it in the last
    // two positions. Trying every pair is ~7 * 10^8 checks in total; one pass
    // with a hash map is 1.4 * 10^5 lookups.
    let n = 10_000;
    for round in 0..14 {
        // All even and distinct, shifted each round so no two inputs are alike.
        let mut nums: Vec<i32> = (0..n as i32).map(|i| 2 * (i + round)).collect();
        if round % 2 == 0 {
            assert_eq!(two_sum(&nums, 1_000_001), None, "round {round}, n = {n}");
        } else {
            nums[n - 2] = 1_000_001 + 2 * round; // the only odd values
            nums[n - 1] = 1_000_003;
            assert_eq!(
                two_sum(&nums, 2_000_004 + 2 * round),
                Some((n - 2, n - 1)),
                "round {round}, n = {n}"
            );
        }
    }
}
