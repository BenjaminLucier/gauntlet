// Tests for Edit Distance — shared by template.rs, solution.rs and your attempt.
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

    fn string(&mut self, len: usize, alphabet: &[char]) -> String {
        (0..len)
            .map(|_| alphabet[self.below(alphabet.len() as u64) as usize])
            .collect()
    }
}

/// Exponential recursion over chars, no memo.
fn brute_ed(a: &[char], b: &[char]) -> usize {
    match (a, b) {
        ([], _) => b.len(),
        (_, []) => a.len(),
        ([x, a_rest @ ..], [y, b_rest @ ..]) if x == y => brute_ed(a_rest, b_rest),
        ([_, a_rest @ ..], [_, b_rest @ ..]) => {
            1 + brute_ed(a_rest, b).min(brute_ed(a, b_rest)).min(brute_ed(a_rest, b_rest))
        }
    }
}

fn brute(a: &str, b: &str) -> usize {
    let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
    brute_ed(&a, &b)
}

#[test]
fn example_1() {
    assert_eq!(edit_distance("horse", "ros"), 3);
}

#[test]
fn example_2() {
    assert_eq!(edit_distance("intention", "execution"), 5);
}

#[test]
fn both_empty() {
    assert_eq!(edit_distance("", ""), 0);
}

#[test]
fn one_empty() {
    assert_eq!(edit_distance("", "abc"), 3);
    assert_eq!(edit_distance("abcd", ""), 4);
}

#[test]
fn identical() {
    assert_eq!(edit_distance("same", "same"), 0);
    assert_eq!(edit_distance("a", "a"), 0);
}

#[test]
fn single_operation_each_kind() {
    assert_eq!(edit_distance("abc", "abd"), 1); // replace
    assert_eq!(edit_distance("abc", "abxc"), 1); // insert
    assert_eq!(edit_distance("abc", "ac"), 1); // delete
    assert_eq!(edit_distance("a", "b"), 1);
}

#[test]
fn replace_is_cheaper_than_delete_plus_insert() {
    // An insert/delete-only (LCS-based) distance gives 2 * differences.
    assert_eq!(edit_distance("abcdef", "azcdyf"), 2);
}

#[test]
fn not_hamming_distance() {
    // Position-by-position comparison gives 4; one insert + one delete suffices.
    assert_eq!(edit_distance("abcd", "bcde"), 2);
}

#[test]
fn classic_pairs() {
    assert_eq!(edit_distance("kitten", "sitting"), 3);
    assert_eq!(edit_distance("sunday", "saturday"), 3);
    assert_eq!(edit_distance("flaw", "lawn"), 2);
}

#[test]
fn counts_chars_not_bytes() {
    // Byte-level comparison would count 2 for each accented letter.
    assert_eq!(edit_distance("café", "cafe"), 1);
    assert_eq!(edit_distance("naïve", "naive"), 1);
    assert_eq!(edit_distance("é", "è"), 1);
    assert_eq!(edit_distance("", "日本語"), 3);
    assert_eq!(edit_distance("🦀rust", "rust🦀"), 2);
}

#[test]
fn symmetric() {
    let mut rng = Rng(0xC0FF_EE00_1234_5678);
    for _ in 0..100 {
        let (la, lb) = (rng.below(12) as usize, rng.below(12) as usize);
        let a = rng.string(la, &['a', 'b', 'c']);
        let b = rng.string(lb, &['a', 'b', 'c']);
        assert_eq!(edit_distance(&a, &b), edit_distance(&b, &a), "a = {a:?}, b = {b:?}");
    }
}

#[test]
fn random_vs_brute_force() {
    let mut rng = Rng(0x00DD_BA11_5EED);
    for _ in 0..400 {
        let (la, lb) = (rng.below(7) as usize, rng.below(7) as usize);
        let a = rng.string(la, &['a', 'b', 'c']);
        let b = rng.string(lb, &['a', 'b', 'c']);
        assert_eq!(edit_distance(&a, &b), brute(&a, &b), "from = {a:?}, to = {b:?}");
    }
}

#[test]
fn random_unicode_vs_brute_force() {
    let mut rng = Rng(0xFACE_B00C_0000_0007);
    for _ in 0..300 {
        let (la, lb) = (rng.below(7) as usize, rng.below(7) as usize);
        let a = rng.string(la, &['é', 'è', 'e', '🦀']);
        let b = rng.string(lb, &['é', 'è', 'e', '🦀']);
        assert_eq!(edit_distance(&a, &b), brute(&a, &b), "from = {a:?}, to = {b:?}");
    }
}

#[test]
fn large_disjoint_alphabets() {
    assert_eq!(edit_distance(&"a".repeat(500), &"b".repeat(500)), 500);
    assert_eq!(edit_distance(&"a".repeat(500), &"b".repeat(300)), 500);
}

#[test]
fn large_against_empty() {
    assert_eq!(edit_distance(&"x".repeat(500), ""), 500);
    assert_eq!(edit_distance("", &"x".repeat(500)), 500);
    assert_eq!(edit_distance(&"é".repeat(500), ""), 500);
}

#[test]
fn large_known_substitutions() {
    // Replace every 7th char with 'z' (absent from the original): exactly that many edits.
    let mut rng = Rng(123);
    let a = rng.string(500, &['a', 'b', 'c', 'd']);
    let b: String = a
        .chars()
        .enumerate()
        .map(|(i, c)| if i % 7 == 3 { 'z' } else { c })
        .collect();
    let k = (0..500).filter(|i| i % 7 == 3).count();
    assert_eq!(edit_distance(&a, &b), k);
    assert_eq!(edit_distance(&a, &a), 0);
}

#[test]
fn large_known_deletions_and_insertions() {
    let mut rng = Rng(321);
    let a = rng.string(500, &['a', 'b', 'c', 'd']);
    // delete every 5th char: exactly 100 edits (length difference is a lower bound)
    let shorter: String = a
        .chars()
        .enumerate()
        .filter(|(i, _)| i % 5 != 0)
        .map(|(_, c)| c)
        .collect();
    assert_eq!(edit_distance(&a, &shorter), 100);
    assert_eq!(edit_distance(&shorter, &a), 100);
    // prefix vs whole
    assert_eq!(edit_distance(&a[..250], &a), 250);
}
