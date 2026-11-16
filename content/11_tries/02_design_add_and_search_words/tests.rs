// Tests for Design Add and Search Words Data Structure — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::collections::HashSet;

fn dict(words: &[&str]) -> WordDictionary {
    let mut d = WordDictionary::new();
    for w in words {
        d.insert(w);
    }
    d
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

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }

    fn word(&mut self, max_len: usize, alphabet: usize) -> String {
        let len = 1 + self.below(max_len);
        (0..len).map(|_| char::from(b'a' + self.below(alphabet) as u8)).collect()
    }

    /// Replace each letter by '.' with probability 1/k.
    fn dotted(&mut self, w: &str, k: usize) -> String {
        w.chars().map(|c| if self.below(k) == 0 { '.' } else { c }).collect()
    }
}

fn brute_match(words: &[String], q: &str) -> bool {
    words.iter().any(|w| {
        w.len() == q.len() && w.bytes().zip(q.bytes()).all(|(a, b)| b == b'.' || a == b)
    })
}

#[test]
fn example_1() {
    let mut d = WordDictionary::new();
    d.insert("bad");
    d.insert("dad");
    d.insert("mad");
    assert!(!d.search("pad"));
    assert!(d.search("bad"));
    assert!(d.search(".ad"));
    assert!(d.search("b.."));
    assert!(!d.search("b."));
}

#[test]
fn empty_dictionary() {
    let d = WordDictionary::new();
    assert!(!d.search("a"));
    assert!(!d.search("."));
    assert!(!d.search(""));
}

#[test]
fn default_is_empty() {
    let d = WordDictionary::default();
    assert!(!d.search("a"));
    assert!(!d.search(""));
}

#[test]
fn empty_word_and_empty_pattern() {
    let mut d = dict(&["ab"]);
    assert!(!d.search(""), "the empty pattern only matches the empty word");
    d.insert("");
    assert!(d.search(""));
    assert!(!d.search("."));
    assert!(d.search(".."));
}

#[test]
fn dot_matches_exactly_one_letter() {
    let d = dict(&["ab"]);
    assert!(d.search("a."));
    assert!(d.search(".b"));
    assert!(d.search(".."));
    assert!(!d.search("."));
    assert!(!d.search("..."));
    assert!(!d.search("ab."));
    assert!(!d.search(".ab"));
}

#[test]
fn prefix_is_not_a_match() {
    let d = dict(&["bad"]);
    assert!(!d.search("ba"));
    assert!(!d.search("b."));
    assert!(!d.search("bad."));
}

#[test]
fn single_letters() {
    let d = dict(&["a"]);
    assert!(d.search("a"));
    assert!(d.search("."));
    assert!(!d.search("b"));
    assert!(!d.search("aa"));
}

#[test]
fn wildcard_must_backtrack() {
    // The first child tried for '.' ('a') leads to a dead end; 'b' works.
    let d = dict(&["axy", "bxz"]);
    assert!(d.search(".xz"));
    assert!(d.search(".x."));
    assert!(!d.search(".xa"));
    assert!(!d.search("a.z"));
}

#[test]
fn shorter_word_inside_longer_path() {
    let d = dict(&["abcd", "ab"]);
    assert!(d.search("ab"));
    assert!(d.search(".."));
    assert!(!d.search("abc"));
    assert!(!d.search("..."));
    assert!(d.search("...."));
}

#[test]
fn all_dots_of_each_length() {
    let d = dict(&["a", "bb", "dddd"]);
    let got: Vec<bool> = (1..=5).map(|n| d.search(&".".repeat(n))).collect();
    assert_eq!(got, [true, true, false, true, false]);
}

#[test]
fn queries_interleaved_with_inserts() {
    let mut d = WordDictionary::new();
    assert!(!d.search("a.c"));
    d.insert("abc");
    assert!(d.search("a.c"));
    assert!(!d.search("a.d"));
    d.insert("azd");
    assert!(d.search("a.d"));
}

#[test]
fn inserting_twice_is_harmless() {
    let d = dict(&["abc", "abc"]);
    assert!(d.search("abc"));
    assert!(d.search("..."));
    assert!(!d.search("ab"));
}

#[test]
fn accepts_any_kind_of_str() {
    let owned = String::from("crab");
    let mut d = WordDictionary::new();
    d.insert(&owned);
    let pattern = owned.replace('r', ".");
    assert!(d.search(&pattern));
    assert!(d.search(&owned[..]));
    assert_eq!(owned, "crab", "the caller keeps its string");
}

#[test]
fn max_length_words() {
    let w = "z".repeat(24) + "y";
    let d = dict(&[&w]);
    assert!(d.search(&w));
    assert!(d.search(&(".".repeat(24) + "y")));
    assert!(!d.search(&(".".repeat(24) + "z")));
    assert!(!d.search(&".".repeat(24)));
}

#[test]
fn randomized_vs_brute_force() {
    let mut rng = Rng(62);
    for _ in 0..40 {
        let mut d = WordDictionary::new();
        let mut words: Vec<String> = vec![];
        for _ in 0..200 {
            if rng.below(3) == 0 {
                let w = rng.word(5, 3);
                d.insert(&w);
                words.push(w);
            } else {
                // Half the queries are dotted versions of stored words.
                let base = if !words.is_empty() && rng.below(2) == 0 {
                    words[rng.below(words.len())].clone()
                } else {
                    rng.word(5, 3)
                };
                let q = rng.dotted(&base, 3);
                assert_eq!(d.search(&q), brute_match(&words, &q), "search({q}) with words {words:?}");
            }
        }
    }
}

#[test]
fn large_workload() {
    // 10^4 calls: 5000 inserts, 5000 searches (plain, with 2 dots, and many dots).
    let mut rng = Rng(6262);
    let mut d = WordDictionary::new();
    let mut words: Vec<String> = vec![];
    let mut set = HashSet::new();
    for _ in 0..5000 {
        let w = rng.word(25, 26);
        d.insert(&w);
        set.insert(w.clone());
        words.push(w);
    }
    for i in 0..5000 {
        let base = if i % 2 == 0 { words[rng.below(words.len())].clone() } else { rng.word(25, 26) };
        let q = match i % 5 {
            0 | 1 => base,
            4 => rng.dotted(&base, 2),
            _ => {
                let mut b = base.into_bytes();
                for _ in 0..2 {
                    let k = rng.below(b.len());
                    b[k] = b'.';
                }
                String::from_utf8(b).expect("ASCII stays valid UTF-8")
            }
        };
        let expected = if q.contains('.') { brute_match(&words, &q) } else { set.contains(&q) };
        assert_eq!(d.search(&q), expected, "search({q})");
    }
}
