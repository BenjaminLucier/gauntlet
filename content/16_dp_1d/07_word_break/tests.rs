// Tests for Word Break — shared by template.rs, solution.rs and your attempt.
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

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// Exponential brute force: try every dictionary word as the next piece.
fn brute_break(s: &str, words: &[&str]) -> bool {
    s.is_empty()
        || words
            .iter()
            .any(|w| s.strip_prefix(w).is_some_and(|rest| brute_break(rest, words)))
}

fn as_strs(owned: &[String]) -> Vec<&str> {
    owned.iter().map(String::as_str).collect()
}

#[test]
fn example_1() {
    assert!(word_break("gauntlet", &["gaunt", "let"]));
}

#[test]
fn example_2() {
    assert!(word_break("applepenapple", &["apple", "pen"]));
}

#[test]
fn example_3() {
    assert!(!word_break("catsandog", &["cats", "dog", "sand", "and", "cat"]));
}

#[test]
fn empty_string_is_zero_words() {
    assert!(word_break("", &[]));
    assert!(word_break("", &["a"]));
}

#[test]
fn empty_dictionary() {
    assert!(!word_break("a", &[]));
}

#[test]
fn single_char() {
    assert!(word_break("a", &["a"]));
    assert!(!word_break("a", &["b"]));
}

#[test]
fn whole_string_is_one_word() {
    assert!(word_break("pineapple", &["pine", "pineapple"]));
}

#[test]
fn word_longer_than_string() {
    assert!(!word_break("ab", &["abc"]));
}

#[test]
fn greedy_longest_match_fails() {
    // Taking "car" first leaves "s"; the answer is "ca" + "rs".
    assert!(word_break("cars", &["car", "ca", "rs"]));
}

#[test]
fn greedy_shortest_match_fails() {
    // Always taking the shortest word ("a") strands the final "b";
    // the answer is "a" + "a" + "ab". Likewise "go" strands "alspecial".
    assert!(word_break("aaab", &["a", "aa", "ab"]));
    assert!(word_break("goalspecial", &["go", "goal", "goals", "special"]));
}

#[test]
fn leftover_suffix_is_false() {
    // Every prefix segmentation leaves "x" at the end.
    assert!(!word_break("catcatx", &["cat", "ca", "t"]));
}

#[test]
fn reuse_same_word_many_times() {
    assert!(word_break(&"ab".repeat(50), &["ab"]));
    assert!(!word_break(&format!("{}a", "ab".repeat(50)), &["ab"]));
}

#[test]
fn overlapping_words() {
    assert!(word_break("catsanddog", &["cat", "cats", "and", "sand", "dog"]));
    assert!(word_break("aaaaaaa", &["aaaa", "aaa"]));
    assert!(!word_break("aaaaaaa", &["aaaa", "aaaaaa"]));
}

#[test]
fn non_ascii_words() {
    // Trap: `&s[j..i]` at every byte offset panics inside 'è' or '日'.
    assert!(word_break("crème brûlée", &["crème", " ", "brûlée"]));
    assert!(word_break("日本語", &["日本", "語"]));
    assert!(word_break("日本語", &["日", "本語"]));
    assert!(!word_break("日本語", &["日本", "本語"]));
    assert!(word_break("🦀🦀🦀", &["🦀"]));
}

#[test]
fn non_ascii_partial_characters_never_match() {
    // "é" is [0xC3, 0xA9] and "è" is [0xC3, 0xA8]: they share a first byte but
    // no word can match half a character.
    assert!(!word_break("é", &["è"]));
    assert!(!word_break("éè", &["é", "e"]));
    assert!(word_break("éè", &["é", "è"]));
    // Precomposed 'é' (U+00E9) is not 'e' + combining acute (U+0301).
    assert!(!word_break("caf\u{e9}", &["caf", "e\u{301}"]));
    assert!(word_break("cafe\u{301}", &["caf", "e\u{301}"]));
}

#[test]
fn works_with_owned_dictionary() {
    let owned: Vec<String> = ["apple", "pen"].iter().map(|w| w.to_string()).collect();
    let text = String::from("penapple");
    assert!(word_break(&text, &as_strs(&owned)));
}

#[test]
fn random_small_match_brute_force() {
    let mut rng = Rng(0x2468_1357_ACE0_BDF1);
    let alphabet = ['a', 'b', 'c', 'é'];
    for _ in 0..500 {
        let k = 1 + rng.below(alphabet.len());
        let make = |rng: &mut Rng, len: usize| -> String {
            (0..len).map(|_| alphabet[rng.below(k)]).collect()
        };
        let mut dict: Vec<String> = Vec::new();
        for _ in 0..1 + rng.below(5) {
            let len = 1 + rng.below(4);
            let w = make(&mut rng, len);
            if !dict.contains(&w) {
                dict.push(w);
            }
        }
        let n = rng.below(15);
        let mut s = if rng.below(2) == 0 {
            // Build from dictionary words.
            let mut s = String::new();
            while s.chars().count() < n {
                s.push_str(&dict[rng.below(dict.len())]);
            }
            s
        } else {
            make(&mut rng, n)
        };
        if !s.is_empty() && rng.below(3) == 0 {
            // Mutate one character.
            let mut chars: Vec<char> = s.chars().collect();
            let i = rng.below(chars.len());
            chars[i] = alphabet[rng.below(k)];
            s = chars.into_iter().collect();
        }
        let words = as_strs(&dict);
        assert_eq!(
            word_break(&s, &words),
            brute_break(&s, &words),
            "s = {s:?}, words = {words:?}"
        );
    }
}

#[test]
fn large_exponential_backtracking_trap() {
    // Classic killer for unmemoized recursion: many ways to split the a's,
    // but the final 'b' makes every one of them fail.
    let s = format!("{}b", "a".repeat(299));
    let dict: Vec<String> = (1..=10).map(|k| "a".repeat(k)).collect();
    assert!(!word_break(&s, &as_strs(&dict)));
    // And the matching true case.
    assert!(word_break(&"a".repeat(300), &as_strs(&dict)));
}

#[test]
fn large_dictionary() {
    // 1000 distinct words of length 5..=20 over "ab"; the string is a
    // concatenation of dictionary words, so the answer is true.
    let mut rng = Rng(4242);
    let mut dict: Vec<String> = Vec::new();
    while dict.len() < 1000 {
        let len = 5 + rng.below(16);
        let w: String = (0..len).map(|_| if rng.below(2) == 0 { 'a' } else { 'b' }).collect();
        if !dict.contains(&w) {
            dict.push(w);
        }
    }
    let mut s = String::new();
    while s.len() < 280 {
        s.push_str(&dict[rng.below(dict.len())]);
    }
    assert!(s.len() <= 300);
    let words = as_strs(&dict);
    assert!(word_break(&s, &words), "built from dictionary words");
    // Replacing the last letter with one that appears in no word makes it impossible.
    s.pop();
    s.push('z');
    assert!(!word_break(&s, &words));
}

#[test]
fn large_non_ascii_trap() {
    // 2-byte characters: 150 'é's (300 bytes) with words of 1..=10 'é's.
    let dict: Vec<String> = (1..=10).map(|k| "é".repeat(k)).collect();
    let words = as_strs(&dict);
    assert!(word_break(&"é".repeat(150), &words));
    let s = format!("{}è", "é".repeat(149));
    assert!(!word_break(&s, &words));
}
