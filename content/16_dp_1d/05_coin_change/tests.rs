// Tests for Coin Change — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::collections::VecDeque;

/// Tiny deterministic PRNG (xorshift64) so random tests are reproducible.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    /// Uniform in `lo..=hi`.
    fn range(&mut self, lo: u32, hi: u32) -> u32 {
        let span = u64::from(hi - lo) + 1;
        lo + u32::try_from(self.next() % span).expect("below span")
    }

    fn distinct_coins(&mut self, count: usize, lo: u32, hi: u32) -> Vec<u32> {
        let mut coins = Vec::new();
        while coins.len() < count {
            let c = self.range(lo, hi);
            if !coins.contains(&c) {
                coins.push(c);
            }
        }
        coins
    }
}

/// Exhaustive search: for each denomination in turn, try every count of it.
fn brute_min(coins: &[u32], amount: u64) -> Option<usize> {
    if amount == 0 {
        return Some(0);
    }
    let (&c, rest) = coins.split_first()?;
    let c = u64::from(c);
    (0..=amount / c)
        .filter_map(|k| brute_min(rest, amount - k * c).map(|m| m + k as usize))
        .min()
}

/// BFS oracle: each coin is an edge; the shortest path from 0 to `amount`.
fn bfs_min(coins: &[u32], amount: u32) -> Option<usize> {
    let amount = u64::from(amount);
    let mut dist: Vec<Option<usize>> = vec![None; amount as usize + 1];
    dist[0] = Some(0);
    let mut queue = VecDeque::from([0u64]);
    while let Some(a) = queue.pop_front() {
        let d = dist[a as usize].expect("queued amounts are reached");
        for &c in coins {
            let next = a + u64::from(c);
            if next <= amount && dist[next as usize].is_none() {
                dist[next as usize] = Some(d + 1);
                queue.push_back(next);
            }
        }
    }
    dist[amount as usize]
}

#[test]
fn example_1() {
    assert_eq!(coin_change(&[1, 2, 5], 11), Some(3));
}

#[test]
fn example_2() {
    assert_eq!(coin_change(&[2], 3), None);
}

#[test]
fn example_3() {
    assert_eq!(coin_change(&[1], 0), Some(0));
}

#[test]
fn amount_zero_needs_no_coins_even_if_unusable() {
    assert_eq!(coin_change(&[7, 11], 0), Some(0));
    assert_eq!(coin_change(&[], 0), Some(0));
}

#[test]
fn no_coins_cannot_pay() {
    assert_eq!(coin_change(&[], 1), None);
    assert_eq!(coin_change(&[], 10_000), None);
}

#[test]
fn exact_single_coin() {
    assert_eq!(coin_change(&[3, 7, 10], 10), Some(1));
}

#[test]
fn greedy_largest_first_fails() {
    // Greedy: 4 + 1 + 1 = 3 coins. Optimal: 3 + 3 = 2 coins.
    assert_eq!(coin_change(&[1, 3, 4], 6), Some(2));
    // Greedy: 25 + 1*5 = 6 coins. Optimal: 10 + 10 + 10 = 3 coins.
    assert_eq!(coin_change(&[1, 10, 25], 30), Some(3));
}

#[test]
fn greedy_gets_stuck_but_answer_exists() {
    // Greedy takes 5, leaving 1 which can't be paid. Optimal: 3 + 3.
    assert_eq!(coin_change(&[3, 5], 6), Some(2));
}

#[test]
fn unreachable_amounts() {
    assert_eq!(coin_change(&[2, 4, 6], 7), None);
    assert_eq!(coin_change(&[5, 10], 3), None);
}

#[test]
fn coins_bigger_than_amount_ignored() {
    assert_eq!(coin_change(&[100, 2, 50], 6), Some(3));
}

#[test]
fn huge_coin_no_overflow() {
    // Trap: `a + coin` with coins near u32::MAX overflows.
    assert_eq!(coin_change(&[u32::MAX], 10_000), None);
    assert_eq!(coin_change(&[u32::MAX, 1], 10_000), Some(10_000));
    assert_eq!(coin_change(&[u32::MAX, u32::MAX - 1, 3], 9), Some(3));
}

#[test]
fn unsorted_and_duplicate_coins() {
    assert_eq!(coin_change(&[5, 1, 2], 11), Some(3));
    assert_eq!(coin_change(&[2, 2, 3, 3], 7), Some(3));
}

#[test]
fn random_small_match_brute_force() {
    let mut rng = Rng(0x1357_9BDF_2468_ACE0);
    for _ in 0..400 {
        let count = rng.range(0, 4) as usize;
        let coins = rng.distinct_coins(count, 1, 15);
        let amount = rng.range(0, 40);
        assert_eq!(
            coin_change(&coins, amount),
            brute_min(&coins, u64::from(amount)),
            "coins = {coins:?}, amount = {amount}"
        );
    }
}

#[test]
fn known_hard_case() {
    // Plain recursion without memoization times out on this one.
    assert_eq!(coin_change(&[186, 419, 83, 408], 6249), Some(20));
}

#[test]
fn large_amount_all_ones() {
    assert_eq!(coin_change(&[1], 10_000), Some(10_000));
}

#[test]
fn large_unreachable() {
    assert_eq!(coin_change(&[2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24], 9_999), None);
}

#[test]
fn large_random_match_bfs() {
    let mut rng = Rng(31337);
    for _ in 0..3 {
        let count = rng.range(1, 12) as usize;
        let coins = rng.distinct_coins(count, 2, 500);
        let amount = rng.range(9_000, 10_000);
        assert_eq!(
            coin_change(&coins, amount),
            bfs_min(&coins, amount),
            "coins = {coins:?}, amount = {amount}"
        );
    }
}
