// Trie — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no HashSet/BTreeSet/Vec of whole words inside the trie; no sorting in words_with_prefix.
#![allow(unused_imports, unused_variables, dead_code)]

use std::collections::{BTreeMap, HashMap};

pub struct Trie {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: (),
}

impl Trie {
    pub fn new() -> Self {
        todo!()
    }

    /// Adds `word`; returns `true` if it wasn't in the trie before.
    pub fn insert(&mut self, word: &str) -> bool {
        todo!()
    }

    pub fn contains(&self, word: &str) -> bool {
        todo!()
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        todo!()
    }

    /// Deletes `word`, pruning nodes that no longer lead to any word.
    /// Returns `true` if it was there.
    pub fn remove(&mut self, word: &str) -> bool {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// How many stored words start with `prefix`.
    pub fn count_with_prefix(&self, prefix: &str) -> usize {
        todo!()
    }

    /// The stored words that start with `prefix`, sorted.
    pub fn words_with_prefix(&self, prefix: &str) -> Vec<String> {
        todo!()
    }

    /// The longest common prefix of every stored word; `None` if empty.
    pub fn longest_common_prefix(&self) -> Option<String> {
        todo!()
    }

    /// Number of nodes, not counting the root.
    pub fn node_count(&self) -> usize {
        todo!()
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

/// Works for `&str`, `String`, `&String`, ...: anything that is `AsRef<str>`.
impl<S: AsRef<str>> FromIterator<S> for Trie {
    fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self {
        todo!()
    }
}

impl<S: AsRef<str>> Extend<S> for Trie {
    /// Inserts every word; duplicates are ignored, as with `insert`.
    fn extend<I: IntoIterator<Item = S>>(&mut self, iter: I) {
        todo!()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
