## Description

Place `n` chess queens on an `n x n` board so that no two attack each other —
no two queens share a row, a column, or a diagonal.

Return **every** distinct placement. Each board is a list of `n` strings, one
per row, where `'Q'` marks a queen and `'.'` an empty square. Boards may be
returned in **any order**. For `n = 0` there is exactly one (empty) board.

## Signature

```rust
pub fn solve_n_queens(n: usize) -> Vec<Vec<String>>
```

`n` is a size, so `usize`. Each board is a `Vec<String>` of rows: it prints
nicely, and tests can compare it straight against string literals
(`vec![".Q..", "...Q", …]`). A board with no solutions is just an empty
outer `Vec` — no sentinel.

## Examples

```rust
let mut boards = solve_n_queens(4);
boards.sort();
assert_eq!(
    boards,
    vec![
        vec!["..Q.", "Q...", "...Q", ".Q.."],   // '.' sorts before 'Q'
        vec![".Q..", "...Q", "Q...", "..Q."],
    ]
);
assert_eq!(solve_n_queens(1), vec![vec!["Q"]]);
assert!(solve_n_queens(3).is_empty());         // 2 and 3 have no solution
assert_eq!(solve_n_queens(8).len(), 92);
```

## Constraints

- `0 <= n <= 9`

## Rust notes

- Keep the search state compact — a `&mut Vec<usize>` of queen columns, one
  per row — and only build `String`s for complete boards. `push`, recurse,
  `pop` is the whole backtracking step.
- For a diagonal index like `r + n - 1 - c`, order the
  arithmetic so no intermediate goes below zero: `r - c` underflows in `usize`.
- A row string is `(0..n).map(|j| if j == col { 'Q' } else { '.' }).collect()`;
  alternatively `".".repeat(n)` and `replace_range`.
- When several pieces of state travel through the recursion, a small struct
  with one `&mut self` method keeps the signature readable.
