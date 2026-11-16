// Top K Frequent Elements — reference solution.
// Count with a HashMap of references, bucket values by count, then read the
// buckets from the highest count down, cloning only the k winners.
// Time O(n), space O(n).

use std::collections::HashMap;
use std::hash::Hash;

pub fn top_k_frequent<T: Hash + Eq + Clone>(items: &[T], k: usize) -> Vec<T> {
    let mut counts: HashMap<&T, usize> = HashMap::new();
    for item in items {
        *counts.entry(item).or_insert(0) += 1;
    }
    // buckets[c] holds the values that occur exactly c times (1 <= c <= n).
    let mut buckets: Vec<Vec<&T>> = vec![Vec::new(); items.len() + 1];
    for (item, count) in counts {
        buckets[count].push(item);
    }
    buckets
        .into_iter()
        .rev()
        .flatten()
        .take(k)
        .cloned()
        .collect()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
