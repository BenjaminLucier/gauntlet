// Tests for Climbing Stairs — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Exponential brute force: try a 1-step and a 2-step from every position.
fn brute_ways(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => brute_ways(n - 1) + brute_ways(n - 2),
    }
}

/// fib[k] with fib[1] = fib[2] = 1, computed in u128 so it can't overflow here.
fn fibonacci(len: usize) -> Vec<u128> {
    let mut fib = vec![0u128, 1, 1];
    while fib.len() < len {
        let k = fib.len();
        fib.push(fib[k - 1] + fib[k - 2]);
    }
    fib
}

#[test]
fn example_1() {
    assert_eq!(climb_stairs(2), 2);
}

#[test]
fn example_2() {
    assert_eq!(climb_stairs(3), 3);
}

#[test]
fn one_step() {
    // Base case trap: dp[2] must not be read/written when n == 1.
    assert_eq!(climb_stairs(1), 1);
}

#[test]
fn zero_steps_is_one_empty_way() {
    assert_eq!(climb_stairs(0), 1);
}

#[test]
fn four_and_five_steps() {
    assert_eq!(climb_stairs(4), 5);
    assert_eq!(climb_stairs(5), 8);
}

#[test]
fn ten_and_twenty_steps() {
    assert_eq!(climb_stairs(10), 89);
    assert_eq!(climb_stairs(20), 10_946);
}

#[test]
fn small_n_match_brute_force() {
    for n in 0..=22 {
        assert_eq!(climb_stairs(n), brute_ways(n), "n = {n}");
    }
}

#[test]
fn every_n_matches_fibonacci() {
    // ways(n) = fib(n + 1) with fib(1) = fib(2) = 1.
    let fib = fibonacci(95);
    for n in 0..=92u32 {
        assert_eq!(u128::from(climb_stairs(n)), fib[n as usize + 1], "n = {n}");
    }
}

#[test]
fn large_n_is_not_exponential() {
    // Plain recursion makes ~2 * ways(n) calls: ~3.7 * 10^9 at n = 45, and
    // ~2.4 * 10^19 at n = 92. Memoize or iterate.
    assert_eq!(climb_stairs(44), 1_134_903_170);
    assert_eq!(climb_stairs(45), 1_836_311_903);
    assert_eq!(climb_stairs(92), 12_200_160_415_121_876_738);
}

#[test]
fn past_i32_range() {
    // An i32 accumulator overflows from n = 45 on.
    assert_eq!(climb_stairs(46), 2_971_215_073);
    assert_eq!(climb_stairs(60), 2_504_730_781_961);
}

#[test]
fn past_i64_range() {
    // fib(93) > i64::MAX: a signed 64-bit accumulator overflows at n = 92.
    assert_eq!(climb_stairs(91), 7_540_113_804_746_346_429);
    assert_eq!(climb_stairs(92), 12_200_160_415_121_876_738);
}

#[test]
fn overflow_computing_one_term_too_many() {
    // ways(93) doesn't fit in u64. A loop that computes one term beyond the
    // answer (and throws it away) panics here under overflow checks.
    assert_eq!(climb_stairs(92), 12_200_160_415_121_876_738);
}

#[test]
fn consecutive_values_follow_recurrence() {
    for n in 2..=92 {
        let (a, b, c) = (climb_stairs(n - 2), climb_stairs(n - 1), climb_stairs(n));
        assert_eq!(u128::from(a) + u128::from(b), u128::from(c), "n = {n}");
    }
}

#[test]
fn repeated_calls_are_independent() {
    // Catches solutions that keep state between calls (e.g. a static cache bug).
    assert_eq!(climb_stairs(45), 1_836_311_903);
    assert_eq!(climb_stairs(1), 1);
    assert_eq!(climb_stairs(30), 1_346_269);
    assert_eq!(climb_stairs(2), 2);
    assert_eq!(climb_stairs(0), 1);
}
