## Description

Given a rectangular matrix, return the length of the longest **strictly
increasing** path in it.

From any cell you may step to one of its four neighbours — up, down, left or
right. Diagonal moves and wrapping around the edges are not allowed. The length
of a path is the number of cells on it, so any non-empty matrix has a path of
length at least `1`; a matrix with no cells has `0`.

The function is generic: the cells can hold anything that can be ordered —
integers of any width, `char`s, strings.

## Signature

```rust
pub fn longest_increasing_path<T: Ord>(matrix: &[Vec<T>]) -> usize
```

The algorithm only compares cells, so `T: Ord` is all it needs (no `Copy`, no
arithmetic). The matrix is borrowed as a slice of rows. The result is a number
of cells, hence `usize`.

## Examples

```rust
// One longest path is 1 → 2 → 6 → 9.
assert_eq!(longest_increasing_path(&[vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]), 4);
// 3 → 4 → 5 → 6; moving diagonally is not allowed.
assert_eq!(longest_increasing_path(&[vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]]), 4);
assert_eq!(longest_increasing_path(&[vec![1]]), 1);
assert_eq!(longest_increasing_path::<i32>(&[]), 0);   // no cells
assert_eq!(longest_increasing_path(&[vec!['a', 'b'], vec!['d', 'c']]), 4);
```

## Constraints

- `0 <= rows, cols <= 200`; every row has the same length.

A longest path can visit every cell (`40 000` cells). If you use recursion, the
tests run that case on a thread with a large stack, so a recursive DFS is fine.

## Rust notes

- A flat `vec![0; rows * cols]` indexed by `r * cols + c` is one allocation and
  is easy to pass around mutably, unlike borrowing a `Vec<Vec<_>>` in a closure.
- Neighbours without signed arithmetic: `r.checked_sub(1)` is `None` at the top
  edge; collect the candidates as `Option<(usize, usize)>`s and `flatten()`
  them, then filter `r < rows && c < cols`.
- `sort_unstable_by_key(|&(r, c)| Reverse(&matrix[r][c]))` sorts cell
  coordinates by value, largest first, without cloning any `T` — the key borrows
  from the matrix, not from the element being sorted.
- Recursive helpers that need the matrix and a memo table take them as
  parameters (`fn dfs<T: Ord>(m: &[Vec<T>], memo: &mut [usize], …)`): closures
  can't recurse.
