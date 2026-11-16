// Tests for Single Number — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::collections::{HashMap, HashSet};

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

/// Oracle: count occurrences, return the value seen once.
fn brute_single(nums: &[i32]) -> Option<i32> {
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for &x in nums {
        *counts.entry(x).or_default() += 1;
    }
    counts.into_iter().find(|&(_, c)| c == 1).map(|(x, _)| x)
}

fn shuffle<T>(rng: &mut Rng, v: &mut [T]) {
    for i in (1..v.len()).rev() {
        let j = rng.range(0, i as i64) as usize;
        v.swap(i, j);
    }
}

/// Builds a shuffled input with `pairs` distinct paired values plus `single`.
fn make_input(rng: &mut Rng, pairs: usize, single: i32) -> Vec<i32> {
    let mut used = HashSet::from([single]);
    let mut v = vec![single];
    while used.len() <= pairs {
        let x = rng.range(-30_000, 30_000) as i32;
        if used.insert(x) {
            v.push(x);
            v.push(x);
        }
    }
    shuffle(rng, &mut v);
    v
}

fn check(nums: &[i32], expected: i32) {
    assert_eq!(single_number(nums), Some(expected), "nums = {nums:?}");
}

#[test]
fn example_1() {
    check(&[2, 2, 1], 1);
}

#[test]
fn example_2() {
    check(&[4, 1, 2, 1, 2], 4);
}

#[test]
fn example_3() {
    check(&[1], 1);
}

#[test]
fn empty_slice_is_none() {
    assert_eq!(single_number::<i32>(&[]), None);
    assert_eq!(single_number::<u64>(&[]), None);
}

#[test]
fn single_is_zero() {
    check(&[5, 0, 5], 0);
    check(&[0], 0);
}

#[test]
fn single_is_negative() {
    check(&[-1, 3, 3], -1);
    check(&[7, -30_000, 7], -30_000);
}

#[test]
fn negatives_paired() {
    check(&[-5, -5, -7, 2, -7], 2);
}

#[test]
fn single_at_start_and_end() {
    check(&[9, 1, 1, 2, 2], 9);
    check(&[1, 1, 2, 2, 9], 9);
}

#[test]
fn pairs_not_adjacent() {
    check(&[1, 2, 3, 4, 5, 1, 2, 3, 4], 5);
}

#[test]
fn bounds() {
    check(&[30_000, -30_000, 30_000], -30_000);
    check(&[-30_000, 30_000, -30_000], 30_000);
    check(&[i32::MIN, i32::MAX, i32::MIN], i32::MAX);
    check(&[-1, i32::MIN, -1], i32::MIN);
}

#[test]
fn generic_over_integer_types() {
    assert_eq!(single_number(&[7u8, 3, 7]), Some(3));
    assert_eq!(single_number(&[u8::MAX, 0, 0]), Some(u8::MAX));
    assert_eq!(single_number(&[-1i64, i64::MIN, -1]), Some(i64::MIN));
    assert_eq!(single_number(&[u128::MAX, 5, u128::MAX]), Some(5));
    assert_eq!(single_number(&[3usize, 3, 0]), Some(0));
}

#[test]
fn generic_over_bool() {
    assert_eq!(single_number(&[true, false, true]), Some(false));
    assert_eq!(single_number(&[false, true, false]), Some(true));
    assert_eq!(single_number(&[true]), Some(true));
}

#[test]
fn generic_over_custom_type() {
    // A user-defined bitmask type with its own `^`.
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Mask(u16);
    impl BitXor for Mask {
        type Output = Mask;
        fn bitxor(self, rhs: Mask) -> Mask {
            Mask(self.0 ^ rhs.0)
        }
    }
    let masks = [Mask(0b1010), Mask(0xFF00), Mask(0b1010)];
    assert_eq!(single_number(&masks), Some(Mask(0xFF00)));
}

#[test]
fn randomized_vs_brute_force() {
    let mut rng = Rng(0x1111_2222_3333_4444);
    for _ in 0..300 {
        let pairs = rng.range(0, 30) as usize;
        let single = rng.range(-30_000, 30_000) as i32;
        let nums = make_input(&mut rng, pairs, single);
        assert_eq!(brute_single(&nums), Some(single));
        check(&nums, single);
    }
}

#[test]
fn large_input() {
    let mut rng = Rng(0x9999_8888_7777_6666);
    let nums = make_input(&mut rng, 14_999, -12_345);
    assert_eq!(nums.len(), 29_999);
    for _ in 0..20 {
        check(&nums, -12_345);
    }
}

#[test]
fn large_many_values_u64() {
    let mut rng = Rng(0x0abc_def0_1234_5678);
    let mut nums: Vec<u64> = (0..200_000).map(|_| rng.next()).collect();
    let single = rng.next();
    nums.extend_from_within(..);
    nums.push(single);
    shuffle(&mut rng, &mut nums);
    assert_eq!(single_number(&nums), Some(single));
}
