// Tests for K Closest Points to Origin — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::collections::BTreeMap;

type Point = (i32, i32);

fn dist(&(x, y): &Point) -> u64 {
    let (x, y) = (u64::from(x.unsigned_abs()), u64::from(y.unsigned_abs()));
    x * x + y * y
}

/// Validates the property directly: `out` is a sub-multiset of `points` of
/// size min(k, n), and no excluded point is strictly closer than an included one.
fn check(points: &[Point], k: usize, out: &[Point]) {
    let ctx = || format!("points={points:?}, k={k}, out={out:?}");
    assert_eq!(out.len(), k.min(points.len()), "wrong number of points; {}", ctx());
    let mut pool: BTreeMap<Point, usize> = BTreeMap::new();
    for &p in points {
        *pool.entry(p).or_default() += 1;
    }
    for p in out {
        match pool.get_mut(p) {
            Some(c) if *c > 0 => *c -= 1,
            _ => panic!("{p:?} returned more often than it appears in the input; {}", ctx()),
        }
    }
    let max_in = out.iter().map(dist).max().unwrap_or(0);
    let min_out = pool
        .iter()
        .filter(|(_, c)| **c > 0)
        .map(|(p, _)| dist(p))
        .min()
        .unwrap_or(u64::MAX);
    assert!(
        max_in <= min_out,
        "an excluded point (dist^2={min_out}) is closer than an included one (dist^2={max_in}); {}",
        ctx()
    );
}

fn sorted(mut v: Vec<Point>) -> Vec<Point> {
    v.sort_unstable();
    v
}

fn run(points: &[Point], k: usize) {
    let out = k_closest(points, k);
    check(points, k, &out);
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
    assert_eq!(k_closest(&[(1, 3), (-2, 2)], 1), vec![(-2, 2)]);
}

#[test]
fn example_2() {
    let out = k_closest(&[(3, 3), (5, -1), (-2, 4)], 2);
    assert_eq!(sorted(out), vec![(-2, 4), (3, 3)]);
}

#[test]
fn example_k_larger_than_n_returns_everything() {
    assert_eq!(k_closest(&[(1, 1)], 5), vec![(1, 1)]);
}

#[test]
fn example_k_zero_returns_nothing() {
    assert!(k_closest(&[(1, 1)], 0).is_empty());
}

#[test]
fn empty_input() {
    assert!(k_closest(&[], 0).is_empty());
    assert!(k_closest(&[], 3).is_empty());
}

#[test]
fn single_point() {
    assert_eq!(k_closest(&[(7, -7)], 1), vec![(7, -7)]);
}

#[test]
fn k_equals_n_returns_everything() {
    let p = [(1, 2), (-3, 4), (0, 0), (5, 5)];
    assert_eq!(sorted(k_closest(&p, 4)), sorted(p.to_vec()));
}

#[test]
fn origin_is_closest() {
    assert_eq!(k_closest(&[(1, 0), (0, 0), (0, -1), (2, 2)], 1), vec![(0, 0)]);
}

#[test]
fn manhattan_distance_is_wrong() {
    // Euclidean: (2,2) -> 8 < (3,0) -> 9. Manhattan would pick (3,0).
    assert_eq!(k_closest(&[(3, 0), (2, 2)], 1), vec![(2, 2)]);
}

#[test]
fn sign_does_not_matter() {
    // (-1,-1) (2) is closer than (2,0) (4) — sorting by raw x or x+y is wrong.
    assert_eq!(k_closest(&[(2, 0), (-1, -1), (-3, 0)], 1), vec![(-1, -1)]);
}

#[test]
fn duplicate_points() {
    let out = k_closest(&[(1, 1), (5, 5), (1, 1)], 2);
    assert_eq!(sorted(out), vec![(1, 1), (1, 1)]);
}

#[test]
fn ties_at_the_boundary_accept_either() {
    let p = [(1, 0), (0, 1), (-1, 0), (5, 5)];
    run(&p, 2);
    run(&p, 3);
}

#[test]
fn extreme_coordinates() {
    let p = [(10_000, 10_000), (-10_000, -9_999), (10_000, -10_000), (-9_999, 10_000)];
    let out = k_closest(&p, 2);
    assert_eq!(sorted(out), sorted(vec![(-10_000, -9_999), (-9_999, 10_000)]));
}

#[test]
fn overflow_in_squared_distance() {
    // 46_341² already exceeds i32::MAX, and i32::MIN² + i32::MIN² = 2^63
    // doesn't even fit in i64 — only u64 holds every squared distance.
    let p = [(i32::MIN, i32::MIN), (46_341, 0), (i32::MAX, i32::MAX), (0, -46_340)];
    assert_eq!(k_closest(&p, 1), vec![(0, -46_340)]);
    assert_eq!(sorted(k_closest(&p, 2)), vec![(0, -46_340), (46_341, 0)]);
    let out = k_closest(&p, 3);
    check(&p, 3, &out);
    assert!(out.contains(&(i32::MAX, i32::MAX)), "out={out:?}");
}

#[test]
fn input_is_left_untouched() {
    let p = vec![(5, 5), (1, 1), (3, 3)];
    let _ = k_closest(&p, 1);
    assert_eq!(p, vec![(5, 5), (1, 1), (3, 3)]);
}

#[test]
fn randomized_property_check() {
    let mut rng = Rng(0x0666_1234);
    for _ in 0..400 {
        let n = rng.range(0, 15) as usize;
        let c = if rng.range(0, 1) == 0 { 3 } else { 100 };
        let p: Vec<Point> = (0..n).map(|_| (rng.range(-c, c), rng.range(-c, c))).collect();
        let k = rng.range(0, n as i32 + 2) as usize;
        run(&p, k);
    }
}

#[test]
fn randomized_full_range_coordinates() {
    let mut rng = Rng(0x0666_ffff);
    for _ in 0..200 {
        let n = rng.range(1, 12) as usize;
        let p: Vec<Point> = (0..n).map(|_| (rng.range(i32::MIN, i32::MAX), rng.range(i32::MIN, i32::MAX))).collect();
        let k = rng.range(0, n as i32) as usize;
        run(&p, k);
    }
}

#[test]
fn large_input() {
    let mut rng = Rng(0x00ab_cdef_0666);
    let p: Vec<Point> = (0..10_000)
        .map(|_| (rng.range(-10_000, 10_000), rng.range(-10_000, 10_000)))
        .collect();
    for k in [1, 10, 5_000, 10_000] {
        run(&p, k);
    }
}

#[test]
fn large_concentric_rings() {
    // Points on axes at increasing radius, listed farthest-first.
    let mut p = Vec::new();
    for r in (1..=2_500).rev() {
        p.extend([(r, 0), (0, -r), (-r, 0), (0, r)]);
    }
    let out = k_closest(&p, 40);
    check(&p, 40, &out);
    assert!(out.iter().all(|q| dist(q) <= 100), "out={out:?}");
}
