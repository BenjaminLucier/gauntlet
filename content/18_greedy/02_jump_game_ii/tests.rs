// Tests for Jump Game II — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::collections::VecDeque;

/// BFS over indices, one edge per possible jump: O(n²) but obviously right.
fn brute_min_jumps(jumps: &[usize]) -> Option<usize> {
    let n = jumps.len();
    if n == 0 {
        return Some(0);
    }
    let mut dist: Vec<Option<usize>> = vec![None; n];
    dist[0] = Some(0);
    let mut queue = VecDeque::from([0usize]);
    while let Some(i) = queue.pop_front() {
        let d = dist[i].expect("queued indices have a distance");
        let end = i.saturating_add(jumps[i]).min(n - 1);
        for (j, slot) in dist.iter_mut().enumerate().take(end + 1).skip(i + 1) {
            if slot.is_none() {
                *slot = Some(d + 1);
                queue.push_back(j);
            }
        }
    }
    dist[n - 1]
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
}

#[test]
fn example_1() {
    assert_eq!(min_jumps(&[2, 3, 1, 1, 4]), Some(2));
}

#[test]
fn example_2() {
    assert_eq!(min_jumps(&[2, 3, 0, 1, 4]), Some(2));
}

#[test]
fn example_unreachable() {
    assert_eq!(min_jumps(&[3, 2, 1, 0, 4]), None);
}

#[test]
fn empty_needs_zero_jumps() {
    assert_eq!(min_jumps(&[]), Some(0));
}

#[test]
fn single_element_needs_zero_jumps() {
    assert_eq!(min_jumps(&[0]), Some(0));
    assert_eq!(min_jumps(&[7]), Some(0));
}

#[test]
fn two_elements() {
    assert_eq!(min_jumps(&[1, 0]), Some(1));
    assert_eq!(min_jumps(&[5, 9]), Some(1));
    assert_eq!(min_jumps(&[0, 9]), None);
}

#[test]
fn all_ones() {
    assert_eq!(min_jumps(&[1, 1, 1, 1, 1]), Some(4));
}

#[test]
fn one_big_jump() {
    assert_eq!(min_jumps(&[10, 0, 0, 0, 0, 0]), Some(1));
}

#[test]
fn exact_reach_no_extra_jump() {
    // Off-by-one trap: counting a jump when already at the last index.
    assert_eq!(min_jumps(&[2, 1, 0]), Some(1));
    assert_eq!(min_jumps(&[1, 2, 0]), Some(2));
}

#[test]
fn greedy_by_value_is_wrong() {
    // Jumping to the largest value (index 1, value 4) needs 3 jumps;
    // jumping to index 3 (3 + 3 = 6) needs only 2.
    assert_eq!(min_jumps(&[3, 4, 0, 3, 0, 0, 0]), Some(2));
}

#[test]
fn greedy_farthest_position_is_wrong() {
    // Always jumping as far as possible lands on a 1-chain.
    assert_eq!(min_jumps(&[3, 4, 1, 1, 1, 1]), Some(2));
}

#[test]
fn zeros_in_middle() {
    assert_eq!(min_jumps(&[3, 0, 0, 2, 0, 1]), Some(2));
}

#[test]
fn stuck_is_none_not_infinite_loop() {
    // Trap: looping forever (or counting phantom jumps) on a zero wall.
    assert_eq!(min_jumps(&[1, 0, 1]), None);
    assert_eq!(min_jumps(&[2, 0, 0, 1]), None);
    assert_eq!(min_jumps(&[0, 0, 0, 0]), None);
}

#[test]
fn overflow_with_huge_jump() {
    // Trap: `i + jumps[i]` overflows usize and panics.
    assert_eq!(min_jumps(&[usize::MAX, 0, 0]), Some(1));
    assert_eq!(min_jumps(&[1, usize::MAX, 0, 0, 0]), Some(2));
    assert_eq!(min_jumps(&[1, 0, usize::MAX, 0]), None);
}

#[test]
fn random_against_bfs() {
    let mut rng = Rng(0x1234_5678_9ABC_DEF1);
    let mut unreachable = 0;
    for _ in 0..800 {
        let n = rng.below(15) as usize;
        let jumps: Vec<usize> = (0..n).map(|_| rng.below(5) as usize).collect();
        let expected = brute_min_jumps(&jumps);
        unreachable += usize::from(expected.is_none());
        assert_eq!(min_jumps(&jumps), expected, "jumps: {jumps:?}");
    }
    assert!(unreachable > 50, "generator should produce unreachable cases");
}

#[test]
fn large_all_ones() {
    assert_eq!(min_jumps(&[1; 10_000]), Some(9_999));
}

#[test]
fn large_max_values() {
    // 10_000 indices with jump 1000: ceil(9999 / 1000) = 10 jumps.
    assert_eq!(min_jumps(&[1000; 10_000]), Some(10));
}

#[test]
fn large_decreasing() {
    // Index i < 999 reaches exactly 1000; afterwards every jump is 1.
    let jumps: Vec<usize> = (0..10_000).map(|i: usize| 1000usize.saturating_sub(i).max(1)).collect();
    assert_eq!(min_jumps(&jumps), Some(9_000));
}

#[test]
fn large_wall_near_end() {
    // Big jumps everywhere, but they all stop short at a zero.
    let mut jumps: Vec<usize> = (0..10_000).map(|i: usize| 9_998usize.saturating_sub(i)).collect();
    jumps[9_998] = 0;
    assert_eq!(min_jumps(&jumps), None);
}

#[test]
fn large_walls_and_short_jumps() {
    // Two worst cases of n = 10^4, alternating. A wall just before the end:
    // every index jumps up to the wall, so a DP or BFS that visits every
    // target of every jump does ~n^2 / 2 steps, and none of them finishes
    // early. Jumps of mostly 1 and a few 2: walking back from the end and
    // rescanning from index 0 for the leftmost index that reaches the current
    // position does ~n^2 / 2 steps. Tracking the current jump's reach is n.
    const ROUNDS: usize = 10;
    let n = 10_000;
    let mut rng = Rng(0x0BAD_1EA9_0018_0002);
    for round in 0..ROUNDS {
        let wall = n - 2 - rng.below(100) as usize;
        let walled: Vec<usize> = (0..n).map(|i: usize| wall.saturating_sub(i)).collect();
        assert_eq!(min_jumps(&walled), None, "round {round}: wall at {wall}, n = {n}");
        let short: Vec<usize> = (0..n).map(|_| if rng.below(8) == 0 { 2 } else { 1 }).collect();
        assert_eq!(min_jumps(&short), brute_min_jumps(&short), "round {round}: jumps of 1 and 2, n = {n}");
    }
}
