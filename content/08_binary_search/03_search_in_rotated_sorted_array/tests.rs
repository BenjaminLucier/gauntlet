// Tests for Search in Rotated Sorted Array — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::cell::Cell;
use std::cmp::Ordering;

fn check(nums: &[i32], target: i32, expected: Option<usize>) {
    assert_eq!(search_rotated(nums, &target), expected, "nums = {nums:?}, target = {target}");
}

fn rotated<T: Clone>(sorted: &[T], k: usize) -> Vec<T> {
    let mut v = sorted.to_vec();
    if !v.is_empty() {
        v.rotate_left(k % sorted.len());
    }
    v
}

fn brute_search<T: Eq>(nums: &[T], target: &T) -> Option<usize> {
    nums.iter().position(|x| x == target)
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

    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        lo + self.below((hi - lo + 1) as u64) as i32
    }
}

/// An integer whose comparisons are counted.
struct Counted<'a>(i64, &'a Cell<usize>);

impl PartialEq for Counted<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Counted<'_> {}

impl PartialOrd for Counted<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Counted<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.set(self.1.get() + 1);
        self.0.cmp(&other.0)
    }
}

#[test]
fn example_1() {
    check(&[4, 5, 6, 7, 0, 1, 2], 0, Some(4));
}

#[test]
fn example_2() {
    check(&[4, 5, 6, 7, 0, 1, 2], 3, None);
}

#[test]
fn example_3() {
    check(&[1], 0, None);
}

#[test]
fn example_4_chars() {
    assert_eq!(search_rotated(&['x', 'y', 'z', 'a', 'b'], &'a'), Some(3));
}

#[test]
fn empty_slice() {
    assert_eq!(search_rotated::<i32>(&[], &5), None);
}

#[test]
fn single_element_hit() {
    check(&[1], 1, Some(0));
}

#[test]
fn two_elements_rotated() {
    check(&[3, 1], 1, Some(1));
    check(&[3, 1], 3, Some(0));
    check(&[3, 1], 2, None);
    check(&[3, 1], 0, None);
    check(&[3, 1], 4, None);
}

#[test]
fn not_rotated() {
    check(&[1, 2, 3, 4, 5], 1, Some(0));
    check(&[1, 2, 3, 4, 5], 5, Some(4));
    check(&[1, 2, 3, 4, 5], 6, None);
    check(&[1, 2, 3, 4, 5], 0, None);
}

#[test]
fn target_at_pivot_and_its_neighbour() {
    let v = [6, 7, 8, 1, 2, 3, 4, 5];
    check(&v, 1, Some(3));
    check(&v, 8, Some(2));
}

#[test]
fn boundary_comparisons_trap() {
    // nums[lo] == nums[mid] when lo == mid: using `<` to detect the sorted left
    // half picks the wrong side here.
    check(&[3, 1], 1, Some(1));
    check(&[5, 1, 3], 5, Some(0));
    check(&[5, 1, 3], 3, Some(2));
    check(&[4, 5, 6, 7, 8, 1, 2, 3], 8, Some(4));
}

#[test]
fn target_between_halves_value_range() {
    // 3 is larger than the right half and smaller than the left half.
    check(&[4, 5, 6, 7, 0, 1, 2], 3, None);
    check(&[4, 5, 6, 7, 0, 1, 2], -1, None);
    check(&[4, 5, 6, 7, 0, 1, 2], 8, None);
}

#[test]
fn extreme_values() {
    let v = [0, i32::MAX, i32::MIN, -1];
    for (i, x) in v.iter().enumerate() {
        check(&v, *x, Some(i));
    }
    check(&v, 1, None);
    check(&v, i32::MIN + 1, None);
}

#[test]
fn every_rotation_every_target() {
    let sorted = [-10, -4, 0, 3, 8, 12, 20, 33, 41];
    for k in 0..sorted.len() {
        let v = rotated(&sorted, k);
        for t in -11..=42 {
            assert_eq!(search_rotated(&v, &t), brute_search(&v, &t), "nums = {v:?}, target = {t}");
        }
    }
}

#[test]
fn generic_over_strings() {
    let words: Vec<String> = ["kiwi", "lime", "pear", "apple", "fig"].map(String::from).to_vec();
    assert_eq!(search_rotated(&words, &"fig".to_string()), Some(4));
    assert_eq!(search_rotated(&words, &"kiwi".to_string()), Some(0));
    assert_eq!(search_rotated(&words, &"grape".to_string()), None);
}

#[test]
fn uses_logarithmic_comparisons() {
    let count = Cell::new(0);
    let sorted: Vec<i64> = (0..100_000).map(|i| 3 * i).collect();
    for k in [0, 1, 50_000, 99_999, 31_337] {
        let items: Vec<Counted> = rotated(&sorted, k).into_iter().map(|x| Counted(x, &count)).collect();
        for t in [0, 3 * 77_777, 299_997, 1, -3] {
            count.set(0);
            let got = search_rotated(&items, &Counted(t, &count));
            let used = count.get();
            let expected = items.iter().position(|c| c.0 == t);
            assert_eq!(got, expected, "rotation {k}, target {t}");
            assert!(used <= 150, "{used} comparisons for rotation {k}, target {t}");
        }
    }
}

#[test]
fn randomized_vs_brute() {
    let mut rng = Rng(0x3201_cafe_d00d);
    for _ in 0..600 {
        let n = rng.range(1, 25) as usize;
        let mut s: Vec<i32> = (0..n * 2).map(|_| rng.range(-60, 60)).collect();
        s.sort_unstable();
        s.dedup();
        s.truncate(n);
        let k = rng.range(0, s.len() as i32 - 1) as usize;
        let v = rotated(&s, k);
        let target = if rng.below(2) == 0 {
            v[rng.below(v.len() as u64) as usize]
        } else {
            rng.range(-62, 62)
        };
        assert_eq!(
            search_rotated(&v, &target),
            brute_search(&v, &target),
            "nums = {v:?}, target = {target}"
        );
    }
}

#[test]
fn large_many_rotations() {
    let n = 100_000;
    let sorted: Vec<i32> = (0..n as i32).map(|i| 4 * i - 200_000).collect();
    let mut rng = Rng(0x3202_0f0f_f0f0);
    for k in (0..n).step_by(1_997) {
        let v = rotated(&sorted, k);
        for _ in 0..10 {
            let i = rng.below(n as u64) as usize;
            assert_eq!(search_rotated(&v, &v[i]), Some(i), "rotation {k}, target {}", v[i]);
            assert_eq!(search_rotated(&v, &(v[i] + 1)), None, "rotation {k}, target {}", v[i] + 1);
        }
        assert_eq!(search_rotated(&v, &-200_000), Some((n - k) % n), "rotation {k}");
    }
}
