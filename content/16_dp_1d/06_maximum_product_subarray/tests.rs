// Tests for Maximum Product Subarray — shared by template.rs, solution.rs and your attempt.
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

    /// Uniform in `-m..=m`.
    fn signed(&mut self, m: i32) -> i32 {
        let span = 2 * m.unsigned_abs() as usize + 1;
        i32::try_from(self.below(span)).expect("small") - m
    }
}

/// Brute force O(n^2) in i128: product of every subarray.
fn brute_max(nums: &[i32]) -> Option<i128> {
    (0..nums.len())
        .flat_map(|i| {
            nums[i..].iter().scan(1i128, |p, &x| {
                *p *= i128::from(x);
                Some(*p)
            })
        })
        .max()
}

/// Independent O(n) oracle: within each zero-free segment the best product is
/// the whole segment (even number of negatives), or the part after the first
/// negative / before the last negative (odd number). Zeros and single
/// elements are covered by taking the max element too.
fn oracle_max(nums: &[i32]) -> Option<i128> {
    let prod = |s: &[i32]| s.iter().map(|&x| i128::from(x)).product::<i128>();
    let mut best = i128::from(*nums.iter().max()?);
    for seg in nums.split(|&x| x == 0).filter(|seg| !seg.is_empty()) {
        if prod(seg) > 0 {
            best = best.max(prod(seg));
        } else {
            let first = seg.iter().position(|&x| x < 0).expect("odd negatives");
            let last = seg.iter().rposition(|&x| x < 0).expect("odd negatives");
            if first + 1 < seg.len() {
                best = best.max(prod(&seg[first + 1..]));
            }
            if last > 0 {
                best = best.max(prod(&seg[..last]));
            }
        }
    }
    Some(best)
}

fn wide(nums: &[i32]) -> Option<i128> {
    max_product(nums).map(i128::from)
}

#[test]
fn example_1() {
    assert_eq!(max_product(&[2, 3, -2, 4]), Some(6));
}

#[test]
fn example_2() {
    assert_eq!(max_product(&[-2, 0, -1]), Some(0));
}

#[test]
fn empty_is_none() {
    assert_eq!(max_product(&[]), None);
}

#[test]
fn single_negative() {
    // Trap: initialising the answer to 0 or 1 instead of the first element.
    assert_eq!(max_product(&[-5]), Some(-5));
    assert_eq!(max_product(&[-10]), Some(-10));
}

#[test]
fn single_zero_and_positive() {
    assert_eq!(max_product(&[0]), Some(0));
    assert_eq!(max_product(&[7]), Some(7));
}

#[test]
fn two_negatives_make_a_positive() {
    // Trap: tracking only the running maximum (sum-style).
    assert_eq!(max_product(&[-2, 3, -4]), Some(24));
    assert_eq!(max_product(&[-2, -3]), Some(6));
}

#[test]
fn odd_number_of_negatives() {
    assert_eq!(max_product(&[-1, -2, -3]), Some(6));
    assert_eq!(max_product(&[2, -5, -2, -4, 3]), Some(24));
}

#[test]
fn zeros_split_the_array() {
    assert_eq!(max_product(&[-2, 0, -3]), Some(0));
    assert_eq!(max_product(&[2, 3, 0, 4, 5]), Some(20));
    assert_eq!(max_product(&[0, -2, -3, 0, 1]), Some(6));
}

#[test]
fn all_negative_with_zero_between() {
    assert_eq!(max_product(&[-1, 0, -1]), Some(0));
    assert_eq!(max_product(&[-3, -1, -1]), Some(3));
}

#[test]
fn overflow_in_i32_product() {
    // Trap: multiplying in i32. 10^10 and beyond don't fit.
    assert_eq!(max_product(&[10; 10]), Some(10_000_000_000));
    assert_eq!(max_product(&[10; 18]), Some(1_000_000_000_000_000_000));
    // (-1) * (-10)^17 * 10 = 10^18; the whole array is the answer.
    let mut nums = vec![-1];
    nums.extend([-10; 17]);
    nums.push(10);
    assert_eq!(max_product(&nums), Some(1_000_000_000_000_000_000));
}

#[test]
fn product_hits_i64_min_but_answer_is_positive() {
    // The whole slice's product is (-2)^63 = i64::MIN (still fits);
    // the best is 62 of them: 2^62.
    assert_eq!(max_product(&[-2; 63]), Some(1 << 62));
}

#[test]
fn random_small_match_brute_force() {
    let mut rng = Rng(0xABCD_EF01_2345_6789);
    for _ in 0..500 {
        // |products| stay within i64: 10^18 at most.
        let n = rng.below(19);
        let m = [2, 4, 10][rng.below(3)];
        let nums: Vec<i32> = (0..n).map(|_| rng.signed(m)).collect();
        assert_eq!(wide(&nums), brute_max(&nums), "nums = {nums:?}");
    }
}

#[test]
fn oracle_agrees_with_brute_force() {
    // Sanity-check the linear oracle used by the large tests.
    let mut rng = Rng(5);
    for _ in 0..300 {
        let n = rng.below(13);
        let nums: Vec<i32> = (0..n).map(|_| rng.signed(4)).collect();
        assert_eq!(oracle_max(&nums), brute_max(&nums), "nums = {nums:?}");
        assert_eq!(wide(&nums), oracle_max(&nums), "nums = {nums:?}");
    }
}

#[test]
fn large_random_plus_minus_one_and_twos() {
    // n = 2 * 10^4 of +-1 and at most 62 entries of +-2 (so every subarray
    // product fits in i64). Round 0 also has a few zeros; the others have
    // none, so a scan can't cut a row short when its product hits 0.
    // Multiplying out every (start, end) pair is n^2 / 2 = 2 * 10^8 products
    // per input; one pass is 2 * 10^4.
    const ROUNDS: usize = 3;
    let n = 20_000;
    let mut rng = Rng(8675309);
    for round in 0..ROUNDS {
        let zeros = if round == 0 { 2 } else { 0 };
        let mut nums: Vec<i32> = (0..n)
            .map(|_| {
                let r = rng.below(1000);
                if r < zeros {
                    0
                } else if r <= 500 {
                    1
                } else {
                    -1
                }
            })
            .collect();
        for _ in 0..62 {
            let i = rng.below(n);
            nums[i] = if rng.below(2) == 0 { 2 } else { -2 };
        }
        assert_eq!(wide(&nums), oracle_max(&nums), "round {round}, n = {n}");
    }
}

#[test]
fn large_no_zeros_single_negative_in_middle() {
    // 35 twos before the -1 and 25 after (all products fit: |2^60| at most):
    // best is the left part, 2^35.
    let mut nums = vec![1; 20_000];
    for i in 0..35 {
        nums[i * 280] = 2;
    }
    nums[10_000] = -1;
    for i in 0..25 {
        nums[11_000 + i * 350] = 2;
    }
    assert_eq!(max_product(&nums), Some(1 << 35));
}
