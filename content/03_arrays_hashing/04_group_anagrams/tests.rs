// Tests for Group Anagrams — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::collections::HashMap;

/// Sort inside each group, then sort the groups, so any order is accepted.
fn normalize(mut groups: Vec<Vec<&str>>) -> Vec<Vec<&str>> {
    for g in &mut groups {
        g.sort_unstable();
    }
    groups.sort_unstable();
    groups
}

fn check(input: &[&str], expected: &[&[&str]]) {
    let got = normalize(group_anagrams(input));
    let exp = normalize(expected.iter().map(|g| g.to_vec()).collect());
    assert_eq!(got, exp, "words = {input:?}");
}

/// Quadratic oracle: put each word into the first group whose head has the
/// same sorted letters.
fn brute_group<'a>(input: &[&'a str]) -> Vec<Vec<&'a str>> {
    let sorted = |s: &str| {
        let mut v: Vec<u8> = s.bytes().collect();
        v.sort_unstable();
        v
    };
    let mut groups: Vec<Vec<&'a str>> = Vec::new();
    for &s in input {
        match groups.iter_mut().find(|g| sorted(g[0]) == sorted(s)) {
            Some(g) => g.push(s),
            None => groups.push(vec![s]),
        }
    }
    groups
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
    check(
        &["eat", "tea", "tan", "ate", "nat", "bat"],
        &[&["bat"], &["nat", "tan"], &["ate", "eat", "tea"]],
    );
}

#[test]
fn example_2() {
    check(&[""], &[&[""]]);
}

#[test]
fn example_3() {
    check(&["a"], &[&["a"]]);
}

#[test]
fn empty_input_no_groups() {
    assert!(group_anagrams(&[]).is_empty());
}

#[test]
fn multiple_empty_strings_one_group() {
    check(&["", "", "a", ""], &[&["", "", ""], &["a"]]);
}

#[test]
fn duplicates_are_kept() {
    // Trap: deduplicating with a HashSet.
    check(&["ab", "ba", "ab"], &[&["ab", "ab", "ba"]]);
}

#[test]
fn same_letters_different_counts() {
    // Trap: keying by the *set* of letters.
    check(&["aab", "abb", "bab"], &[&["aab"], &["abb", "bab"]]);
}

#[test]
fn same_length_same_byte_sum() {
    // Trap: keying by length + byte sum. 'a'+'d' == 'b'+'c'.
    check(&["ad", "bc", "da"], &[&["ad", "da"], &["bc"]]);
}

#[test]
fn no_anagrams_at_all() {
    check(&["abc", "abd", "abe", "x"], &[&["abc"], &["abd"], &["abe"], &["x"]]);
}

#[test]
fn all_one_group() {
    check(
        &["abc", "bca", "cab", "acb", "bac", "cba"],
        &[&["abc", "bca", "cab", "acb", "bac", "cba"]],
    );
}

#[test]
fn prefix_is_not_anagram() {
    check(&["a", "aa", "aaa"], &[&["a"], &["aa"], &["aaa"]]);
}

#[test]
fn full_alphabet_and_z() {
    // Letters at both ends of the alphabet.
    check(
        &["zyxwvutsrqponmlkjihgfedcba", "abcdefghijklmnopqrstuvwxyz", "zz", "z"],
        &[
            &["abcdefghijklmnopqrstuvwxyz", "zyxwvutsrqponmlkjihgfedcba"],
            &["zz"],
            &["z"],
        ],
    );
}

#[test]
fn result_borrows_from_the_original_strings() {
    // The output points into the owned Strings, not into fresh copies, and it
    // outlives the temporary Vec<&str> it was built from.
    let owned: Vec<String> = vec!["listen".into(), "silent".into(), "enlist".into()];
    let groups = {
        let refs: Vec<&str> = owned.iter().map(String::as_str).collect();
        group_anagrams(&refs)
    };
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].len(), 3);
    for word in &groups[0] {
        assert!(
            owned.iter().any(|s| std::ptr::eq(s.as_str(), *word)),
            "{word:?} is not one of the input strings"
        );
    }
}

#[test]
fn randomized_against_brute() {
    let mut rng = Rng(0x1234_5678_9ABC_DEF1);
    for _ in 0..300 {
        let n = rng.below(11) as usize;
        let owned: Vec<String> = (0..n)
            .map(|_| {
                let len = rng.below(4) as usize;
                (0..len)
                    .map(|_| char::from(b'a' + rng.below(3) as u8))
                    .collect()
            })
            .collect();
        let input: Vec<&str> = owned.iter().map(String::as_str).collect();
        let got = normalize(group_anagrams(&input));
        let exp = normalize(brute_group(&input));
        assert_eq!(got, exp, "words = {input:?}");
    }
}

#[test]
fn large_input() {
    // 10^4 words of length 100: 100 groups of 100 rotations each.
    let mut owned = Vec::with_capacity(10_000);
    for g in 0..100u32 {
        let base: Vec<u8> = (0..100u32)
            .map(|i| b'a' + ((i * i + g * 7 + i * g) % 26) as u8)
            .collect();
        for r in 0..100 {
            let mut s = base.clone();
            s.rotate_left(r);
            owned.push(String::from_utf8(s).expect("ASCII"));
        }
    }
    let input: Vec<&str> = owned.iter().map(String::as_str).collect();
    let got = group_anagrams(&input);
    let total: usize = got.iter().map(Vec::len).sum();
    assert_eq!(total, 10_000, "every word must appear exactly once");
    let exp = {
        // Sorted-key grouping is a fast enough oracle here.
        let mut m: HashMap<Vec<u8>, Vec<&str>> = HashMap::new();
        for &s in &input {
            let mut k: Vec<u8> = s.bytes().collect();
            k.sort_unstable();
            m.entry(k).or_default().push(s);
        }
        m.into_values().collect::<Vec<_>>()
    };
    assert_eq!(normalize(got), normalize(exp));
}
