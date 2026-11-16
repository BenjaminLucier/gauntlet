// Tests for House Robber II — shared by template.rs, solution.rs and your attempt.
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

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }

    fn cash(&mut self, max: u32) -> u32 {
        u32::try_from(self.below(u64::from(max) + 1)).expect("below max + 1")
    }
}

/// Brute force: every subset with no two circularly-adjacent houses.
fn brute_rob(houses: &[u32]) -> u64 {
    let n = houses.len();
    (0u32..1 << n)
        .filter(|mask| mask & (mask >> 1) == 0)
        .filter(|mask| n < 2 || !(mask & 1 == 1 && mask >> (n - 1) & 1 == 1))
        .map(|mask| {
            (0..n)
                .filter(|&i| mask >> i & 1 == 1)
                .map(|i| u64::from(houses[i]))
                .sum::<u64>()
        })
        .max()
        .unwrap_or(0)
}

/// Two-pass table DP oracle.
fn table_rob(houses: &[u32]) -> u64 {
    let line = |a: &[u32]| {
        let mut dp = vec![0u64; a.len() + 2];
        for (i, &x) in a.iter().enumerate() {
            dp[i + 2] = dp[i + 1].max(dp[i] + u64::from(x));
        }
        dp[a.len() + 1]
    };
    let n = houses.len();
    line(&houses[1..]).max(line(&houses[..n - 1]))
}

#[test]
fn example_1() {
    assert_eq!(rob_circle(&[2, 3, 2]), 3);
}

#[test]
fn example_2() {
    assert_eq!(rob_circle(&[1, 2, 3, 1]), 4);
}

#[test]
fn example_3() {
    assert_eq!(rob_circle(&[1, 2, 3]), 3);
}

#[test]
fn empty_circle() {
    assert_eq!(rob_circle(&[]), 0);
}

#[test]
fn single_house() {
    // Trap: both "all but first" and "all but last" are empty, yet the answer
    // is the house itself.
    assert_eq!(rob_circle(&[5]), 5);
    assert_eq!(rob_circle(&[0]), 0);
    assert_eq!(rob_circle(&[u32::MAX]), u64::from(u32::MAX));
}

#[test]
fn two_houses_are_adjacent() {
    assert_eq!(rob_circle(&[4, 9]), 9);
    assert_eq!(rob_circle(&[9, 4]), 9);
}

#[test]
fn first_and_last_conflict() {
    // Linear House Robber would take 100 + 100 at both ends.
    assert_eq!(rob_circle(&[100, 1, 1, 100]), 101);
    assert_eq!(rob_circle(&[100, 1, 50, 1, 100]), 150);
}

#[test]
fn best_uses_first_house() {
    assert_eq!(rob_circle(&[50, 1, 1, 40, 1]), 90);
}

#[test]
fn best_uses_last_house() {
    assert_eq!(rob_circle(&[1, 40, 1, 1, 50]), 90);
}

#[test]
fn all_zero() {
    assert_eq!(rob_circle(&[0; 7]), 0);
}

#[test]
fn all_equal_odd_and_even_length() {
    // Odd n: at most (n - 1) / 2 houses on a circle.
    assert_eq!(rob_circle(&[1000; 5]), 2000);
    assert_eq!(rob_circle(&[1000; 6]), 3000);
    assert_eq!(rob_circle(&[1000; 3]), 1000);
}

#[test]
fn overflow_when_summing_in_u32() {
    let max = u64::from(u32::MAX);
    assert_eq!(rob_circle(&[u32::MAX; 4]), 2 * max);
    assert_eq!(rob_circle(&[u32::MAX, 0, u32::MAX, 0, u32::MAX]), 2 * max);
}

#[test]
fn random_small_match_brute_force() {
    let mut rng = Rng(0x0123_4567_89AB_CDEF);
    for _ in 0..400 {
        let n = rng.below(16) as usize;
        let max = [5, 1000, u32::MAX][rng.below(3) as usize];
        let houses: Vec<u32> = (0..n).map(|_| rng.cash(max)).collect();
        assert_eq!(rob_circle(&houses), brute_rob(&houses), "houses = {houses:?}");
    }
}

#[test]
fn large_odd_circle_all_max() {
    // 99_999 houses on a circle: at most 49_999 can be robbed.
    let max = u64::from(u32::MAX);
    assert_eq!(rob_circle(&vec![u32::MAX; 99_999]), 49_999 * max);
}

#[test]
fn large_even_circle_all_max() {
    let max = u64::from(u32::MAX);
    assert_eq!(rob_circle(&vec![u32::MAX; 100_000]), 50_000 * max);
}

#[test]
fn large_random_matches_two_pass_table() {
    let mut rng = Rng(777);
    for max in [1000, u32::MAX] {
        let houses: Vec<u32> = (0..100_000).map(|_| rng.cash(max)).collect();
        assert_eq!(rob_circle(&houses), table_rob(&houses), "max = {max}");
    }
}
