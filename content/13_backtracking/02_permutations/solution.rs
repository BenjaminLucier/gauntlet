// Permutations — reference solution.
// In-place swap backtracking on a private copy: fix position i by swapping each
// remaining element into it, recurse, swap back. Time O(n * n!), space O(n) extra.

pub fn permutations<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    fn go<T: Clone>(i: usize, work: &mut [T], out: &mut Vec<Vec<T>>) {
        if i == work.len() {
            out.push(work.to_vec());
            return;
        }
        for j in i..work.len() {
            work.swap(i, j);
            go(i + 1, work, out);
            work.swap(i, j);
        }
    }

    let mut work = items.to_vec();
    let mut out = Vec::with_capacity((1..=items.len()).product());
    go(0, &mut work, &mut out);
    out
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
