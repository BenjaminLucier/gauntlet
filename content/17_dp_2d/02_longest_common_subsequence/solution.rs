// Longest Common Subsequence — reference solution.
// prev[j] = LCS of a[..i] and b[..j]; a match extends the diagonal, otherwise take the
// better of dropping one element. Two rolling rows. Time O(m * n), space O(n).

use std::mem;

pub fn longest_common_subsequence<T: Eq>(a: &[T], b: &[T]) -> usize {
    let mut prev = vec![0; b.len() + 1];
    let mut cur = vec![0; b.len() + 1];
    for x in a {
        for (j, y) in b.iter().enumerate() {
            cur[j + 1] = if x == y {
                prev[j] + 1
            } else {
                prev[j + 1].max(cur[j])
            };
        }
        mem::swap(&mut prev, &mut cur);
    }
    prev[b.len()]
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
