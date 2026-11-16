// Trie — reference solution.
// Each node keeps its children in a `BTreeMap<char, Node>` (so depth-first walks
// come out sorted), an end-of-word flag and the number of words in its subtree.
// Point operations are O(L log σ); `remove` cuts the first child whose count would hit 0.

use std::collections::BTreeMap;

#[derive(Default)]
struct Node {
    children: BTreeMap<char, Node>,
    is_word: bool,
    /// Words stored in this subtree, including one ending here.
    count: usize,
}

impl Node {
    /// Appends every word below this node, in sorted order, to `out`.
    /// `buf` holds the path from the root to this node.
    fn collect(&self, buf: &mut String, out: &mut Vec<String>) {
        if self.is_word {
            out.push(buf.clone());
        }
        for (&c, child) in &self.children {
            buf.push(c);
            child.collect(buf, out);
            buf.pop();
        }
    }

    /// Nodes below this one (not counting itself).
    fn descendants(&self) -> usize {
        self.children.values().map(|c| 1 + c.descendants()).sum()
    }
}

pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: Node::default(),
        }
    }

    /// The node reached by following `s` from the root, if that path exists.
    fn find(&self, s: &str) -> Option<&Node> {
        s.chars()
            .try_fold(&self.root, |node, c| node.children.get(&c))
    }

    /// Adds `word`; returns `true` if it wasn't in the trie before.
    pub fn insert(&mut self, word: &str) -> bool {
        if self.contains(word) {
            return false;
        }
        let mut node = &mut self.root;
        node.count += 1;
        for c in word.chars() {
            node = node.children.entry(c).or_default();
            node.count += 1;
        }
        node.is_word = true;
        true
    }

    pub fn contains(&self, word: &str) -> bool {
        self.find(word).is_some_and(|node| node.is_word)
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        self.count_with_prefix(prefix) > 0
    }

    /// Deletes `word`, pruning nodes that no longer lead to any word.
    /// Returns `true` if it was there.
    pub fn remove(&mut self, word: &str) -> bool {
        if !self.contains(word) {
            return false;
        }
        let mut node = &mut self.root;
        node.count -= 1;
        for c in word.chars() {
            let child_count = node.children.get(&c).expect("word is stored").count;
            if child_count == 1 {
                // `word` was the only word below `c`: drop the whole branch.
                node.children.remove(&c);
                return true;
            }
            node = node.children.get_mut(&c).expect("word is stored");
            node.count -= 1;
        }
        node.is_word = false;
        true
    }

    pub fn len(&self) -> usize {
        self.root.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// How many stored words start with `prefix`.
    pub fn count_with_prefix(&self, prefix: &str) -> usize {
        self.find(prefix).map_or(0, |node| node.count)
    }

    /// The stored words that start with `prefix`, sorted.
    pub fn words_with_prefix(&self, prefix: &str) -> Vec<String> {
        let mut out = Vec::new();
        if let Some(node) = self.find(prefix) {
            node.collect(&mut prefix.to_string(), &mut out);
        }
        out
    }

    /// The longest common prefix of every stored word; `None` if empty.
    pub fn longest_common_prefix(&self) -> Option<String> {
        if self.is_empty() {
            return None;
        }
        let mut prefix = String::new();
        let mut node = &self.root;
        // Stop where a word ends or the path branches.
        while !node.is_word && node.children.len() == 1 {
            let (&c, child) = node.children.iter().next().expect("exactly one child");
            prefix.push(c);
            node = child;
        }
        Some(prefix)
    }

    /// Number of nodes, not counting the root.
    pub fn node_count(&self) -> usize {
        self.root.descendants()
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: AsRef<str>> FromIterator<S> for Trie {
    fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self {
        let mut trie = Trie::new();
        trie.extend(iter);
        trie
    }
}

impl<S: AsRef<str>> Extend<S> for Trie {
    /// Inserts every word; duplicates are ignored, as with `insert`.
    fn extend<I: IntoIterator<Item = S>>(&mut self, iter: I) {
        for word in iter {
            self.insert(word.as_ref());
        }
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
