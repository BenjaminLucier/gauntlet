// Tests for Gas Station — shared by template.rs, solution.rs and your attempt.
use super::*;

fn completes_from(gas: &[u32], cost: &[u32], start: usize) -> bool {
    let n = gas.len();
    let mut tank = 0i64;
    (0..n).all(|k| {
        let i = (start + k) % n;
        tank += i64::from(gas[i]) - i64::from(cost[i]);
        tank >= 0
    })
}

/// O(n²) simulation: every valid start.
fn brute_valid_starts(gas: &[u32], cost: &[u32]) -> Vec<usize> {
    (0..gas.len()).filter(|&s| completes_from(gas, cost, s)).collect()
}

/// Checks the answer against the simulation. If several starts happen to be
/// valid (the classic problem guarantees uniqueness), any of them is accepted.
fn check(gas: &[u32], cost: &[u32]) {
    let valid = brute_valid_starts(gas, cost);
    let got = can_complete_circuit(gas, cost);
    match got {
        None => assert!(valid.is_empty(), "gas: {gas:?}, cost: {cost:?}: got None, valid starts {valid:?}"),
        Some(s) => assert!(
            valid.contains(&s),
            "gas: {gas:?}, cost: {cost:?}: got {s}, valid starts {valid:?}"
        ),
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
}

#[test]
fn example_1() {
    assert_eq!(can_complete_circuit(&[1, 2, 3, 4, 5], &[3, 4, 5, 1, 2]), Some(3));
}

#[test]
fn example_2() {
    assert_eq!(can_complete_circuit(&[2, 3, 4], &[3, 4, 3]), None);
}

#[test]
fn no_stations_is_none() {
    assert_eq!(can_complete_circuit(&[], &[]), None);
}

#[test]
fn single_station_enough() {
    assert_eq!(can_complete_circuit(&[5], &[4]), Some(0));
    assert_eq!(can_complete_circuit(&[4], &[4]), Some(0));
}

#[test]
fn single_station_not_enough() {
    assert_eq!(can_complete_circuit(&[3], &[4]), None);
}

#[test]
fn all_zero() {
    assert_eq!(can_complete_circuit(&[0], &[0]), Some(0));
}

#[test]
fn answer_is_last_station() {
    assert_eq!(can_complete_circuit(&[0, 0, 0, 10], &[1, 1, 1, 7]), Some(3));
}

#[test]
fn answer_is_first_station() {
    assert_eq!(can_complete_circuit(&[4, 1, 1], &[2, 2, 2]), Some(0));
}

#[test]
fn exact_total_equal() {
    // Total gas == total cost: still solvable, tank ends at exactly 0.
    assert_eq!(can_complete_circuit(&[3, 1, 1], &[1, 2, 2]), Some(0));
    assert_eq!(can_complete_circuit(&[1, 1, 3], &[2, 2, 1]), Some(2));
}

#[test]
fn start_at_max_gas_is_wrong() {
    // Station 0 has the most gas but its leg is very expensive.
    assert_eq!(can_complete_circuit(&[10, 2, 2], &[11, 1, 2]), Some(1));
}

#[test]
fn start_at_max_surplus_is_wrong() {
    // Stations 0 and 2 both have the largest surplus (+3); only 0 works —
    // starting at 2 runs dry on the expensive leg 3.
    check(&[4, 0, 4, 0], &[1, 2, 1, 4]);
    assert_eq!(can_complete_circuit(&[4, 0, 4, 0], &[1, 2, 1, 4]), Some(0));
}

#[test]
fn local_surplus_but_global_deficit() {
    assert_eq!(can_complete_circuit(&[5, 5, 0], &[1, 1, 9]), None);
}

#[test]
fn unsigned_subtraction_underflow() {
    // Trap: `gas[i] - cost[i]` on u32 panics when cost > gas.
    assert_eq!(can_complete_circuit(&[0, 7], &[7, 0]), Some(1));
}

#[test]
fn huge_values_do_not_overflow() {
    // Trap: summing u32::MAX values in i32 or u32 overflows.
    let gas = [u32::MAX, u32::MAX, 0];
    let cost = [0, u32::MAX, u32::MAX];
    assert_eq!(can_complete_circuit(&gas, &cost), Some(0));
    let gas = vec![u32::MAX; 1000];
    let mut cost = vec![u32::MAX; 1000];
    cost[999] = 0;
    assert_eq!(can_complete_circuit(&gas, &cost), Some(0));
}

#[test]
fn random_against_simulation() {
    let mut rng = Rng(0xABCD_EF01_2345_6789);
    for _ in 0..600 {
        let n = rng.below(11) as usize;
        let gas: Vec<u32> = (0..n).map(|_| rng.below(7) as u32).collect();
        let cost: Vec<u32> = (0..n).map(|_| rng.below(7) as u32).collect();
        check(&gas, &cost);
    }
}

#[test]
fn large_input_with_one_valid_start() {
    // Gas equals cost everywhere except station a - 1 (one short) and a (one
    // over), so a is the only valid start, and every other start drives on
    // until it fails at a - 1. With a at ~0.7 n, "try every start" makes
    // ~n^2 / 4 = 5 * 10^8 steps whether it tries the starts from 0 up or from
    // n - 1 down; one pass is n.
    let n = 45_000;
    let mut rng = Rng(0x6A55_7A71_0018_0003);
    let a = n / 100 * 69 + rng.below(n as u64 / 100 * 3) as usize;
    let cost: Vec<u32> = (0..n).map(|_| 1 + rng.below(9_999) as u32).collect();
    let mut gas = cost.clone();
    gas[a - 1] -= 1;
    gas[a] += 1;
    assert_eq!(can_complete_circuit(&gas, &cost), Some(a), "n = {n}, answer {a}");
}

#[test]
fn large_no_answer() {
    let n = 2000;
    let mut gas = vec![10_000; n];
    let cost = vec![10_000; n];
    gas[n / 2] = 9_999;
    assert_eq!(can_complete_circuit(&gas, &cost), None);
}
