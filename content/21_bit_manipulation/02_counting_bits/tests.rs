// Tests for Counting Bits — shared by template.rs, solution.rs and your attempt.
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
}

/// Oracle: count each number's bits from scratch by repeated division.
fn brute_count_bits(n: usize) -> Vec<u32> {
    (0..=n)
        .map(|mut i| {
            let mut c = 0;
            while i > 0 {
                c += (i % 2) as u32;
                i /= 2;
            }
            c
        })
        .collect()
}

fn check(n: usize, expected: &[u32]) {
    assert_eq!(count_bits(n), expected, "n = {n}");
}

#[test]
fn example_1() {
    check(2, &[0, 1, 1]);
}

#[test]
fn example_2() {
    check(5, &[0, 1, 1, 2, 1, 2]);
}

#[test]
fn zero_gives_single_element() {
    check(0, &[0]);
}

#[test]
fn one() {
    check(1, &[0, 1]);
}

#[test]
fn length_is_n_plus_one() {
    for n in [0, 1, 7, 8, 100, 1023, 1024] {
        assert_eq!(count_bits(n).len(), n + 1, "n = {n}");
    }
}

#[test]
fn up_to_power_of_two() {
    check(16, &[0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1]);
}

#[test]
fn ends_at_all_ones() {
    let ans = count_bits(1023);
    assert_eq!(ans[1023], 10);
    assert_eq!(ans[512], 1);
    assert_eq!(ans[511], 9);
}

#[test]
fn small_n_vs_brute_force() {
    for n in 0..=300 {
        check(n, &brute_count_bits(n));
    }
}

#[test]
fn randomized_vs_brute_force() {
    let mut rng = Rng(0x0bad_cafe_dead_beef);
    for _ in 0..50 {
        let n = rng.below(5_001) as usize;
        check(n, &brute_count_bits(n));
    }
}

#[test]
fn large_max_n() {
    let ans = count_bits(100_000);
    assert_eq!(ans.len(), 100_001);
    for (i, &c) in ans.iter().enumerate() {
        assert_eq!(c, i.count_ones(), "i = {i}");
    }
}

#[test]
fn large_repeated_calls() {
    // 50 calls with n = 10^5 — the O(n) DP handles this easily.
    let reference: Vec<u32> = (0..=100_000usize).map(usize::count_ones).collect();
    for _ in 0..50 {
        assert_eq!(count_bits(100_000), reference);
    }
}

#[test]
fn beyond_u16_range() {
    // Indices past 2^16 need the full width of the index type.
    let ans = count_bits(70_000);
    assert_eq!(ans[65_535], 16);
    assert_eq!(ans[65_536], 1);
    assert_eq!(ans[69_999], 69_999usize.count_ones());
}
