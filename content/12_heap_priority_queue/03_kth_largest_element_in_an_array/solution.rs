// Kth Largest Element in an Array — reference solution.
// Quickselect on a copy via `select_nth_unstable` at ascending index n - k
// (std's introselect is O(n) and robust to sorted input and duplicates).
// Time O(n) average, space O(n) for the copy.

pub fn kth_largest<T: Ord + Copy>(items: &[T], k: usize) -> Option<T> {
    let index = items.len().checked_sub(k).filter(|_| k > 0)?;
    let mut copy = items.to_vec();
    let (_, &mut kth, _) = copy.select_nth_unstable(index);
    Some(kth)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
