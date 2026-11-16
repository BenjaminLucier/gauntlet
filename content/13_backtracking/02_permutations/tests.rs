// Tests for Permutations — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Order matters inside a permutation, so only the outer list is sorted.
fn normalize<T: Ord>(mut v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    v.sort();
    v
}

/// Lexicographic next-permutation enumeration from the sorted input.
fn brute_permutations(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut a = nums.to_vec();
    a.sort_unstable();
    let mut out = vec![a.clone()];
    loop {
        let n = a.len();
        let Some(i) = (0..n.saturating_sub(1)).rev().find(|&i| a[i] < a[i + 1]) else {
            break;
        };
        let j = (i + 1..n).rev().find(|&j| a[j] > a[i]).expect("a[i+1] > a[i] exists");
        a.swap(i, j);
        a[i + 1..].reverse();
        out.push(a.clone());
    }
    out
}

fn check(nums: &[i32]) {
    let got = normalize(permutations(nums));
    assert_eq!(got, brute_permutations(nums), "nums={nums:?}");
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
    assert_eq!(
        normalize(permutations(&[1, 2, 3])),
        vec![vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3], vec![2, 3, 1], vec![3, 1, 2], vec![3, 2, 1]]
    );
}

#[test]
fn example_2() {
    assert_eq!(normalize(permutations(&[0, 1])), vec![vec![0, 1], vec![1, 0]]);
}

#[test]
fn example_3() {
    assert_eq!(permutations(&[1]), vec![vec![1]]);
}

#[test]
fn empty_input_has_one_empty_permutation() {
    assert_eq!(permutations::<i32>(&[]), vec![Vec::<i32>::new()]);
}

#[test]
fn negatives_and_extremes() {
    check(&[-10, 10]);
    check(&[-1, -5, 0, 7]);
    check(&[i32::MIN, 0, i32::MAX]);
}

#[test]
fn unsorted_input() {
    check(&[3, -2, 9, 0]);
}

#[test]
fn each_output_is_a_full_permutation() {
    let nums = [4, 8, 15, 16, 23];
    let got = permutations(&nums);
    assert_eq!(got.len(), 120);
    for p in &got {
        let mut s = p.clone();
        s.sort_unstable();
        assert_eq!(s, nums, "{p:?} is not a permutation of {nums:?}");
    }
}

#[test]
fn no_duplicate_permutations() {
    let got = normalize(permutations(&[1, 2, 3, 4]));
    for w in got.windows(2) {
        assert_ne!(w[0], w[1], "duplicate permutation {:?}", w[0]);
    }
    assert_eq!(got.len(), 24);
}

#[test]
fn randomized_against_next_permutation() {
    let mut rng = Rng(0x0744_0744);
    for _ in 0..60 {
        let n = rng.below(7) as usize;
        let mut pool: Vec<i32> = (-10..=10).collect();
        for i in (1..pool.len()).rev() {
            let j = rng.below(i as u64 + 1) as usize;
            pool.swap(i, j);
        }
        check(&pool[..n]);
    }
}

#[test]
fn max_size() {
    let nums = [6, -4, 2, 0, -10, 10];
    assert_eq!(permutations(&nums).len(), 720);
    check(&nums);
}

#[test]
fn two_elements_negative() {
    assert_eq!(normalize(permutations(&[-3, -7])), vec![vec![-7, -3], vec![-3, -7]]);
}

#[test]
fn input_order_does_not_change_result_set() {
    let a = normalize(permutations(&[1, 2, 3, 4]));
    let b = normalize(permutations(&[4, 3, 2, 1]));
    assert_eq!(a, b);
}

#[test]
fn each_element_starts_equally_often() {
    // Every element leads (n-1)! permutations.
    let got = permutations(&[5, 6, 7, 8]);
    for x in [5, 6, 7, 8] {
        assert_eq!(got.iter().filter(|p| p[0] == x).count(), 6, "first element {x}");
    }
}

#[test]
fn input_is_not_modified() {
    let nums = vec![3, 1, 2];
    let _ = permutations(&nums);
    assert_eq!(nums, vec![3, 1, 2]);
}

#[test]
fn generic_over_chars_and_strings() {
    let got = normalize(permutations(&['a', 'b', 'c']));
    let words: Vec<String> = got.iter().map(|p| p.iter().collect()).collect();
    assert_eq!(words, ["abc", "acb", "bac", "bca", "cab", "cba"]);

    let owned: Vec<String> = vec!["héllo".into(), "wörld".into()];
    let got = normalize(permutations(&owned));
    assert_eq!(got, vec![vec!["héllo", "wörld"], vec!["wörld", "héllo"]]);
}

#[test]
fn works_without_ord_or_hash() {
    // Clone only — the API must not demand comparisons.
    #[derive(Clone, Debug)]
    struct Step(u8);
    let got = permutations(&[Step(1), Step(2), Step(3)]);
    let mut codes: Vec<u32> = got
        .iter()
        .map(|p| p.iter().fold(0, |acc, s| acc * 10 + u32::from(s.0)))
        .collect();
    codes.sort_unstable();
    assert_eq!(codes, vec![123, 132, 213, 231, 312, 321]);
}
