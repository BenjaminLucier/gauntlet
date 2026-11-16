## Description

Given an `m x n` grid of letters `board` and a list of distinct `words`,
return every word from the list that can be traced on the board.

A word is traced by a sequence of cells where consecutive cells are
horizontally or vertically adjacent (no diagonals), and **no cell is used
more than once** within the same word. Return each found word once, in any
order.

The board is given as rows of lowercase ASCII letters, all the same length:
`&["oaan", "etae"]` is a 2 x 4 board.

## Signature

```rust
pub fn find_words<'w>(board: &[&str], words: &[&'w str]) -> Vec<&'w str>
```

Both inputs are only read, so both are borrowed. The answer is a subset of
`words`, so instead of cloning each found word into a new `String`, it hands
back the caller's own `&str`s: the lifetime `'w` says the results borrow from
`words` (not from `board`), so they stay valid as long as the words do.

## Examples

```rust
let board = ["oaan", "etae", "ihkr", "iflv"];
let mut found = find_words(&board, &["oath", "pea", "eat", "rain"]);
found.sort_unstable(); // any order is accepted
assert_eq!(found, ["eat", "oath"]);

// "abcb" would need to reuse the cell 'b' (and b-c isn't adjacent anyway).
assert!(find_words(&["ab", "cd"], &["abcb"]).is_empty());
```

## Constraints

- `1 <= m, n <= 12` (an empty board finds nothing)
- `0 <= words.len() <= 3 * 10^4`, `1 <= word.len() <= 10`
- Board cells and words are lowercase ASCII letters; all words are distinct

## Rust notes

- The trie's nodes can store `Option<&'w str>` for the word ending there —
  a struct holding a reference needs a lifetime parameter (`Node<'w>`), and
  that is exactly what lets you return the borrowed words without cloning.
  `Option::take` hands the word out once and leaves `None`.
- To mark cells as visited, copy the board into a `Vec<Vec<u8>>` you can
  overwrite and restore while backtracking (or keep a separate `visited`
  grid).
- Neighbour arithmetic on `usize` can't go below zero: `r.wrapping_sub(1)`
  produces a huge index that `grid.get(r)` simply rejects, which keeps the
  bounds checks in one place.
- The recursive DFS needs `&mut` access to both the grid and the current trie
  node at once; passing them as separate parameters keeps the borrows
  disjoint.
