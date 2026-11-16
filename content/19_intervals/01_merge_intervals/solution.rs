// Merge Intervals — reference solution.
// Sort by start, then coalesce neighbours in place with dedup_by: an interval
// starting at or before the kept one's end is absorbed into it.
// Time O(n log n), space O(1) extra beyond the reused buffer.

pub fn merge_intervals(mut intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    intervals.sort_unstable_by_key(|&(start, _)| start);
    intervals.dedup_by(|next, kept| {
        let overlaps = next.0 <= kept.1;
        if overlaps {
            kept.1 = kept.1.max(next.1);
        }
        overlaps
    });
    intervals
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
