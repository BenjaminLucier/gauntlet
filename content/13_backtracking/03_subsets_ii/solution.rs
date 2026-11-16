// Subsets II — reference solution.
// Sort a copy; record the path at every node and, within one depth, skip an
// item equal to its predecessor so each multiset is generated exactly once.
// Time O(n * 2^n), space O(n) besides the output.

pub fn subsets_with_dup<T: Ord + Clone>(items: &[T]) -> Vec<Vec<T>> {
    fn go<T: Ord + Clone>(rest: &[T], path: &mut Vec<T>, out: &mut Vec<Vec<T>>) {
        out.push(path.clone());
        for (i, item) in rest.iter().enumerate() {
            if i > 0 && *item == rest[i - 1] {
                continue; // same value already tried at this depth
            }
            path.push(item.clone());
            go(&rest[i + 1..], path, out);
            path.pop();
        }
    }

    let mut sorted = items.to_vec();
    sorted.sort_unstable();
    let mut out = Vec::new();
    go(&sorted, &mut Vec::with_capacity(sorted.len()), &mut out);
    out
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
