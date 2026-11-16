// Tests for Trie — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::collections::{BTreeSet, HashSet};

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

    /// Random word of length `min_len..=max_len` (in chars) over `alphabet`.
    fn word(&mut self, min_len: usize, max_len: usize, alphabet: &[char]) -> String {
        let len = min_len + self.below(max_len - min_len + 1);
        (0..len)
            .map(|_| alphabet[self.below(alphabet.len())])
            .collect()
    }
}

fn trie_of(words: &[&str]) -> Trie {
    let mut trie = Trie::new();
    for w in words {
        trie.insert(w);
    }
    trie
}

fn brute_words(model: &BTreeSet<String>, prefix: &str) -> Vec<String> {
    model
        .iter()
        .filter(|w| w.starts_with(prefix))
        .cloned()
        .collect()
}

fn brute_lcp(model: &BTreeSet<String>) -> Option<String> {
    let first = model.first()?;
    let last = model.last()?;
    // In sorted order, the common prefix of all is that of the first and last.
    Some(
        first
            .chars()
            .zip(last.chars())
            .take_while(|(a, b)| a == b)
            .map(|(a, _)| a)
            .collect(),
    )
}

/// Distinct non-empty prefixes (in chars) of the stored words.
fn brute_node_count(model: &BTreeSet<String>) -> usize {
    let mut prefixes = HashSet::new();
    for w in model {
        for (i, c) in w.char_indices() {
            prefixes.insert(&w[..i + c.len_utf8()]);
        }
    }
    prefixes.len()
}

#[test]
fn readme_example() {
    let mut t = Trie::new();
    for w in ["car", "cart", "care", "cat", "dog"] {
        assert!(t.insert(w));
    }
    assert!(!t.insert("cat"));
    assert_eq!(t.len(), 5);
    assert!(t.contains("car") && !t.contains("ca"));
    assert!(t.starts_with("ca"));
    assert_eq!(t.count_with_prefix("car"), 3);
    assert_eq!(t.words_with_prefix("car"), ["car", "care", "cart"]);
    assert_eq!(t.longest_common_prefix(), Some(String::new()));
    assert!(t.remove("dog"));
    assert!(!t.remove("dog"));
    assert!(!t.starts_with("d"));
    assert_eq!(t.node_count(), 6);
    assert_eq!(t.longest_common_prefix(), Some("ca".to_string()));

    let mut u = Trie::new();
    u.insert("日本");
    u.insert("日本語");
    assert_eq!(u.node_count(), 3);
    assert_eq!(u.words_with_prefix("日"), ["日本", "日本語"]);
    let mut v: Trie = ["to", "tea", "to"].into_iter().collect();
    v.extend(vec![String::from("ten")]);
    assert_eq!(v.words_with_prefix("t"), ["tea", "ten", "to"]);
}

#[test]
fn collect_from_str_and_string_items() {
    let t: Trie = ["car", "cart", "car", "", "日本"].into_iter().collect();
    assert_eq!(t.len(), 4, "duplicates are ignored");
    assert!(t.contains("") && t.contains("cart") && t.contains("日本"));
    assert_eq!(t.node_count(), 6); // c, ca, car, cart, 日, 日本
    let owned: Vec<String> = vec!["b".into(), "a".into(), "ab".into()];
    let by_ref: Trie = owned.iter().collect(); // &String items
    let by_value: Trie = owned.into_iter().collect(); // String items
    assert_eq!(by_ref.words_with_prefix(""), ["a", "ab", "b"]);
    assert_eq!(by_value.words_with_prefix(""), ["a", "ab", "b"]);
    let empty: Trie = std::iter::empty::<&str>().collect();
    assert!(empty.is_empty());
    assert_eq!(empty.node_count(), 0);
}

#[test]
fn extend_adds_to_an_existing_trie() {
    let mut t = trie_of(&["dog", "door"]);
    t.extend(["do", "door", "dot"]);
    assert_eq!(t.len(), 4);
    assert_eq!(t.words_with_prefix("do"), ["do", "dog", "door", "dot"]);
    t.extend(vec![String::from("cat")]);
    assert_eq!(t.longest_common_prefix(), Some(String::new()));
    t.extend(Vec::<String>::new());
    assert_eq!(t.len(), 5);
    // Same result as inserting one by one.
    let mut one_by_one = Trie::new();
    for w in ["dog", "door", "do", "dot", "cat"] {
        one_by_one.insert(w);
    }
    assert_eq!(t.node_count(), one_by_one.node_count());
    assert_eq!(t.words_with_prefix(""), one_by_one.words_with_prefix(""));
}

#[test]
fn empty_trie() {
    for t in [Trie::new(), Trie::default()] {
        assert!(t.is_empty());
        assert_eq!(t.len(), 0);
        assert!(!t.contains(""));
        assert!(!t.starts_with(""));
        assert_eq!(t.count_with_prefix(""), 0);
        assert!(t.words_with_prefix("").is_empty());
        assert_eq!(t.longest_common_prefix(), None);
        assert_eq!(t.node_count(), 0);
    }
}

#[test]
fn empty_prefix_matches_once_anything_is_stored() {
    // Walking zero chars always "succeeds"; that alone is not enough.
    let mut t = Trie::new();
    assert!(!t.starts_with(""));
    t.insert("q");
    assert!(t.starts_with(""));
    assert!(!t.contains(""));
    assert_eq!(t.count_with_prefix(""), 1);
    assert_eq!(t.words_with_prefix(""), ["q"]);
    t.remove("q");
    assert!(
        !t.starts_with(""),
        "the root survives removal, but holds no word"
    );
}

#[test]
fn empty_word_can_be_stored_and_removed() {
    let mut t = Trie::new();
    assert!(t.insert(""));
    assert!(!t.insert(""));
    assert!(t.contains(""));
    assert!(t.starts_with(""));
    assert!(!t.starts_with("a"));
    assert_eq!(t.len(), 1);
    assert_eq!(t.node_count(), 0, "the empty word lives at the root");
    assert_eq!(t.words_with_prefix(""), [""]);
    assert_eq!(t.longest_common_prefix(), Some(String::new()));
    t.insert("ab");
    assert_eq!(t.words_with_prefix(""), ["", "ab"]);
    assert!(t.remove(""));
    assert!(!t.contains(""));
    assert!(t.contains("ab"));
    assert_eq!(t.longest_common_prefix(), Some("ab".to_string()));
}

#[test]
fn prefix_of_a_word_is_not_a_word() {
    let mut t = trie_of(&["apple"]);
    assert!(!t.contains("app"));
    assert!(t.starts_with("app"));
    assert!(t.starts_with("apple"), "a word is a prefix of itself");
    assert!(!t.starts_with("apples"));
    // The path for "app" already exists, but no word ended there yet.
    assert!(t.insert("app"));
    assert_eq!(t.len(), 2);
    assert_eq!(t.node_count(), 5);
}

#[test]
fn remove_missing_word_changes_nothing() {
    // Decrementing counts on the way down before checking the word exists
    // corrupts them for every word sharing the path.
    let mut t = trie_of(&["car", "cart"]);
    assert!(!t.remove("ca"), "a prefix, not a word");
    assert!(!t.remove("carts"), "longer than any path");
    assert!(!t.remove("cab"), "path breaks off");
    assert!(!t.remove(""));
    assert_eq!(t.len(), 2);
    assert_eq!(t.count_with_prefix("c"), 2);
    assert_eq!(t.count_with_prefix("car"), 2);
    assert_eq!(t.node_count(), 4);
    assert_eq!(t.words_with_prefix(""), ["car", "cart"]);
}

#[test]
fn remove_prunes_only_the_unshared_tail() {
    let mut t = trie_of(&["car", "cartoon", "cat"]);
    assert_eq!(t.node_count(), 8);
    assert!(t.remove("cartoon"));
    assert_eq!(t.node_count(), 4, "t, to, too, toon below \"car\" must go");
    assert!(t.contains("car") && t.contains("cat"));
    assert!(!t.starts_with("cart"));
    assert!(t.remove("cat"));
    assert_eq!(t.node_count(), 3);
    assert!(t.remove("car"));
    assert_eq!(t.node_count(), 0);
    assert!(t.is_empty());
}

#[test]
fn removing_a_prefix_word_keeps_the_longer_word() {
    let mut t = trie_of(&["car", "cart"]);
    assert!(t.remove("car"));
    assert!(!t.contains("car"));
    assert!(t.contains("cart"));
    assert!(t.starts_with("car"));
    assert_eq!(t.node_count(), 4, "the path is still needed by \"cart\"");
    assert_eq!(t.count_with_prefix("car"), 1);
}

#[test]
fn reinsert_after_remove() {
    let mut t = trie_of(&["dog"]);
    assert!(t.remove("dog"));
    assert!(!t.contains("dog"));
    assert!(t.insert("dog"));
    assert!(t.contains("dog"));
    assert_eq!(t.len(), 1);
    assert_eq!(t.count_with_prefix("do"), 1);
    assert_eq!(t.node_count(), 3);
}

#[test]
fn count_with_prefix_cases() {
    let t = trie_of(&["a", "ab", "abc", "abd", "b", "ba"]);
    assert_eq!(t.count_with_prefix(""), 6);
    assert_eq!(
        t.count_with_prefix("a"),
        4,
        "counts the word ending at the prefix"
    );
    assert_eq!(t.count_with_prefix("ab"), 3);
    assert_eq!(t.count_with_prefix("abc"), 1);
    assert_eq!(t.count_with_prefix("abcd"), 0);
    assert_eq!(t.count_with_prefix("c"), 0);
}

#[test]
fn words_with_prefix_are_sorted_without_sorting() {
    let t = trie_of(&["pear", "peach", "pea", "apple", "peanut", "pecan", "plum"]);
    assert_eq!(
        t.words_with_prefix("pea"),
        ["pea", "peach", "peanut", "pear"]
    );
    assert_eq!(
        t.words_with_prefix("pe"),
        ["pea", "peach", "peanut", "pear", "pecan"]
    );
    assert_eq!(t.words_with_prefix("pecan"), ["pecan"]);
    assert!(t.words_with_prefix("pecans").is_empty());
    assert!(t.words_with_prefix("z").is_empty());
    assert_eq!(t.words_with_prefix("").len(), 7);
}

#[test]
fn sorted_means_code_point_order() {
    // Uppercase before lowercase, ASCII before accented letters.
    let t = trie_of(&["é", "zoo", "Zoo", "apple", "Äpfel", "ß", "a"]);
    let words = t.words_with_prefix("");
    let mut expected = words.clone();
    expected.sort();
    assert_eq!(words, expected);
    assert_eq!(words, ["Zoo", "a", "apple", "zoo", "Äpfel", "ß", "é"]);
}

#[test]
fn longest_common_prefix_cases() {
    assert_eq!(
        trie_of(&["flower"]).longest_common_prefix(),
        Some("flower".to_string())
    );
    assert_eq!(
        trie_of(&["flower", "flow", "flight"]).longest_common_prefix(),
        Some("fl".to_string())
    );
    // A stored word stops the prefix even if the path continues.
    assert_eq!(
        trie_of(&["inter", "interval"]).longest_common_prefix(),
        Some("inter".to_string())
    );
    assert_eq!(
        trie_of(&["dog", "cat"]).longest_common_prefix(),
        Some(String::new())
    );
    assert_eq!(
        trie_of(&["", "x"]).longest_common_prefix(),
        Some(String::new())
    );
    let mut t = trie_of(&["flower", "flow", "flight"]);
    t.remove("flight");
    assert_eq!(t.longest_common_prefix(), Some("flow".to_string()));
}

#[test]
fn non_ascii_words() {
    let mut t = trie_of(&["naïve", "naïveté", "nai", "日本語", "日本", "🦀rust", "🦀"]);
    assert!(t.contains("naïve") && t.contains("nai"));
    assert!(!t.contains("naï"));
    assert!(t.starts_with("naï"));
    assert_eq!(t.count_with_prefix("na"), 3);
    assert_eq!(t.words_with_prefix("naï"), ["naïve", "naïveté"]);
    assert_eq!(t.words_with_prefix("🦀"), ["🦀", "🦀rust"]);
    assert!(t.remove("日本"));
    assert_eq!(t.words_with_prefix("日"), ["日本語"]);
    assert!(t.remove("日本語"));
    assert!(!t.starts_with("日"));
    assert_eq!(
        trie_of(&["日本語", "日本人"]).longest_common_prefix(),
        Some("日本".to_string())
    );
}

#[test]
fn node_count_is_in_chars_not_bytes() {
    // 'é' is 2 bytes, '日' 3, '🦀' 4: a byte trie would report 9 and 10.
    let t = trie_of(&["é日🦀"]);
    assert_eq!(t.node_count(), 3);
    let t = trie_of(&["é日🦀", "é日"]);
    assert_eq!(t.node_count(), 3);
    let t = trie_of(&["éa", "èa"]); // same first byte, different chars
    assert_eq!(t.node_count(), 4);
    assert_eq!(t.longest_common_prefix(), Some(String::new()));
}

#[test]
fn long_words() {
    let long = "ab".repeat(500);
    let mut t = trie_of(&[&long, &long[..999]]);
    assert!(t.contains(&long));
    assert!(t.contains(&long[..999]));
    assert!(!t.contains(&long[..998]));
    assert_eq!(t.node_count(), 1000);
    assert_eq!(t.longest_common_prefix().as_deref(), Some(&long[..999]));
    assert!(t.remove(&long));
    assert_eq!(t.node_count(), 999);
}

#[test]
fn randomized_vs_model() {
    let alphabet = ['a', 'b', 'é', '日'];
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for round in 0..20 {
        let mut t = Trie::new();
        let mut model = BTreeSet::new();
        for step in 0..300 {
            let w = rng.word(0, 5, &alphabet);
            let ctx = format!("round {round} step {step}, word {w:?}, model {model:?}");
            match rng.below(7) {
                0 | 1 => assert_eq!(t.insert(&w), model.insert(w.clone()), "insert: {ctx}"),
                2 => assert_eq!(t.remove(&w), model.remove(&w), "remove: {ctx}"),
                3 => assert_eq!(t.contains(&w), model.contains(&w), "contains: {ctx}"),
                4 => assert_eq!(
                    t.count_with_prefix(&w),
                    brute_words(&model, &w).len(),
                    "count_with_prefix: {ctx}"
                ),
                5 => assert_eq!(
                    t.words_with_prefix(&w),
                    brute_words(&model, &w),
                    "words: {ctx}"
                ),
                _ => assert_eq!(
                    t.starts_with(&w),
                    !brute_words(&model, &w).is_empty(),
                    "starts_with: {ctx}"
                ),
            }
            assert_eq!(t.len(), model.len(), "len: {ctx}");
            assert_eq!(t.longest_common_prefix(), brute_lcp(&model), "lcp: {ctx}");
            assert_eq!(
                t.node_count(),
                brute_node_count(&model),
                "node_count: {ctx}"
            );
        }
    }
}

#[test]
fn removing_everything_leaves_no_nodes() {
    let alphabet: Vec<char> = ('a'..='z').collect();
    let mut rng = Rng(77);
    let mut t = Trie::new();
    let mut words = Vec::new();
    for _ in 0..2000 {
        let w = rng.word(1, 10, &alphabet);
        if t.insert(&w) {
            words.push(w);
        }
    }
    assert!(t.node_count() > 0);
    for w in &words {
        assert!(t.remove(w), "remove({w:?})");
    }
    assert!(t.is_empty());
    assert_eq!(t.node_count(), 0, "every branch should have been pruned");
    assert_eq!(t.longest_common_prefix(), None);
}

#[test]
fn many_counts_of_short_prefixes() {
    // 10^4 words of 8-16 letters, then 7000 one-letter prefix counts. Walking
    // the ~4000 nodes below the prefix for every query is ~3 * 10^7 node
    // visits; a count stored in each node makes it one lookup per query.
    let alphabet: Vec<char> = ('a'..='z').collect();
    let mut rng = Rng(2024);
    let mut t = Trie::new();
    let mut model = BTreeSet::new();
    for _ in 0..10_000 {
        let w = rng.word(8, 16, &alphabet);
        t.insert(&w);
        model.insert(w);
    }
    assert_eq!(t.len(), model.len());
    let mut per_letter = [0usize; 26];
    for w in &model {
        per_letter[usize::from(w.as_bytes()[0] - b'a')] += 1;
    }
    for i in 0..7_000 {
        let letter = i % 26;
        let prefix = char::from(b'a' + letter as u8).to_string();
        assert_eq!(
            t.count_with_prefix(&prefix),
            per_letter[letter],
            "prefix {prefix}"
        );
    }
    assert_eq!(t.count_with_prefix(""), model.len());
}
