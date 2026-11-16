// Tests for Palindrome Partitioning — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Pieces stay in order; only the outer list is sorted. Duplicates are kept.
fn normalize<T: Ord>(mut v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    v.sort();
    v
}

fn is_pal(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

/// Try every subset of the n-1 cut positions (between chars).
fn brute_partition(s: &str) -> Vec<Vec<String>> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n == 0 {
        return vec![Vec::new()];
    }
    let mut out = Vec::new();
    for mask in 0..1u32 << (n - 1) {
        let mut pieces = Vec::new();
        let mut start = 0;
        for i in 0..n {
            if i == n - 1 || mask >> i & 1 == 1 {
                pieces.push(chars[start..=i].iter().collect::<String>());
                start = i + 1;
            }
        }
        if pieces.iter().all(|p| is_pal(p)) {
            out.push(pieces);
        }
    }
    normalize(out)
}

fn check(s: &str) {
    let got = normalize(partition(s));
    assert_eq!(got, brute_partition(s), "s={s:?}");
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
    assert_eq!(normalize(partition("aab")), vec![vec!["a", "a", "b"], vec!["aa", "b"]]);
}

#[test]
fn example_2() {
    assert_eq!(partition("a"), vec![vec!["a"]]);
}

#[test]
fn empty_string_has_one_empty_partition() {
    assert_eq!(partition(""), vec![Vec::<&str>::new()]);
}

#[test]
fn two_equal_letters() {
    assert_eq!(normalize(partition("bb")), vec![vec!["b", "b"], vec!["bb"]]);
}

#[test]
fn two_different_letters() {
    assert_eq!(partition("ab"), vec![vec!["a", "b"]]);
}

#[test]
fn odd_palindrome_in_middle() {
    assert_eq!(normalize(partition("aba")), vec![vec!["a", "b", "a"], vec!["aba"]]);
}

#[test]
fn piece_order_matters() {
    // ["b","aa"] is not a valid partition of "aab" — pieces must stay in order.
    for p in partition("aab") {
        assert_eq!(p.concat(), "aab", "pieces {p:?} don't concatenate back to the input");
    }
}

#[test]
fn whole_string_palindrome() {
    check("racecar");
    assert!(partition("racecar").contains(&vec!["racecar"]));
}

#[test]
fn all_distinct_has_one_partition() {
    let s = "abcdefghijklmnop";
    let single: Vec<String> = s.chars().map(String::from).collect();
    assert_eq!(partition(s), vec![single]);
}

#[test]
fn overlapping_palindromes() {
    check("abba");
    check("aabaa");
    check("efe");
    check("cbbbcc");
}

#[test]
fn pieces_borrow_from_the_input() {
    let s = String::from("abba");
    let range = s.as_bytes().as_ptr_range();
    for p in partition(&s) {
        for piece in p {
            assert!(range.contains(&piece.as_ptr()), "{piece:?} is not a slice of the input");
        }
    }
}

#[test]
fn non_ascii_multibyte_chars() {
    // Byte-wise reversal of "éé" is not a palindrome; char-wise it is. Byte
    // slicing at a non-boundary would panic.
    assert_eq!(normalize(partition("éé")), vec![vec!["é", "é"], vec!["éé"]]);
    assert_eq!(normalize(partition("aéa")), vec![vec!["a", "é", "a"], vec!["aéa"]]);
    check("日本日");
    check("🦀a🦀🦀");
}

#[test]
fn randomized_against_cut_masks() {
    let mut rng = Rng(0x0777_0777);
    let alphabet = ['a', 'b', 'ß']; // one two-byte char keeps byte slicing honest
    for _ in 0..300 {
        let len = rng.below(10) + 1;
        let letters = rng.below(3) + 1;
        let s: String = (0..len).map(|_| alphabet[rng.below(letters) as usize]).collect();
        check(&s);
    }
}

#[test]
fn large_all_same_letter() {
    // Every one of the 2^15 cut patterns is valid.
    let s = "a".repeat(16);
    assert_eq!(partition(&s).len(), 1 << 15);
    check(&s);
}

#[test]
fn no_duplicate_partitions() {
    let got = normalize(partition("aaba"));
    for w in got.windows(2) {
        assert_ne!(w[0], w[1], "duplicate partition {:?}", w[0]);
    }
    check("aaba");
}
