// Longest Increasing Subsequence — reference solution.
// Patience sorting: tails[k] = smallest last item of an increasing subsequence
// of length k+1; each item replaces the first tail >= it (found with
// partition_point) or extends the list. Time O(n log n), space O(n).

pub fn length_of_lis<T: Ord>(items: &[T]) -> usize {
    let mut tails: Vec<&T> = Vec::with_capacity(items.len());
    for item in items {
        let k = tails.partition_point(|&tail| tail < item);
        match tails.get_mut(k) {
            Some(slot) => *slot = item,
            None => tails.push(item),
        }
    }
    tails.len()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
