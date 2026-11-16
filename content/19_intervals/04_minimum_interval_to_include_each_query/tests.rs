// Tests for Minimum Interval to Include Each Query — shared by template.rs, solution.rs and your attempt.
use super::*;

/// O(n) per query: scan every interval.
fn brute_one(intervals: &[(i32, i32)], q: i32) -> Option<u32> {
    intervals
        .iter()
        .filter(|&&(left, right)| left <= q && q <= right)
        .map(|&(left, right)| (right - left + 1) as u32)
        .min()
}

fn brute_min_interval_sizes(intervals: &[(i32, i32)], queries: &[i32]) -> Vec<Option<u32>> {
    queries.iter().map(|&q| brute_one(intervals, q)).collect()
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
    assert_eq!(
        min_interval_sizes(vec![(1, 4), (2, 4), (3, 6), (4, 4)], &[2, 3, 4, 5]),
        [Some(3), Some(3), Some(1), Some(4)]
    );
}

#[test]
fn example_2() {
    assert_eq!(
        min_interval_sizes(vec![(2, 3), (2, 5), (1, 8), (20, 25)], &[2, 19, 5, 22]),
        [Some(2), None, Some(4), Some(6)]
    );
}

#[test]
fn no_intervals_all_none() {
    assert_eq!(min_interval_sizes(vec![], &[1, 2]), [None, None]);
}

#[test]
fn no_queries_empty_answer() {
    assert_eq!(min_interval_sizes(vec![(1, 3)], &[]), []);
    assert_eq!(min_interval_sizes(vec![], &[]), []);
}

#[test]
fn single_point_interval() {
    assert_eq!(min_interval_sizes(vec![(5, 5)], &[4, 5, 6]), [None, Some(1), None]);
}

#[test]
fn query_on_endpoints() {
    // Both ends are inclusive.
    assert_eq!(
        min_interval_sizes(vec![(3, 7)], &[3, 7, 2, 8]),
        [Some(5), Some(5), None, None]
    );
}

#[test]
fn no_interval_covers() {
    assert_eq!(min_interval_sizes(vec![(1, 2), (10, 12)], &[5, 13, 9]), [None, None, None]);
}

#[test]
fn answers_in_original_order() {
    // Trap: returning answers in sorted-query order.
    assert_eq!(
        min_interval_sizes(vec![(1, 1), (2, 3), (4, 7)], &[7, 1, 3, 4]),
        [Some(4), Some(1), Some(2), Some(4)]
    );
}

#[test]
fn duplicate_queries() {
    assert_eq!(
        min_interval_sizes(vec![(1, 10), (4, 5)], &[4, 4, 9, 4]),
        [Some(2), Some(2), Some(10), Some(2)]
    );
}

#[test]
fn expired_small_interval_must_be_dropped() {
    // Trap: (1, 2) (size 2) is smallest but no longer contains q = 5.
    assert_eq!(min_interval_sizes(vec![(1, 2), (1, 100)], &[1, 5]), [Some(2), Some(100)]);
}

#[test]
fn interval_not_started_yet() {
    // Trap: adding intervals whose left end is beyond the query.
    assert_eq!(min_interval_sizes(vec![(5, 6), (1, 20)], &[3, 5]), [Some(20), Some(2)]);
}

#[test]
fn nested_intervals() {
    assert_eq!(
        min_interval_sizes(vec![(1, 100), (10, 90), (20, 80), (40, 60)], &[50, 15, 85, 95, 30]),
        [Some(21), Some(81), Some(81), Some(100), Some(61)]
    );
}

#[test]
fn same_size_different_positions() {
    assert_eq!(
        min_interval_sizes(vec![(1, 3), (2, 4), (3, 5)], &[1, 3, 5]),
        [Some(3), Some(3), Some(3)]
    );
}

#[test]
fn bounds() {
    assert_eq!(
        min_interval_sizes(vec![(1, 10_000_000), (10_000_000, 10_000_000)], &[1, 10_000_000]),
        [Some(10_000_000), Some(1)]
    );
}

#[test]
fn queries_slice_is_borrowed() {
    let queries = vec![3, 1];
    let answers = min_interval_sizes(vec![(1, 3)], &queries);
    assert_eq!(answers, [Some(3), Some(3)]);
    assert_eq!(queries, [3, 1]);
}

#[test]
fn random_against_brute_force() {
    let mut rng = Rng(0x6A09_E667_F3BC_C908);
    for _ in 0..400 {
        let n = rng.range(0, 10) as usize;
        let m = rng.range(0, 10) as usize;
        let intervals: Vec<(i32, i32)> = (0..n)
            .map(|_| {
                let l = rng.range(1, 20);
                (l, l + rng.range(0, 8))
            })
            .collect();
        let queries: Vec<i32> = (0..m).map(|_| rng.range(1, 30)).collect();
        assert_eq!(
            min_interval_sizes(intervals.clone(), &queries),
            brute_min_interval_sizes(&intervals, &queries),
            "intervals: {intervals:?}, queries: {queries:?}"
        );
    }
}

#[test]
fn large_input() {
    // 2000 random intervals and queries.
    let mut rng = Rng(0xBB67_AE85_84CA_A73B);
    let intervals: Vec<(i32, i32)> = (0..2000)
        .map(|_| {
            let l = rng.range(1, 9_000_000);
            (l, l + rng.range(0, 1_000_000))
        })
        .collect();
    let queries: Vec<i32> = (0..2000).map(|_| rng.range(1, 10_000_000)).collect();
    let answers = min_interval_sizes(intervals.clone(), &queries);
    assert_eq!(answers.len(), queries.len());
    for (j, (&q, &got)) in queries.iter().zip(&answers).enumerate() {
        assert_eq!(got, brute_one(&intervals, q), "query #{j} = {q}");
    }
}

#[test]
fn large_nested_lazy_deletion() {
    // Intervals (i, 4001 - i): query q is contained in those with
    // i <= min(q, 4001 - q); the innermost has size 4001 - 2i + 1.
    let n = 2000;
    let intervals: Vec<(i32, i32)> = (1..=n).map(|i| (i, 4001 - i)).collect();
    let queries: Vec<i32> = (1..=n).map(|q| 2 * q).collect();
    let answers = min_interval_sizes(intervals, &queries);
    for (&q, &got) in queries.iter().zip(&answers) {
        let i = q.min(4001 - q);
        let expected = (i >= 1).then(|| (4001 - 2 * i + 1) as u32);
        assert_eq!(got, expected, "q = {q}");
    }
}

#[test]
fn large_queries_in_gaps() {
    // n - 2 short intervals with gaps between them, plus two long ones that
    // cover the gaps. Every query sits in a gap, so the answer is a long
    // interval: a scan per query, even one over intervals sorted by size that
    // stops at the first hit, checks ~n intervals each, 2.6 * 10^8 per call.
    // Sweeping the sorted queries with a heap is ~n log n.
    // Three calls, with the queries in different gaps and orders.
    let n: i32 = 16_000;
    let mut intervals: Vec<(i32, i32)> = (0..n - 2).map(|i| (100 * i + 1, 100 * i + 50)).collect();
    intervals.push((1, 6_000_000));
    intervals.push((1, 10_000_000));
    for (step, offset) in [(7_919, 75), (9_973, 51), (1, 100)] {
        let queries: Vec<i32> = (0..n).map(|j| 100 * (j * step % (n - 2)) + offset).collect();
        let answers = min_interval_sizes(intervals.clone(), &queries);
        for (&q, &got) in queries.iter().zip(&answers) {
            let expected = if q <= 6_000_000 { 6_000_000 } else { 10_000_000 };
            assert_eq!(got, Some(expected), "q = {q}, {n} intervals, {n} queries");
        }
    }
}
