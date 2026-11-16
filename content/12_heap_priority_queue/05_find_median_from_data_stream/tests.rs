// Tests for Find Median from Data Stream — shared by template.rs, solution.rs and your attempt.
use super::*;

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-5
}

fn assert_median(mf: &MedianFinder, expected: f64) {
    match mf.median() {
        Some(m) => assert!(close(m, expected), "got {m}, expected {expected}"),
        None => panic!("got None, expected {expected}"),
    }
}

/// Adds each value and returns the median after every insertion.
fn medians(values: &[i32]) -> Vec<Option<f64>> {
    let mut mf = MedianFinder::new();
    values
        .iter()
        .map(|&v| {
            mf.add(v);
            mf.median()
        })
        .collect()
}

/// Naive model: sorted Vec.
fn brute_medians(values: &[i32]) -> Vec<f64> {
    let mut s: Vec<i32> = Vec::new();
    values
        .iter()
        .map(|&v| {
            let pos = s.partition_point(|&x| x < v);
            s.insert(pos, v);
            let n = s.len();
            if n % 2 == 1 {
                f64::from(s[n / 2])
            } else {
                (f64::from(s[n / 2 - 1]) + f64::from(s[n / 2])) / 2.0
            }
        })
        .collect()
}

fn assert_medians(values: &[i32], expected: &[f64]) {
    let got = medians(values);
    assert_eq!(got.len(), expected.len());
    for (i, (g, e)) in got.iter().zip(expected).enumerate() {
        let ok = g.is_some_and(|g| close(g, *e));
        assert!(ok, "after adding {:?}: got {g:?}, expected {e}", &values[..=i]);
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

    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        (i64::from(lo) + (self.next() % (i64::from(hi) - i64::from(lo) + 1) as u64) as i64) as i32
    }
}

#[test]
fn example_1() {
    let mut mf = MedianFinder::new();
    assert_eq!(mf.median(), None);
    mf.add(1);
    mf.add(2);
    assert_median(&mf, 1.5);
    mf.add(3);
    assert_median(&mf, 2.0);
}

#[test]
fn empty_has_no_median() {
    assert_eq!(MedianFinder::new().median(), None);
    assert_eq!(MedianFinder::default().median(), None);
}

#[test]
fn single_number() {
    assert_medians(&[42], &[42.0]);
    assert_medians(&[-7], &[-7.0]);
    assert_medians(&[0], &[0.0]);
}

#[test]
fn even_count_is_not_integer_division() {
    assert_medians(&[2, 3], &[2.0, 2.5]);
    assert_medians(&[-1, -2], &[-1.0, -1.5]);
}

#[test]
fn repeated_median_is_stable() {
    let mut mf = MedianFinder::new();
    mf.add(5);
    mf.add(10);
    for _ in 0..5 {
        assert_median(&mf, 7.5);
    }
}

#[test]
fn decreasing_input() {
    assert_medians(&[5, 4, 3, 2, 1], &[5.0, 4.5, 4.0, 3.5, 3.0]);
}

#[test]
fn increasing_input() {
    assert_medians(&[1, 2, 3, 4, 5, 6], &[1.0, 1.5, 2.0, 2.5, 3.0, 3.5]);
}

#[test]
fn duplicates() {
    assert_medians(&[3, 3, 3, 1, 1], &[3.0, 3.0, 3.0, 3.0, 3.0]);
    assert_medians(&[0, 0, 0, 0], &[0.0, 0.0, 0.0, 0.0]);
}

#[test]
fn new_value_lands_in_lower_half() {
    // A naive "push to whichever heap is smaller" without comparing values fails here.
    assert_medians(&[10, 20, 1], &[10.0, 15.0, 10.0]);
    assert_medians(&[10, 1, 20, 0], &[10.0, 5.5, 10.0, 5.5]);
}

#[test]
fn extremes() {
    assert_medians(&[100_000, -100_000], &[100_000.0, 0.0]);
    assert_medians(&[100_000, 100_000], &[100_000.0, 100_000.0]);
    assert_medians(&[-100_000, -99_999], &[-100_000.0, -99_999.5]);
}

#[test]
fn overflow_in_mean() {
    // i32::MAX + i32::MAX overflows i32: convert before adding.
    assert_medians(&[i32::MAX, i32::MAX], &[f64::from(i32::MAX), f64::from(i32::MAX)]);
    assert_medians(&[i32::MIN, i32::MIN], &[f64::from(i32::MIN), f64::from(i32::MIN)]);
    assert_medians(&[i32::MIN, i32::MAX], &[f64::from(i32::MIN), -0.5]);
    assert_medians(&[i32::MAX, i32::MAX - 1], &[f64::from(i32::MAX), f64::from(i32::MAX) - 0.5]);
}

#[test]
fn instances_are_independent() {
    let mut a = MedianFinder::new();
    let mut b = MedianFinder::new();
    a.add(1);
    b.add(100);
    a.add(2);
    assert_median(&a, 1.5);
    assert_median(&b, 100.0);
}

#[test]
fn median_through_shared_reference() {
    let mut mf = MedianFinder::new();
    mf.add(4);
    let r1 = &mf;
    let r2 = &mf;
    assert_eq!(r1.median(), r2.median());
}

#[test]
fn randomized_against_sorted_model() {
    let mut rng = Rng(0x0700_0700);
    for _ in 0..300 {
        let n = rng.range(1, 40) as usize;
        let c = match rng.range(0, 2) {
            0 => 5,
            1 => 100_000,
            _ => i32::MAX,
        };
        let values: Vec<i32> = (0..n).map(|_| rng.range(-c, c)).collect();
        assert_medians(&values, &brute_medians(&values));
    }
}

#[test]
fn large_random_stream() {
    // 6000 adds, a median after each. Copying and sorting the values on every
    // median is ~n log n per call, ~2 * 10^8 element operations in total; two
    // heaps answer each median in O(1).
    let mut rng = Rng(0xcafe_0070);
    let values: Vec<i32> = (0..6000).map(|_| rng.range(-1_000_000_000, 1_000_000_000)).collect();
    assert_medians(&values, &brute_medians(&values));
}

#[test]
fn large_sorted_streams() {
    // Monotone streams: every new value goes to the same end, so the halves
    // must be rebalanced on every add. Copying the values and running
    // select_nth on every median is ~n per call here: ~3.6 * 10^7 steps.
    let n = 6000;
    let asc: Vec<i32> = (1..=n).collect();
    let exp: Vec<f64> = (1..=n).map(|m| (f64::from(m) + 1.0) / 2.0).collect();
    assert_medians(&asc, &exp);
    let desc: Vec<i32> = (1..=n).rev().collect();
    let exp: Vec<f64> = (1..=n).map(|m| f64::from(n) - (f64::from(m) - 1.0) / 2.0).collect();
    assert_medians(&desc, &exp);
}
