// Tests for Segment Tree — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::cell::Cell;

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
    fn range(&mut self, lo: i64, hi: i64) -> i64 {
        lo + (self.next() % (hi - lo + 1) as u64) as i64
    }
}

const P: u64 = 1_000_000_007;

/// Affine maps `x -> a*x + b (mod P)`, combined as "apply the left one, then
/// the right one". Associative but not commutative.
struct Affine;

impl Monoid for Affine {
    type Item = (u64, u64);

    fn identity() -> (u64, u64) {
        (1, 0)
    }

    fn combine((a1, b1): (u64, u64), (a2, b2): (u64, u64)) -> (u64, u64) {
        (a1 * a2 % P, (b1 * a2 + b2) % P)
    }
}

/// The leftmost `Some` in the range. Also not commutative.
struct First;

impl Monoid for First {
    type Item = Option<char>;

    fn identity() -> Option<char> {
        None
    }

    fn combine(a: Option<char>, b: Option<char>) -> Option<char> {
        a.or(b)
    }
}

/// A user-defined monoid over a different item type.
struct MaxU8;

impl Monoid for MaxU8 {
    type Item = u8;

    fn identity() -> u8 {
        0
    }

    fn combine(a: u8, b: u8) -> u8 {
        a.max(b)
    }
}

thread_local! {
    static COMBINES: Cell<usize> = const { Cell::new(0) };
}

/// `combine` calls made so far on this thread (by `CountedSum`).
fn combines() -> usize {
    COMBINES.with(Cell::get)
}

/// Sums that count every `combine` call, to measure the work per operation.
struct CountedSum;

impl Monoid for CountedSum {
    type Item = i64;

    fn identity() -> i64 {
        0
    }

    fn combine(a: i64, b: i64) -> i64 {
        COMBINES.with(|c| c.set(c.get() + 1));
        a + b
    }
}

/// Size of the counting tests: `log2(N) = 16`.
const N: usize = 1 << 16;

fn brute_query<M: Monoid>(values: &[M::Item], range: Range<usize>) -> M::Item {
    values[range]
        .iter()
        .fold(M::identity(), |acc, &x| M::combine(acc, x))
}

/// Every range of every length, against the brute fold.
fn check_all_ranges<M: Monoid>(values: &[M::Item])
where
    M::Item: PartialEq + std::fmt::Debug,
{
    let st = SegmentTree::<M>::new(values);
    assert_eq!(st.len(), values.len());
    for start in 0..=values.len() {
        for end in start..=values.len() {
            assert_eq!(
                st.query(start..end),
                brute_query::<M>(values, start..end),
                "query({start}..{end}) on {values:?}"
            );
        }
    }
}

#[test]
fn readme_example() {
    let mut sums = SegmentTree::<Sum>::new(&[5, 3, -2, 8, 1]);
    assert_eq!(sums.len(), 5);
    assert_eq!(sums.query(0..5), 15);
    assert_eq!(sums.query(1..4), 9);
    assert_eq!(sums.query(2..2), 0);
    sums.set(2, 10);
    assert_eq!(sums.query(1..4), 21);

    let mins = SegmentTree::<Min>::new(&[5, 3, -2, 8, 1]);
    assert_eq!(mins.query(0..2), 3);
    assert_eq!(mins.query(3..5), 1);
    assert_eq!(mins.query(4..4), i64::MAX);
}

#[test]
fn sum_and_min_monoids() {
    assert_eq!(Sum::identity(), 0);
    assert_eq!(Sum::combine(-4, 9), 5);
    assert_eq!(Min::identity(), i64::MAX);
    assert_eq!(Min::combine(-4, 9), -4);
    assert_eq!(Min::combine(i64::MIN, Min::identity()), i64::MIN);
}

#[test]
fn empty_tree() {
    let st = SegmentTree::<Sum>::new(&[]);
    assert_eq!(st.len(), 0);
    assert!(st.is_empty());
    assert_eq!(st.query(0..0), 0);
    assert_eq!(SegmentTree::<Min>::new(&[]).query(0..0), i64::MAX);
}

#[test]
fn single_element() {
    let mut st = SegmentTree::<Min>::new(&[42]);
    assert!(!st.is_empty());
    assert_eq!(st.query(0..1), 42);
    assert_eq!(st.query(0..0), i64::MAX);
    assert_eq!(st.query(1..1), i64::MAX);
    st.set(0, -7);
    assert_eq!(st.query(0..1), -7);
}

#[test]
fn every_range_of_small_arrays() {
    // Odd lengths and non-powers of two are where index arithmetic slips.
    let mut rng = Rng(7);
    for n in 0..=33 {
        let v: Vec<i64> = (0..n).map(|_| rng.range(-50, 50)).collect();
        check_all_ranges::<Sum>(&v);
        check_all_ranges::<Min>(&v);
    }
}

#[test]
fn combine_order_matters() {
    // Swapping the operands anywhere changes these answers.
    let mut rng = Rng(99);
    for n in [1, 2, 3, 5, 7, 8, 13, 16, 31] {
        let maps: Vec<(u64, u64)> = (0..n).map(|_| (rng.next() % P, rng.next() % P)).collect();
        check_all_ranges::<Affine>(&maps);
        let firsts: Vec<Option<char>> = (0..n)
            .map(|i| (rng.below(3) == 0).then_some((b'a' + i as u8) as char))
            .collect();
        check_all_ranges::<First>(&firsts);
    }
    let st = SegmentTree::<First>::new(&[None, Some('x'), None, Some('y')]);
    assert_eq!(st.query(0..4), Some('x'));
    assert_eq!(st.query(2..4), Some('y'));
    assert_eq!(st.query(2..3), None);
}

#[test]
fn set_replaces_rather_than_adds() {
    let mut st = SegmentTree::<Sum>::new(&[1, 1, 1, 1]);
    st.set(1, 5);
    st.set(1, 5);
    assert_eq!(st.query(0..4), 8, "setting the same value twice is idempotent");
    st.set(3, 0);
    assert_eq!(st.query(2..4), 1);
}

#[test]
fn min_after_raising_the_minimum() {
    // The old minimum must disappear from every ancestor.
    let mut st = SegmentTree::<Min>::new(&[4, 9, 1, 7, 6, 8]);
    assert_eq!(st.query(0..6), 1);
    st.set(2, 100);
    assert_eq!(st.query(0..6), 4);
    assert_eq!(st.query(1..5), 6);
    st.set(0, 50);
    assert_eq!(st.query(0..6), 6);
}

#[test]
fn extreme_values() {
    let big = 1_000_000_000_000_000;
    let mut sums = SegmentTree::<Sum>::new(&[big, -big, big, big, -big]);
    assert_eq!(sums.query(0..5), big);
    assert_eq!(sums.query(2..4), 2 * big);
    sums.set(1, big);
    assert_eq!(sums.query(0..4), 4 * big);
    let mins = SegmentTree::<Min>::new(&[i64::MAX, i64::MIN, 0]);
    assert_eq!(mins.query(0..1), i64::MAX);
    assert_eq!(mins.query(0..3), i64::MIN);
}

#[test]
fn other_item_types() {
    let mut st = SegmentTree::<MaxU8>::new(&[3, 200, 17, 255, 0]);
    assert_eq!(st.query(0..3), 200);
    assert_eq!(st.query(2..5), 255);
    st.set(3, 1);
    assert_eq!(st.query(2..5), 17);
}

#[test]
#[should_panic(expected = "out of bounds")]
fn query_past_the_end_panics() {
    let st = SegmentTree::<Sum>::new(&[1, 2, 3]);
    st.query(1..4);
}

#[test]
#[should_panic(expected = "out of bounds")]
fn reversed_range_panics() {
    let st = SegmentTree::<Sum>::new(&[1, 2, 3]);
    #[allow(clippy::reversed_empty_ranges)]
    st.query(2..1);
}

#[test]
#[should_panic(expected = "out of bounds")]
fn set_past_the_end_panics() {
    let mut st = SegmentTree::<Min>::new(&[1, 2, 3]);
    st.set(3, 0);
}

#[test]
fn random_operations_match_naive() {
    let mut rng = Rng(0xDEAD_BEEF);
    for round in 0..60 {
        let n = 1 + rng.below(70);
        let mut v: Vec<i64> = (0..n).map(|_| rng.range(-1000, 1000)).collect();
        let mut maps: Vec<(u64, u64)> = (0..n).map(|_| (rng.next() % P, rng.next() % P)).collect();
        let mut sums = SegmentTree::<Sum>::new(&v);
        let mut mins = SegmentTree::<Min>::new(&v);
        let mut affine = SegmentTree::<Affine>::new(&maps);
        for step in 0..150 {
            if rng.below(2) == 0 {
                let i = rng.below(n);
                let x = rng.range(-1000, 1000);
                v[i] = x;
                sums.set(i, x);
                mins.set(i, x);
                maps[i] = (rng.next() % P, rng.next() % P);
                affine.set(i, maps[i]);
            } else {
                let a = rng.below(n + 1);
                let b = rng.below(n + 1);
                let r = a.min(b)..a.max(b);
                let ctx = format!("round {round} step {step}: query({r:?}) on {v:?}");
                assert_eq!(sums.query(r.clone()), brute_query::<Sum>(&v, r.clone()), "sum {ctx}");
                assert_eq!(mins.query(r.clone()), brute_query::<Min>(&v, r.clone()), "min {ctx}");
                assert_eq!(affine.query(r.clone()), brute_query::<Affine>(&maps, r), "affine {ctx}");
            }
        }
    }
}

#[test]
fn large_input_at_the_size_limit() {
    // Correctness at n = 2 * 10^5, with sums near the i64 range: 400 sets and
    // 4000 sum queries. (The counting tests below check the work per call.)
    let n = 200_000;
    let mut rng = Rng(2024);
    let mut v: Vec<i64> = (0..n).map(|_| rng.range(-1_000_000_000, 1_000_000_000)).collect();
    let mut st = SegmentTree::<Sum>::new(&v);
    let original = v.clone();
    let mins = SegmentTree::<Min>::new(&original);
    for _round in 0..4 {
        for _ in 0..100 {
            let i = rng.below(n);
            let x = rng.range(-1_000_000_000, 1_000_000_000);
            v[i] = x;
            st.set(i, x);
        }
        // Prefix sums of the current array answer every sum query exactly.
        let mut prefix = vec![0i64; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + v[i];
        }
        for _ in 0..1_000 {
            let a = rng.below(n + 1);
            let b = rng.below(n + 1);
            let (l, r) = (a.min(b), a.max(b));
            assert_eq!(st.query(l..r), prefix[r] - prefix[l], "query({l}..{r})");
        }
    }
    // Min over a static array: every short window, checked directly.
    for (i, w) in original.windows(3).enumerate().step_by(7) {
        let expected = *w.iter().min().expect("window of 3");
        assert_eq!(mins.query(i..i + 3), expected, "min query({i}..{})", i + 3);
    }
    assert_eq!(mins.query(0..n), *original.iter().min().expect("non-empty"));
}

#[test]
fn build_is_linear() {
    // Building by calling `set` for every element makes n * log2(n) combines.
    let values: Vec<i64> = (0..N as i64).collect();
    let before = combines();
    let st = SegmentTree::<CountedSum>::new(&values);
    let calls = combines() - before;
    assert!(calls <= 2 * N, "{calls} combines to build a tree of n = {N}");
    assert_eq!(st.query(0..N), (N as i64 - 1) * N as i64 / 2);
}

#[test]
fn set_does_not_rebuild_the_tree() {
    let mut rng = Rng(0x5E7_5E7);
    let mut v: Vec<i64> = (0..N as i64).collect();
    let mut st = SegmentTree::<CountedSum>::new(&v);
    for step in 0..1_000 {
        let i = rng.below(N);
        let x = rng.range(-1_000_000, 1_000_000);
        v[i] = x;
        let before = combines();
        st.set(i, x);
        let calls = combines() - before;
        assert!(calls <= 40, "set({i}, {x}) made {calls} combines, n = {N} (step {step})");
    }
    assert_eq!(st.query(0..N), v.iter().sum::<i64>());
}

#[test]
fn query_does_not_scan_the_range() {
    let mut rng = Rng(0x0BE_7EE);
    let v: Vec<i64> = (0..N).map(|_| rng.range(-1_000_000, 1_000_000)).collect();
    let mut prefix = vec![0i64; N + 1];
    for i in 0..N {
        prefix[i + 1] = prefix[i] + v[i];
    }
    let st = SegmentTree::<CountedSum>::new(&v);
    for step in 0..1_000 {
        let a = rng.below(N + 1);
        let b = rng.below(N + 1);
        let (l, r) = (a.min(b), a.max(b));
        let before = combines();
        let got = st.query(l..r);
        let calls = combines() - before;
        assert_eq!(got, prefix[r] - prefix[l], "query({l}..{r}), n = {N}");
        assert!(calls <= 80, "query({l}..{r}) made {calls} combines, n = {N} (step {step})");
    }
}
