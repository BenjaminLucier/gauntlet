// Tests for Product of Array Except Self — shared by template.rs, solution.rs and your attempt.
use super::*;

fn check(nums: &[i32], expected: &[i32]) {
    let got = product_except_self(nums);
    if nums.len() <= 50 {
        assert_eq!(got, expected, "nums = {nums:?}");
    } else {
        assert!(got == expected, "wrong answer for nums.len() = {}", nums.len());
    }
}

/// Obviously-correct O(n²) oracle, computed in i64.
fn brute_product(nums: &[i32]) -> Vec<i32> {
    (0..nums.len())
        .map(|i| {
            let p: i64 = nums
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, &v)| i64::from(v))
                .product();
            i32::try_from(p).expect("test inputs keep products in range")
        })
        .collect()
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
    check(&[1, 2, 3, 4], &[24, 12, 8, 6]);
}

#[test]
fn example_2() {
    check(&[-1, 1, 0, -3, 3], &[0, 0, 9, 0, 0]);
}

#[test]
fn example_two_zeros() {
    check(&[0, 2, 0], &[0, 0, 0]);
}

#[test]
fn empty_input() {
    assert_eq!(product_except_self(&[]), Vec::<i32>::new());
}

#[test]
fn single_element_is_empty_product() {
    check(&[7], &[1]);
    check(&[0], &[1]);
    check(&[-30], &[1]);
}

#[test]
fn two_elements() {
    check(&[5, 7], &[7, 5]);
    check(&[-3, 0], &[0, -3]);
}

#[test]
fn single_zero() {
    // Trap: division-based solutions divide by zero.
    check(&[2, 0, 3, 4], &[0, 24, 0, 0]);
}

#[test]
fn two_zeros() {
    // Trap: "product of non-zeros" for the zero position — wrong with two zeros.
    check(&[0, 2, 0, 3], &[0, 0, 0, 0]);
}

#[test]
fn zero_at_ends() {
    check(&[0, 1, 2, 3], &[6, 0, 0, 0]);
    check(&[1, 2, 3, 0], &[0, 0, 0, 6]);
}

#[test]
fn all_ones() {
    check(&[1, 1, 1, 1, 1], &[1, 1, 1, 1, 1]);
}

#[test]
fn all_negative() {
    check(&[-1, -2, -3], &[6, 3, 2]);
    check(&[-1, -2, -3, -4], &[-24, -12, -8, -6]);
}

#[test]
fn extreme_values() {
    // |product of all| = 30^6 = 729_000_000 fits in i32.
    check(
        &[30, 30, 30, 30, 30, -30],
        &[
            -24_300_000,
            -24_300_000,
            -24_300_000,
            -24_300_000,
            -24_300_000,
            24_300_000,
        ],
    );
    check(&[-30, -30, -30, -30, -30, -30], &[-24_300_000; 6]);
}

#[test]
fn input_is_left_untouched() {
    let nums = vec![2, 3, 4];
    let out = product_except_self(&nums);
    assert_eq!(out, vec![12, 8, 6]);
    assert_eq!(nums, vec![2, 3, 4]);
}

#[test]
fn randomized_against_brute() {
    let mut rng = Rng(0x5555_AAAA_1234_4321);
    for _ in 0..400 {
        let n = rng.range(0, 9) as usize;
        let nums: Vec<i32> = (0..n).map(|_| rng.range(-4, 4) as i32).collect();
        check(&nums, &brute_product(&nums));
    }
}

#[test]
fn large_input() {
    // Mostly ±1 with a few 2s so products stay in range. Multiplying
    // everything but nums[i] afresh for every i is ~6 * 10^8 steps here;
    // prefix and suffix passes are 5 * 10^4.
    let n = 25_000;
    let nums: Vec<i32> = (0..n)
        .map(|i| {
            if i % 9973 == 1000 {
                2
            } else if i % 3 == 1 {
                -1
            } else {
                1
            }
        })
        .collect();
    let total: i64 = nums.iter().map(|&v| i64::from(v)).product();
    let expected: Vec<i32> =
        nums.iter().map(|&v| i32::try_from(total / i64::from(v)).expect("in range")).collect();
    assert!(product_except_self(&nums) == expected, "wrong answer for n = {n}");
}

#[test]
fn large_with_one_zero() {
    let n = 2_000;
    let mut nums = vec![1; n];
    nums[n / 2] = 0;
    nums[7] = -2;
    let mut expected = vec![0; n];
    expected[n / 2] = -2;
    check(&nums, &expected);
}
