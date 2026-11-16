// Word Search II — reference solution.
// Build a trie of all words, then DFS from each cell walking board and trie
// together. Found words are taken out of their node, and per-node counts of
// unfound words let the search skip exhausted branches.
// Worst case O(m * n * 4 * 3^(L-1)) time, O(total word length) space.

/// Marks a cell that is on the current path.
const VISITED: u8 = b'#';

#[derive(Default)]
struct Node<'w> {
    children: [Option<Box<Node<'w>>>; 26],
    /// The word that ends here, until it has been found.
    word: Option<&'w str>,
    /// How many words in this subtree haven't been found yet.
    remaining: usize,
}

/// Child index of a lowercase ASCII letter.
fn slot(b: u8) -> usize {
    assert!(b.is_ascii_lowercase(), "letters must be lowercase ASCII");
    usize::from(b - b'a')
}

pub fn find_words<'w>(board: &[&str], words: &[&'w str]) -> Vec<&'w str> {
    let mut root = Node::default();
    for &word in words {
        root.remaining += 1;
        let mut node = &mut root;
        for b in word.bytes() {
            node = node.children[slot(b)].get_or_insert_with(Box::default);
            node.remaining += 1;
        }
        node.word = Some(word);
    }

    let mut grid: Vec<Vec<u8>> = board.iter().map(|row| row.as_bytes().to_vec()).collect();
    let mut found = Vec::new();
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if root.remaining == 0 {
                return found;
            }
            let n = search(&mut grid, r, c, &mut root, &mut found);
            root.remaining -= n;
        }
    }
    found
}

/// Explores paths starting at cell `(r, c)` below trie node `parent`; returns
/// how many new words were found.
fn search<'w>(
    grid: &mut [Vec<u8>],
    r: usize,
    c: usize,
    parent: &mut Node<'w>,
    found: &mut Vec<&'w str>,
) -> usize {
    // Out-of-range coordinates (including wrapped-around `0 - 1`) just miss.
    let Some(&letter) = grid.get(r).and_then(|row| row.get(c)) else {
        return 0;
    };
    if letter == VISITED {
        return 0;
    }
    let Some(node) = parent.children[slot(letter)].as_deref_mut() else {
        return 0;
    };
    if node.remaining == 0 {
        return 0;
    }

    let mut count = 0;
    if let Some(word) = node.word.take() {
        found.push(word);
        count += 1;
    }
    grid[r][c] = VISITED;
    for (nr, nc) in [(r.wrapping_sub(1), c), (r + 1, c), (r, c.wrapping_sub(1)), (r, c + 1)] {
        count += search(grid, nr, nc, node, found);
    }
    grid[r][c] = letter;
    node.remaining -= count;
    count
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
