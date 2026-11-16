// Tests for Maximum Subarray — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Obviously-correct O(n²) oracle: every non-empty (start, end) pair.
fn brute_max_subarray_sum(nums: &[i32]) -> Option<i32> {
    (0..nums.len())
        .flat_map(|i| {
            nums[i..].iter().scan(0, |sum, &x| {
                *sum += x;
                Some(*sum)
            })
        })
        .max()
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
    assert_eq!(max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), Some(6));
}

#[test]
fn example_2() {
    assert_eq!(max_subarray_sum(&[1]), Some(1));
}

#[test]
fn example_3() {
    assert_eq!(max_subarray_sum(&[5, 4, -1, 7, 8]), Some(23));
}

#[test]
fn empty_is_none() {
    assert_eq!(max_subarray_sum(&[]), None);
}

#[test]
fn works_on_vec_and_subslice() {
    let v = vec![3, -10, 2, 2, -10, 3];
    assert_eq!(max_subarray_sum(&v), Some(4));
    assert_eq!(max_subarray_sum(&v[..1]), Some(3));
    assert_eq!(max_subarray_sum(&v[1..2]), Some(-10));
    assert_eq!(max_subarray_sum(&v[6..]), None);
}

#[test]
fn single_negative() {
    assert_eq!(max_subarray_sum(&[-7]), Some(-7));
}

#[test]
fn all_negative_is_not_zero() {
    // Trap: initializing the answer to 0 (allowing the empty subarray).
    assert_eq!(max_subarray_sum(&[-3, -1, -2]), Some(-1));
    assert_eq!(max_subarray_sum(&[-10_000, -10_000]), Some(-10_000));
}

#[test]
fn best_is_last_element() {
    assert_eq!(max_subarray_sum(&[-5, -1, -8, 3]), Some(3));
}

#[test]
fn best_is_first_element() {
    assert_eq!(max_subarray_sum(&[9, -10, 2, 3]), Some(9));
}

#[test]
fn zeros() {
    assert_eq!(max_subarray_sum(&[0, 0, 0]), Some(0));
    assert_eq!(max_subarray_sum(&[-1, 0, -2]), Some(0));
}

#[test]
fn bridge_across_negative() {
    // Keeping the dip is worth it: 5 + (-1) + 5 = 9.
    assert_eq!(max_subarray_sum(&[5, -1, 5]), Some(9));
    // Not worth it: 5 + (-6) + 5 = 4 < 5.
    assert_eq!(max_subarray_sum(&[5, -6, 5]), Some(5));
}

#[test]
fn reset_when_running_sum_negative() {
    assert_eq!(max_subarray_sum(&[2, -3, 1, 1, 1]), Some(3));
}

#[test]
fn all_positive_is_whole_array() {
    assert_eq!(max_subarray_sum(&[1, 2, 3, 4, 5]), Some(15));
}

#[test]
fn min_sentinel_overflow() {
    // Trap: starting a running sum at i32::MIN and adding a negative panics.
    assert_eq!(max_subarray_sum(&[-10_000, -1, -10_000]), Some(-1));
}

#[test]
fn random_against_brute_force() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for _ in 0..500 {
        let n = rng.range(0, 15) as usize;
        let nums: Vec<i32> = (0..n).map(|_| rng.range(-10, 10)).collect();
        assert_eq!(
            max_subarray_sum(&nums),
            brute_max_subarray_sum(&nums),
            "nums: {nums:?}"
        );
    }
}

#[test]
fn large_max_values() {
    let nums = vec![10_000; 2000];
    assert_eq!(max_subarray_sum(&nums), Some(20_000_000));
}

#[test]
fn large_all_min_values() {
    let nums = vec![-10_000; 2000];
    assert_eq!(max_subarray_sum(&nums), Some(-10_000));
}

#[test]
fn large_alternating() {
    let nums: Vec<i32> = (0..2000).map(|i| if i % 2 == 0 { 3 } else { -2 }).collect();
    // 1000 threes and 999 minus-twos between the first and last 3.
    assert_eq!(max_subarray_sum(&nums), Some(1000 * 3 - 999 * 2));
}

#[test]
fn large_inputs_with_one_non_negative_block() {
    // Each input is negative everywhere except one block of non-negative
    // values, whose sum is therefore the answer. The block sits at the very
    // end in the last round. Trying every (start, end) pair costs n^2 / 2 =
    // 2.4 * 10^8 additions per input, with no early exit; one pass is n.
    const ROUNDS: usize = 2;
    let n = 22_000;
    let mut rng = Rng(0x5A1E_0B10_C4ED_0001);
    for round in 0..ROUNDS {
        let mut nums: Vec<i32> = (0..n).map(|_| rng.range(-10_000, -1)).collect();
        let len = 1 + (rng.next() % 1000) as usize;
        let start = if round == ROUNDS - 1 { n - len } else { (rng.next() % (n - len) as u64) as usize };
        for x in &mut nums[start..start + len] {
            *x = rng.range(0, 10_000);
        }
        let want: i32 = nums[start..start + len].iter().sum();
        assert_eq!(max_subarray_sum(&nums), Some(want), "round {round}, n = {n}, block {start}..{}", start + len);
    }
}
