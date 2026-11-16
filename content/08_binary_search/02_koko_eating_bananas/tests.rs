// Tests for Koko Eating Bananas — shared by template.rs, solution.rs and your attempt.
use super::*;

fn check(piles: &[u32], h: u64, expected: Option<u32>) {
    assert_eq!(min_eating_speed(piles, h), expected, "piles = {piles:?}, h = {h}");
}

/// Hours needed at speed k, computed without any chance of overflow.
fn hours(piles: &[u32], k: u64) -> u64 {
    piles.iter().map(|&p| u64::from(p).div_ceil(k)).sum()
}

/// Oracle: try every speed from 1 upward.
fn brute_speed(piles: &[u32], h: u64) -> Option<u32> {
    if h < piles.len() as u64 {
        return None;
    }
    (1..).find(|&k| hours(piles, u64::from(k)) <= h)
}

/// Property check for inputs too large for the oracle: k works and k - 1 doesn't.
fn assert_minimal(piles: &[u32], h: u64, got: Option<u32>) {
    let Some(k) = got else {
        panic!("expected a speed for h = {h}, n = {}", piles.len());
    };
    assert!(k >= 1, "speed must be positive, got {k} (h = {h})");
    assert!(hours(piles, u64::from(k)) <= h, "speed {k} is too slow for h = {h}");
    if k > 1 {
        assert!(hours(piles, u64::from(k) - 1) > h, "speed {} also works for h = {h}", k - 1);
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

    fn range(&mut self, lo: u64, hi: u64) -> u64 {
        lo + self.below(hi - lo + 1)
    }
}

#[test]
fn example_1() {
    check(&[3, 6, 7, 11], 8, Some(4));
}

#[test]
fn example_2() {
    check(&[30, 11, 23, 4, 20], 5, Some(30));
}

#[test]
fn example_3() {
    check(&[30, 11, 23, 4, 20], 6, Some(23));
}

#[test]
fn example_4_impossible() {
    check(&[30, 11, 23, 4, 20], 4, None);
}

#[test]
fn no_piles_any_speed_works() {
    check(&[], 0, Some(1));
    check(&[], 100, Some(1));
}

#[test]
fn fewer_hours_than_piles_is_none() {
    check(&[1], 0, None);
    check(&[1, 1, 1], 2, None);
    check(&[u32::MAX; 10], 9, None);
    check(&[u32::MAX; 10], 10, Some(u32::MAX));
}

#[test]
fn single_pile_one_hour() {
    check(&[1_000_000_000], 1, Some(1_000_000_000));
}

#[test]
fn single_pile_many_hours() {
    check(&[10], 3, Some(4)); // ceil(10/4) = 3, ceil(10/3) = 4
    check(&[10], 10, Some(1));
    check(&[10], 1_000_000_000, Some(1));
}

#[test]
fn h_equals_n_needs_max_pile() {
    check(&[5, 1, 9, 3], 4, Some(9));
}

#[test]
fn plenty_of_time_speed_is_one() {
    check(&[3, 6, 7, 11], 1_000_000_000, Some(1));
    check(&[3, 6, 7, 11], 27, Some(1));
    check(&[3, 6, 7, 11], u64::MAX, Some(1));
}

#[test]
fn ceil_not_floor_trap() {
    // floor(7/4) + floor(6/4) = 2 would wrongly accept k = 4 for h = 3.
    check(&[7, 6], 3, Some(6));
}

#[test]
fn all_ones() {
    check(&[1, 1, 1, 1], 4, Some(1));
}

#[test]
fn huge_total_bananas() {
    // 10^13 bananas in total: hours at k = 1 do not fit in 32 bits.
    let piles = vec![1_000_000_000; 10_000];
    // ceil(10^9 / k) * 10^4 <= 10^9  <=>  ceil(10^9 / k) <= 10^5  <=>  k >= 10^4
    check(&piles, 1_000_000_000, Some(10_000));
}

#[test]
fn piles_near_max_value() {
    check(&[1_000_000_000, 999_999_999], 2, Some(1_000_000_000));
    check(&[1_000_000_000, 999_999_999], 3, Some(999_999_999));
}

#[test]
fn ceil_division_overflow_trap() {
    // (p + k - 1) / k overflows u32 here; div_ceil (or widening) doesn't.
    check(&[u32::MAX], 1, Some(u32::MAX));
    check(&[u32::MAX, u32::MAX - 1], 2, Some(u32::MAX));
    check(&[u32::MAX, u32::MAX - 1], 3, Some(u32::MAX - 1));
    check(&[u32::MAX], 2, Some(u32::MAX / 2 + 1));
    check(&[u32::MAX], u64::from(u32::MAX), Some(1));
}

#[test]
fn randomized_vs_brute() {
    let mut rng = Rng(0x3001_2468_ace0);
    for _ in 0..400 {
        let n = rng.range(0, 8) as usize;
        let piles: Vec<u32> = (0..n).map(|_| rng.range(1, 40) as u32).collect();
        let total: u64 = piles.iter().map(|&p| u64::from(p)).sum();
        // Occasionally go below n to cover the impossible case.
        let h = rng.range((n as u64).saturating_sub(2), total + 3);
        assert_eq!(
            min_eating_speed(&piles, h),
            brute_speed(&piles, h),
            "piles = {piles:?}, h = {h}"
        );
    }
}

#[test]
fn randomized_large_values_property() {
    let mut rng = Rng(0x3002_1357_0246);
    for _ in 0..100 {
        let n = rng.range(1, 50) as usize;
        let piles: Vec<u32> = (0..n).map(|_| rng.range(1, u64::from(u32::MAX)) as u32).collect();
        let h = rng.range(n as u64, 1 << 40);
        assert_minimal(&piles, h, min_eating_speed(&piles, h));
    }
}

#[test]
fn large_random_piles() {
    // A linear scan over speeds would need up to ~10^9 * 10^4 steps.
    let mut rng = Rng(0x3003_9876_5432);
    let piles: Vec<u32> = (0..10_000).map(|_| rng.range(1, 1_000_000_000) as u32).collect();
    for h in [10_000, 10_001, 123_456, 50_000_000, 1_000_000_000] {
        assert_minimal(&piles, h, min_eating_speed(&piles, h));
    }
    assert_eq!(min_eating_speed(&piles, 9_999), None);
}
