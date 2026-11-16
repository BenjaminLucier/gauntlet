// Tests for Word Ladder — shared by template.rs, solution.rs and your attempt.
use super::*;

use std::collections::{BTreeSet, HashSet, VecDeque};

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

fn one_apart(a: &str, b: &str) -> bool {
    a.bytes().zip(b.bytes()).filter(|(x, y)| x != y).count() == 1
}

/// BFS comparing every pair of words directly: O(N² · L).
fn brute_ladder(begin: &str, end: &str, list: &[&str]) -> Option<usize> {
    if !list.contains(&end) {
        return None;
    }
    let mut dist = vec![0; list.len()];
    let mut queue = VecDeque::new();
    for (i, x) in list.iter().enumerate() {
        if one_apart(begin, x) {
            dist[i] = 2;
            queue.push_back(i);
        }
    }
    while let Some(i) = queue.pop_front() {
        if list[i] == end {
            return Some(dist[i]);
        }
        for j in 0..list.len() {
            if dist[j] == 0 && list[j] != begin && one_apart(list[i], list[j]) {
                dist[j] = dist[i] + 1;
                queue.push_back(j);
            }
        }
    }
    None
}

fn hypercube(count: u32) -> Vec<String> {
    (0..count)
        .map(|m| (0..12).map(|i| if m >> i & 1 == 1 { 'b' } else { 'a' }).collect())
        .collect()
}

#[test]
fn example_1() {
    let words = ["hot", "dot", "dog", "lot", "log", "cog"];
    assert_eq!(ladder_length("hit", "cog", &words), Some(5));
}

#[test]
fn example_2() {
    let words = ["hot", "dot", "dog", "lot", "log"];
    assert_eq!(ladder_length("hit", "cog", &words), None);
}

#[test]
fn example_3() {
    assert_eq!(ladder_length("cat", "cot", &["cot"]), Some(2));
}

#[test]
fn one_step() {
    assert_eq!(ladder_length("a", "c", &["a", "b", "c"]), Some(2));
    assert_eq!(ladder_length("hot", "dot", &["dot"]), Some(2));
}

#[test]
fn single_letter_words() {
    assert_eq!(ladder_length("a", "z", &["z"]), Some(2));
}

#[test]
fn end_word_missing_even_if_adjacent() {
    // "hit" -> "hot" is one letter, but "hot" isn't in the list.
    assert_eq!(ladder_length("hit", "hot", &["hat", "hut"]), None);
}

#[test]
fn begin_word_in_list_is_not_revisited() {
    assert_eq!(ladder_length("hot", "dog", &["hot", "dog", "dot"]), Some(3));
}

#[test]
fn no_path_disconnected() {
    assert_eq!(ladder_length("aaa", "bbb", &["aab", "bbb"]), None);
}

#[test]
fn shortest_not_first_found() {
    // A DFS might take the long way: aa -> ab -> bb -> bc vs aa -> ac -> bc.
    assert_eq!(ladder_length("aa", "bc", &["ab", "bb", "ac", "bc"]), Some(3));
}

#[test]
fn two_letter_change_is_not_one_step() {
    assert_eq!(ladder_length("ab", "ba", &["ba"]), None);
    assert_eq!(ladder_length("ab", "ba", &["ba", "bb"]), Some(3));
}

#[test]
fn borrows_from_owned_strings() {
    let owned: Vec<String> = ["hot", "dot", "dog"].iter().map(|s| s.to_string()).collect();
    let words: Vec<&str> = owned.iter().map(String::as_str).collect();
    let begin = String::from("hit");
    assert_eq!(ladder_length(&begin, "dog", &words), Some(4));
    // The caller still owns everything afterwards.
    assert_eq!(owned.len(), 3);
}

#[test]
fn randomized_vs_pairwise_bfs() {
    let mut rng = Rng(0x2468_ACE0_1357_9BDF);
    let letters = b"abc";
    let rand_word = |rng: &mut Rng, len: usize| -> String {
        (0..len).map(|_| letters[rng.below(3) as usize] as char).collect()
    };
    for _ in 0..400 {
        let len = 1 + rng.below(4) as usize;
        let begin = rand_word(&mut rng, len);
        let mut end = rand_word(&mut rng, len);
        while end == begin {
            end = rand_word(&mut rng, len);
        }
        let mut set = BTreeSet::new();
        let size = rng.below(12);
        for _ in 0..size {
            set.insert(rand_word(&mut rng, len));
        }
        if rng.below(3) != 0 {
            set.insert(end.clone());
        }
        let list: Vec<&str> = set.iter().map(String::as_str).collect();
        let expected = brute_ladder(&begin, &end, &list);
        assert_eq!(
            ladder_length(&begin, &end, &list),
            expected,
            "begin = {begin:?}, end = {end:?}, list = {list:?}"
        );
    }
}

#[test]
fn large_random_dictionary() {
    // 5000 random 5-letter words plus a forced 5-step path: answer is exactly 6
    // (Hamming distance 5 needs at least 5 steps).
    let mut rng = Rng(9001);
    let mut set = HashSet::new();
    for p in ["jaaaa", "jjaaa", "jjjaa", "jjjja", "jjjjj"] {
        set.insert(p.to_string());
    }
    while set.len() < 5000 {
        let word: String = (0..5).map(|_| (b'a' + rng.below(10) as u8) as char).collect();
        set.insert(word);
    }
    set.remove("aaaaa");
    let list: Vec<&str> = set.iter().map(String::as_str).collect();
    assert_eq!(ladder_length("aaaaa", "jjjjj", &list), Some(6));
}

#[test]
fn large_hypercube() {
    // All 4096 words of length 12 over {a, b}: every word has 12 neighbours and
    // a shortest ladder flips the 12 letters one at a time.
    let owned = hypercube(4096);
    let list: Vec<&str> = owned.iter().map(String::as_str).collect();
    assert_eq!(ladder_length(&"a".repeat(12), &"b".repeat(12), &list), Some(13));
}

#[test]
fn large_no_path() {
    let mut owned = hypercube(4095);
    owned.push("zzzzzzzzzzzz".to_string());
    let list: Vec<&str> = owned.iter().map(String::as_str).collect();
    assert_eq!(ladder_length(&"a".repeat(12), "zzzzzzzzzzzz", &list), None);
}
