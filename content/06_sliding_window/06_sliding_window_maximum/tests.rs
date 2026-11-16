// Tests for Sliding Window Maximum — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::cell::Cell;
use std::cmp::Ordering;
use std::num::NonZeroUsize;

fn nz(k: usize) -> NonZeroUsize {
    NonZeroUsize::new(k).expect("test window sizes are non-zero")
}

fn check(nums: &[i32], k: usize, expected: &[i32]) {
    let got = max_sliding_window(nums, nz(k));
    if nums.len() <= 50 {
        assert_eq!(got, expected, "nums = {nums:?}, k = {k}");
    } else {
        assert_eq!(
            got.len(),
            expected.len(),
            "wrong output length; nums.len() = {}, k = {k}",
            nums.len()
        );
        if let Some(i) = (0..got.len()).find(|&i| got[i] != expected[i]) {
            panic!(
                "window {i} wrong: got {}, expected {}; nums.len() = {}, k = {k}",
                got[i],
                expected[i],
                nums.len()
            );
        }
    }
}

/// O(n * k) oracle.
fn brute_windows<T: Ord + Copy>(nums: &[T], k: usize) -> Vec<T> {
    nums.windows(k)
        .map(|w| *w.iter().max().expect("windows are non-empty"))
        .collect()
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

    fn range(&mut self, lo: i64, hi: i64) -> i64 {
        lo + (self.next() % (hi - lo + 1) as u64) as i64
    }
}

#[test]
fn example_1() {
    check(&[1, 3, -1, -3, 5, 3, 6, 7], 3, &[3, 3, 5, 5, 6, 7]);
}

#[test]
fn example_2() {
    check(&[1], 1, &[1]);
}

#[test]
fn example_chars() {
    assert_eq!(
        max_sliding_window(&['r', 'u', 's', 't'], nz(2)),
        vec!['u', 'u', 't']
    );
}

#[test]
fn example_window_larger_than_slice() {
    check(&[1, 2], 3, &[]);
}

#[test]
fn empty_input() {
    check(&[], 1, &[]);
    check(&[], 5, &[]);
}

#[test]
fn window_much_larger_than_slice() {
    // Trap: `n - k + 1` underflows when k > n.
    check(&[4, 2, 7], 4, &[]);
    assert!(max_sliding_window(&[1, 2, 3], NonZeroUsize::MAX).is_empty());
}

#[test]
fn k_one_is_identity() {
    check(&[4, -2, 7, 0], 1, &[4, -2, 7, 0]);
}

#[test]
fn k_equals_n() {
    check(&[4, -2, 7, 0], 4, &[7]);
}

#[test]
fn all_negative() {
    // Trap: initializing the running max to 0.
    check(&[-5, -3, -9, -1], 2, &[-3, -3, -1]);
}

#[test]
fn extreme_values() {
    check(
        &[-10_000, 10_000, -10_000, -10_000],
        2,
        &[10_000, 10_000, -10_000],
    );
    check(&[i32::MIN, i32::MIN, i32::MAX], 2, &[i32::MIN, i32::MAX]);
}

#[test]
fn decreasing() {
    // Trap: forgetting to evict the max once it leaves the window.
    check(&[9, 8, 7, 6, 5, 4], 3, &[9, 8, 7, 6]);
}

#[test]
fn increasing() {
    check(&[1, 2, 3, 4, 5, 6], 3, &[3, 4, 5, 6]);
}

#[test]
fn duplicates_of_max() {
    // Trap: popping equal values from the front by value instead of index.
    check(&[5, 5, 1, 1, 1], 2, &[5, 5, 1, 1]);
    check(&[1, 3, 3, 1, 1, 1], 3, &[3, 3, 3, 1]);
}

#[test]
fn max_expires_then_second_best() {
    check(&[7, 2, 4, 1, 0], 2, &[7, 4, 4, 1]);
    check(&[1, 3, 1, 2, 0, 5], 3, &[3, 3, 2, 5]);
}

#[test]
fn generic_over_strs_and_tuples() {
    let words = ["pear", "apple", "zebra", "fig", "kiwi"];
    assert_eq!(
        max_sliding_window(&words, nz(2)),
        vec!["pear", "zebra", "zebra", "kiwi"]
    );
    // Tuples compare lexicographically.
    let pairs = [(1, 9), (2, 0), (1, 5), (2, 1)];
    assert_eq!(
        max_sliding_window(&pairs, nz(3)),
        vec![(2, 0), (2, 1)]
    );
}

#[test]
fn randomized_against_brute() {
    let mut rng = Rng(0x5BE0_CD19_137E_2179);
    for _ in 0..500 {
        let n = rng.range(0, 25) as usize;
        let k = rng.range(1, n as i64 + 2) as usize;
        let spread = if rng.range(0, 1) == 0 { 3 } else { 10_000 };
        let nums: Vec<i32> = (0..n).map(|_| rng.range(-spread, spread) as i32).collect();
        let expected = if k <= n { brute_windows(&nums, k) } else { Vec::new() };
        check(&nums, k, &expected);
    }
}

#[test]
fn randomized_u8_against_brute() {
    let mut rng = Rng(0x0DDB_A11C_0FFE_E000);
    for _ in 0..300 {
        let n = rng.range(1, 20) as usize;
        let k = rng.range(1, n as i64) as usize;
        let nums: Vec<u8> = (0..n).map(|_| rng.range(0, 255) as u8).collect();
        assert_eq!(
            max_sliding_window(&nums, nz(k)),
            brute_windows(&nums, k),
            "nums = {nums:?}, k = {k}"
        );
    }
}

#[test]
fn large_decreasing_half_window() {
    // Each window's maximum is its first element, which expires next step.
    let n = 20_000;
    let k = 10_000;
    let nums: Vec<i32> = (0..n).map(|i| 10_000 - (i as i32) / 5).collect();
    let expected: Vec<i32> = nums[..=n - k].to_vec();
    check(&nums, k, &expected);
}

#[test]
fn large_random() {
    let mut rng = Rng(0xCAFE_BABE_DEAD_10CC);
    let n = 100_000;
    let k = 1_000;
    let nums: Vec<i32> = (0..n).map(|_| rng.range(-10_000, 10_000) as i32).collect();
    // O(n) oracle via block decomposition: prefix maxima within each block of
    // size k plus suffix maxima; a window spans at most two blocks.
    let mut pre = vec![0; n];
    let mut suf = vec![0; n];
    for i in 0..n {
        pre[i] = if i % k == 0 {
            nums[i]
        } else {
            pre[i - 1].max(nums[i])
        };
    }
    for i in (0..n).rev() {
        suf[i] = if i == n - 1 || (i + 1) % k == 0 {
            nums[i]
        } else {
            suf[i + 1].max(nums[i])
        };
    }
    let expected: Vec<i32> = (0..=n - k).map(|i| suf[i].max(pre[i + k - 1])).collect();
    check(&nums, k, &expected);
}

thread_local! {
    static COMPARISONS: Cell<usize> = const { Cell::new(0) };
}

/// An `i32` that counts every comparison made on it (on this thread).
#[derive(Debug, Clone, Copy)]
struct Counted(i32);

impl PartialEq for Counted {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Counted {}

impl PartialOrd for Counted {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Counted {
    fn cmp(&self, other: &Self) -> Ordering {
        COMPARISONS.with(|c| c.set(c.get() + 1));
        self.0.cmp(&other.0)
    }
}

#[test]
fn comparisons_stay_linear_not_n_times_k() {
    // Recomputing every window's maximum makes about n * k comparisons
    // (10^7 for k = 1000); a monotonic deque needs at most 2 per element.
    let n = 10_000;
    let mut rng = Rng(0x1F83_D9AB_5BE0_CD19);
    let random: Vec<i32> = (0..n).map(|_| rng.range(-1_000, 1_000) as i32).collect();
    let increasing: Vec<i32> = (0..n as i32).collect();
    let decreasing: Vec<i32> = (0..n as i32).rev().collect();
    let zigzag: Vec<i32> = (0..n as i32).map(|i| if i % 2 == 0 { i } else { -i }).collect();
    let equal = vec![7; n];
    for (name, nums) in [
        ("random", &random),
        ("increasing", &increasing),
        ("decreasing", &decreasing),
        ("zigzag", &zigzag),
        ("all equal", &equal),
    ] {
        for k in [1, 2, 100, 1_000, 5_000, n] {
            let input: Vec<Counted> = nums.iter().copied().map(Counted).collect();
            COMPARISONS.with(|c| c.set(0));
            let got = max_sliding_window(&input, nz(k));
            let calls = COMPARISONS.with(Cell::get);
            assert!(
                calls <= 4 * n,
                "{calls} comparisons for {name} input, n = {n}, k = {k}"
            );
            assert_eq!(got.len(), n + 1 - k, "{name} input, n = {n}, k = {k}");
            // Spot-check every 101st window and the last one.
            for i in (0..=n - k).step_by(101).chain([n - k]) {
                let expected = *nums[i..i + k].iter().max().expect("k >= 1");
                assert_eq!(got[i].0, expected, "window {i}, {name} input, n = {n}, k = {k}");
            }
        }
    }
}
