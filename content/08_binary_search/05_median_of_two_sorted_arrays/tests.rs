// Tests for Median of Two Sorted Arrays — shared by template.rs, solution.rs and your attempt.
use super::*;

fn assert_close(got: Option<f64>, expected: f64, context: &str) {
    match got {
        Some(x) => assert!((x - expected).abs() < 1e-6, "{context}: expected {expected}, got {x}"),
        None => panic!("{context}: expected {expected}, got None"),
    }
}

fn check(a: &[i32], b: &[i32], expected: f64) {
    assert_close(median_of_sorted(a, b), expected, &format!("a = {a:?}, b = {b:?}"));
}

fn sorted_vec(rng: &mut Rng, len: usize, lo: i64, hi: i64) -> Vec<i32> {
    let mut v: Vec<i32> = (0..len).map(|_| rng.range(lo, hi) as i32).collect();
    v.sort_unstable();
    v
}

/// Oracle: concatenate, sort, take the middle.
fn brute_median(a: &[i32], b: &[i32]) -> Option<f64> {
    let mut all: Vec<i64> = a.iter().chain(b).map(|&x| i64::from(x)).collect();
    all.sort_unstable();
    let n = all.len();
    match n {
        0 => None,
        _ if n % 2 == 1 => Some(all[n / 2] as f64),
        _ => Some((all[n / 2 - 1] + all[n / 2]) as f64 / 2.0),
    }
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

    fn range(&mut self, lo: i64, hi: i64) -> i64 {
        lo + self.below((hi - lo + 1) as u64) as i64
    }
}

#[test]
fn example_1() {
    check(&[1, 3], &[2], 2.0);
}

#[test]
fn example_2() {
    check(&[1, 2], &[3, 4], 2.5);
}

#[test]
fn both_empty_is_none() {
    assert_eq!(median_of_sorted(&[], &[]), None);
}

#[test]
fn first_array_empty() {
    check(&[], &[1], 1.0);
    check(&[], &[2, 3], 2.5);
    check(&[], &[1, 2, 3, 4, 5], 3.0);
}

#[test]
fn second_array_empty() {
    check(&[1], &[], 1.0);
    check(&[2, 3], &[], 2.5);
    check(&[1, 2, 3, 4, 5], &[], 3.0);
}

#[test]
fn both_single() {
    check(&[1], &[2], 1.5);
    check(&[5], &[5], 5.0);
}

#[test]
fn even_total_needs_float_division_trap() {
    // Integer division would give 2 and -2 (or -3 with floor).
    check(&[1, 2], &[3, 4], 2.5);
    check(&[-3], &[-2], -2.5);
    check(&[0, 0], &[0, 1], 0.0);
    check(&[0], &[1], 0.5);
}

#[test]
fn all_of_one_before_all_of_other() {
    check(&[1, 2, 3], &[10, 11, 12, 13], 10.0);
    check(&[10, 11, 12, 13], &[1, 2, 3], 10.0);
    check(&[1, 2], &[3, 4, 5, 6], 3.5);
}

#[test]
fn interleaved() {
    check(&[1, 3, 5, 7], &[2, 4, 6, 8], 4.5);
    check(&[1, 3, 5, 7, 9], &[2, 4, 6, 8], 5.0);
}

#[test]
fn duplicates_across_arrays() {
    check(&[1, 1, 1], &[1, 1, 1], 1.0);
    check(&[1, 2, 2], &[2, 2, 3], 2.0);
}

#[test]
fn very_unbalanced_lengths() {
    // The shorter slice must be the one binary-searched, or indexes go out of range.
    let long: Vec<i32> = (0..1000).collect();
    check(&[500], &long, 500.0);
    check(&long, &[-1], 499.0);
    check(&long, &[2000, 2001], 500.5);
}

#[test]
fn extreme_values_sum_without_overflow() {
    check(&[1_000_000], &[1_000_000], 1_000_000.0);
    check(&[-1_000_000], &[-1_000_000], -1_000_000.0);
    check(&[-1_000_000], &[1_000_000], 0.0);
    check(&[999_999], &[1_000_000], 999_999.5);
}

#[test]
fn i32_limits_overflow_trap() {
    // i32::MAX + i32::MAX overflows an i32 sum.
    check(&[i32::MAX], &[i32::MAX], f64::from(i32::MAX));
    check(&[i32::MIN], &[i32::MIN], f64::from(i32::MIN));
    check(&[i32::MIN], &[i32::MAX], -0.5);
    check(&[i32::MAX - 1], &[i32::MAX], f64::from(i32::MAX) - 0.5);
    check(&[i32::MIN, i32::MIN], &[i32::MAX, i32::MAX], -0.5);
    check(&[i32::MIN, 0, i32::MAX], &[], 0.0);
}

#[test]
fn negatives() {
    check(&[-5, -3, -1], &[-4, -2], -3.0);
}

#[test]
fn argument_order_does_not_matter() {
    let a = [1, 4, 9, 16];
    let b = [2, 3, 5, 7, 11, 13];
    assert_eq!(median_of_sorted(&a, &b), median_of_sorted(&b, &a));
}

#[test]
fn accepts_vec_and_array_slices() {
    let a = vec![1, 2, 3];
    let b = [4, 5];
    check(&a, &b, 3.0);
    check(&a[1..], &b[..1], 3.0);
    check(&a[..2], &b[..1], 2.0);
}

#[test]
fn randomized_vs_brute() {
    let mut rng = Rng(0x3401_7777_1111);
    for _ in 0..800 {
        let m = rng.below(9) as usize;
        let n = rng.below(9) as usize;
        let (lo, hi) = match rng.below(3) {
            0 => (-5, 5),
            1 => (-1_000_000, 1_000_000),
            _ => (i64::from(i32::MIN), i64::from(i32::MAX)),
        };
        let a = sorted_vec(&mut rng, m, lo, hi);
        let b = sorted_vec(&mut rng, n, lo, hi);
        let context = format!("a = {a:?}, b = {b:?}");
        match brute_median(&a, &b) {
            Some(expected) => assert_close(median_of_sorted(&a, &b), expected, &context),
            None => assert_eq!(median_of_sorted(&a, &b), None, "{context}"),
        }
    }
}

#[test]
fn large_max_size_arrays() {
    // Big random pairs: merging is fine here, but the binary search must not
    // index out of range at any size.
    let mut rng = Rng(0x3402_9999_2222);
    for _ in 0..20 {
        let m = rng.range(0, 20_000) as usize;
        let n = rng.range(1, 20_000) as usize;
        let a = sorted_vec(&mut rng, m, -1_000_000, 1_000_000);
        let b = sorted_vec(&mut rng, n, -1_000_000, 1_000_000);
        let expected = brute_median(&a, &b).unwrap_or(f64::NAN);
        assert_close(median_of_sorted(&a, &b), expected, &format!("m = {m}, n = {n}"));
    }
    let a: Vec<i32> = (0..100_000).map(|i| 2 * i).collect();
    let b: Vec<i32> = (0..100_000).map(|i| 2 * i + 1).collect();
    check(&a, &b, 99_999.5);
}

#[test]
fn many_queries_on_prefixes() {
    // 2 * (N + 1) queries on prefixes a[..k], b[..l] of two N-element slices.
    // Walking to the middle costs (k + l) / 2 steps per query, ~10^7 in total
    // (merging costs twice that, sorting the concatenation ~12 times that);
    // the cut search is ~12 steps per query.
    let n = 2_500usize;
    let a: Vec<i32> = (0..n as i32).map(|i| 3 * i).collect();
    let b: Vec<i32> = (0..n as i32).map(|i| 3 * i + 1).collect();
    for l in [n, n - 1] {
        for k in 0..=n {
            let (a_part, b_part) = (&a[..k], &b[..l]);
            let total = k + l;
            let shared = k.min(l);
            // The idx-th smallest of a[..k] ∪ b[..l], computed arithmetically:
            // values 3i and 3i + 1 interleave below 3 * min(k, l).
            let nth = |idx: usize| -> i64 {
                let value = match idx {
                    _ if idx >= 2 * shared && k > l => a[idx - shared],
                    _ if idx >= 2 * shared => b[idx - shared],
                    _ if idx.is_multiple_of(2) => a[idx / 2],
                    _ => b[idx / 2],
                };
                i64::from(value)
            };
            let expected = if total % 2 == 1 {
                nth(total / 2) as f64
            } else {
                (nth(total / 2 - 1) + nth(total / 2)) as f64 / 2.0
            };
            assert_close(median_of_sorted(a_part, b_part), expected, &format!("n = {n}, k = {k}, l = {l}"));
        }
    }
}
