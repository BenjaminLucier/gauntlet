// Tests for Alien Dictionary — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::cmp::Ordering;
use std::collections::BTreeSet;

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

fn as_strs(words: &[String]) -> Vec<&str> {
    words.iter().map(String::as_str).collect()
}

fn letters(words: &[&str]) -> BTreeSet<char> {
    words.iter().flat_map(|w| w.chars()).collect()
}

/// Compare two words under the alphabet `order` (prefix sorts first).
fn cmp_under(a: &str, b: &str, order: &str) -> Ordering {
    let rank = |c: char| order.find(c).expect("letter missing from the order");
    let ka: Vec<usize> = a.chars().map(rank).collect();
    let kb: Vec<usize> = b.chars().map(rank).collect();
    ka.cmp(&kb)
}

fn sorted_under(words: &[&str], order: &str) -> bool {
    words.windows(2).all(|p| cmp_under(p[0], p[1], order) != Ordering::Greater)
}

/// Does any alphabet make `words` sorted? Checked via the derived constraints
/// and a Floyd–Warshall cycle check.
fn brute_exists(words: &[&str]) -> bool {
    let mut reach = [[false; 26]; 26];
    for p in words.windows(2) {
        let (a, b) = (p[0].as_bytes(), p[1].as_bytes());
        match a.iter().zip(b).find(|(x, y)| x != y) {
            Some((&x, &y)) => reach[(x - b'a') as usize][(y - b'a') as usize] = true,
            None if a.len() > b.len() => return false,
            None => {}
        }
    }
    for k in 0..26 {
        for i in 0..26 {
            for j in 0..26 {
                if reach[i][k] && reach[k][j] {
                    reach[i][j] = true;
                }
            }
        }
    }
    (0..26).all(|i| !reach[i][i])
}

/// Does any alphabet make `words` sorted? Exhaustive over permutations (few letters).
fn brute_exists_by_permutation(words: &[&str]) -> bool {
    fn go(rest: &mut Vec<char>, cur: &mut String, words: &[&str]) -> bool {
        if rest.is_empty() {
            return sorted_under(words, cur);
        }
        for i in 0..rest.len() {
            let c = rest.remove(i);
            cur.push(c);
            let ok = go(rest, cur, words);
            cur.pop();
            rest.insert(i, c);
            if ok {
                return true;
            }
        }
        false
    }
    let mut rest: Vec<char> = letters(words).into_iter().collect();
    go(&mut rest, &mut String::new(), words)
}

/// Validate the answer: None exactly when no order exists; otherwise every
/// letter once, and the words sorted under it.
fn check(words: &[&str]) {
    let got = alien_order(words);
    if !brute_exists(words) {
        assert_eq!(got, None, "no valid order exists for {words:?}");
        return;
    }
    let Some(got) = got else {
        panic!("an order exists for {words:?}, but got None");
    };
    let got_set: BTreeSet<char> = got.chars().collect();
    assert_eq!(got.chars().count(), got_set.len(), "letters repeated in {got:?} for {words:?}");
    assert_eq!(got_set, letters(words), "{got:?} must contain exactly the letters of {words:?}");
    assert!(sorted_under(words, &got), "{words:?} is not sorted under order {got:?}");
}

fn shuffled_alphabet(rng: &mut Rng) -> Vec<char> {
    let mut alphabet: Vec<char> = ('a'..='z').collect();
    for i in (1..26).rev() {
        alphabet.swap(i, rng.below(i as u64 + 1) as usize);
    }
    alphabet
}

#[test]
fn example_1() {
    assert_eq!(alien_order(&["wrt", "wrf", "er", "ett", "rftt"]).as_deref(), Some("wertf"));
}

#[test]
fn example_2() {
    assert_eq!(alien_order(&["z", "x"]).as_deref(), Some("zx"));
}

#[test]
fn example_3_contradiction_is_none() {
    assert_eq!(alien_order(&["z", "x", "z"]), None);
}

#[test]
fn example_4_prefix_after_longer_word() {
    assert_eq!(alien_order(&["abc", "ab"]), None);
}

#[test]
fn example_5_free_order() {
    check(&["ab"]);
    assert!(alien_order(&["ab"]).is_some());
}

#[test]
fn empty_list_is_some_empty_string() {
    // Not the same thing as "impossible".
    assert_eq!(alien_order(&[]), Some(String::new()));
}

#[test]
fn prefix_before_longer_word_is_fine() {
    check(&["ab", "abc"]);
}

#[test]
fn single_word_any_order_of_its_letters() {
    check(&["abc"]);
    check(&["zzz"]);
    assert_eq!(alien_order(&["zzz"]).as_deref(), Some("z"));
}

#[test]
fn duplicate_words() {
    assert_eq!(alien_order(&["z", "z"]).as_deref(), Some("z"));
    check(&["ab", "ab", "ac"]);
}

#[test]
fn letters_without_constraints_must_be_included() {
    // Only constraint is t < f, but w, r, x must also appear.
    check(&["wrt", "wrf", "wrfx"]);
    let got = alien_order(&["ba", "bc", "dd"]).expect("valid");
    assert_eq!(got.len(), 4, "got {got:?}");
}

#[test]
fn only_first_difference_counts() {
    // "ca" < "bb" says c < b and nothing about a vs b.
    check(&["ca", "bb", "ba"]);
    let words = ["ca", "cb", "ba"];
    let got = alien_order(&words).expect("valid");
    assert!(sorted_under(&words, &got) && got.len() == 3, "got {got:?}");
}

#[test]
fn non_adjacent_pairs_imply_nothing_new() {
    check(&["a", "b", "c", "d"]);
    assert_eq!(alien_order(&["a", "b", "c", "d"]).as_deref(), Some("abcd"));
}

#[test]
fn duplicate_edge_counted_once() {
    // a < b is implied three times; counting it more than once in an
    // in-degree would strand b and wrongly report a cycle.
    check(&["xa", "xb", "ya", "yb", "za", "zb"]);
    check(&["ca", "cb", "da", "db"]);
    assert_eq!(alien_order(&["a", "b", "a", "b"]), None);
    assert_eq!(alien_order(&["a", "ab", "b", "bb"]).map(|s| s.len()), Some(2));
}

#[test]
fn longer_cycle() {
    assert_eq!(alien_order(&["a", "b", "c", "a"]), None);
    assert_eq!(alien_order(&["ab", "bc", "ca", "ab"]), None);
}

#[test]
fn prefix_invalid_deep_in_list() {
    assert_eq!(alien_order(&["a", "b", "bca", "bc", "c"]), None);
}

#[test]
fn works_with_owned_strings() {
    let owned: Vec<String> = vec!["wrt".into(), "wrf".into(), "er".into()];
    check(&as_strs(&owned));
}

#[test]
fn randomized_sorted_by_hidden_alphabet() {
    let mut rng = Rng(0xA11E_4D1C_7105_0001);
    for _ in 0..300 {
        let k = 1 + rng.below(5) as usize;
        let alphabet = shuffled_alphabet(&mut rng);
        let hidden: String = alphabet[..k].iter().collect();
        let count = 1 + rng.below(8) as usize;
        let mut words: Vec<String> = (0..count)
            .map(|_| {
                let len = 1 + rng.below(4) as usize;
                (0..len).map(|_| alphabet[rng.below(k as u64) as usize]).collect()
            })
            .collect();
        words.sort_by(|a, b| cmp_under(a, b, &hidden));
        let words = as_strs(&words);
        assert!(brute_exists(&words));
        check(&words);
    }
}

#[test]
fn randomized_arbitrary_lists() {
    let mut rng = Rng(0xD1C7_10AA_5EED_0002);
    let alphabet = ['a', 'b', 'c', 'd', 'e'];
    for _ in 0..400 {
        let count = 1 + rng.below(6) as usize;
        let words: Vec<String> = (0..count)
            .map(|_| {
                let len = 1 + rng.below(3) as usize;
                (0..len).map(|_| alphabet[rng.below(5) as usize]).collect()
            })
            .collect();
        let words = as_strs(&words);
        assert_eq!(
            brute_exists(&words),
            brute_exists_by_permutation(&words),
            "oracle mismatch {words:?}"
        );
        check(&words);
    }
}

#[test]
fn large_full_alphabet() {
    let mut rng = Rng(269);
    let alphabet = shuffled_alphabet(&mut rng);
    let hidden: String = alphabet.iter().collect();
    let mut words: Vec<String> = (0..100)
        .map(|_| (0..100).map(|_| alphabet[rng.below(26) as usize]).collect())
        .collect();
    words.sort_by(|a, b| cmp_under(a, b, &hidden));
    check(&as_strs(&words));

    // Each letter as a word in hidden order pins the order down uniquely.
    let singles: Vec<String> = alphabet.iter().map(|c| c.to_string()).collect();
    assert_eq!(alien_order(&as_strs(&singles)), Some(hidden));
}

#[test]
fn large_prefix_violation_at_end() {
    // 99 long words sorted by a hidden alphabet, then a proper prefix of the
    // last one: every letter constraint is satisfiable, but the list is not.
    let mut rng = Rng(1269);
    let alphabet: Vec<char> = "qwertyuiopasdfghjklzxcvbnm".chars().collect();
    let hidden: String = alphabet.iter().collect();
    let mut words: Vec<String> = (0..99)
        .map(|_| (0..100).map(|_| alphabet[rng.below(26) as usize]).collect())
        .collect();
    words.sort_by(|a, b| cmp_under(a, b, &hidden));
    check(&as_strs(&words));
    let prefix = words[98][..60].to_string();
    words.push(prefix);
    assert_eq!(alien_order(&as_strs(&words)), None);
}
