## Description

Given an `m x n` matrix, return all of its elements in **spiral order**:
start at the top-left corner, walk right along the top row, then down the right
column, then left along the bottom row, then up the left column, and keep
spiralling inwards until every element has been visited exactly once.

The function is generic over the element type.

## Signature

```rust
pub fn spiral_order<T: Clone>(matrix: &[Vec<T>]) -> Vec<T>
```

The matrix is only read, so it is borrowed. The output is a new `Vec<T>` of
owned elements, which means each one has to be cloned out of the matrix —
hence `T: Clone` (for `Copy` types such as `i32` that clone is free). An empty
matrix, or one whose rows are empty, gives an empty vector.

## Examples

```rust
let m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
assert_eq!(spiral_order(&m), [1, 2, 3, 6, 9, 8, 7, 4, 5]);

let m = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
assert_eq!(spiral_order(&m), [1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]);

let m = vec![vec!['a', 'b'], vec!['d', 'c']];
assert_eq!(spiral_order(&m), ['a', 'b', 'c', 'd']);

assert!(spiral_order::<i32>(&[]).is_empty());
```

## Constraints

- All rows have the same length
- `0 <= m, n <= 10`

## Rust notes

- With `usize` boundaries, `right - 1` or `bottom - 1` on an exhausted
  dimension underflows and panics. Half-open bounds (`left..right`,
  `top..bottom`) usually keep every subtraction safe.
- Whole row segments are slices: `matrix[top][left..right]` can be pushed with
  `out.extend(slice.iter().cloned())` — or `.rev()` for the bottom row.
- A column is `matrix[top..bottom].iter().map(|row| row[c].clone())`.
- `matrix.first().map_or(0, Vec::len)` gets the width without indexing into a
  possibly empty matrix.
