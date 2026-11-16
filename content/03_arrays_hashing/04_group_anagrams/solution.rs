// Group Anagrams — reference solution.
// Key each word by its 26-letter count array in a HashMap of borrowed words.
// Time O(n * k), space O(n * k) for keys and groups.

use std::collections::HashMap;

pub fn group_anagrams<'a>(words: &[&'a str]) -> Vec<Vec<&'a str>> {
    let mut groups: HashMap<[u8; 26], Vec<&'a str>> = HashMap::new();
    for &word in words {
        let mut key = [0u8; 26];
        for b in word.bytes() {
            key[usize::from(b - b'a')] += 1;
        }
        groups.entry(key).or_default().push(word);
    }
    groups.into_values().collect()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
