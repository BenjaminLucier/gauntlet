// Design Add and Search Words Data Structure — reference solution.
// A trie of boxed nodes; search is a DFS over the pattern's bytes that follows
// one child per letter and every child on '.'. insert O(L); search O(L)
// without wildcards, bounded by the trie's size with them.

#[derive(Default)]
struct Node {
    children: [Option<Box<Node>>; 26],
    is_word: bool,
}

pub struct WordDictionary {
    root: Node,
}

/// Child index of a lowercase ASCII letter.
fn slot(b: u8) -> usize {
    assert!(b.is_ascii_lowercase(), "words must be lowercase ASCII letters");
    usize::from(b - b'a')
}

impl WordDictionary {
    pub fn new() -> Self {
        Self { root: Node::default() }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for b in word.bytes() {
            node = node.children[slot(b)].get_or_insert_with(Box::default);
        }
        node.is_word = true;
    }

    /// Does some stored word match `pattern`, where `.` matches any one letter?
    pub fn search(&self, pattern: &str) -> bool {
        matches(&self.root, pattern.as_bytes())
    }
}

impl Default for WordDictionary {
    fn default() -> Self {
        Self::new()
    }
}

/// Does a word in the subtree of `node` match the rest of the pattern?
fn matches(node: &Node, pattern: &[u8]) -> bool {
    match pattern {
        [] => node.is_word,
        [b'.', rest @ ..] => node.children.iter().flatten().any(|c| matches(c, rest)),
        [b, rest @ ..] => node.children[slot(*b)]
            .as_deref()
            .is_some_and(|c| matches(c, rest)),
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
