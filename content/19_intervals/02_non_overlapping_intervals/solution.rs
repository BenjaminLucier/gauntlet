// Non-overlapping Intervals — reference solution.
// Activity selection: sort by end and fold, keeping each interval that starts
// at or after the last kept end; everything else is removed.
// Time O(n log n), space O(1) extra.

pub fn min_removals(mut intervals: Vec<(i32, i32)>) -> usize {
    intervals.sort_unstable_by_key(|&(_, end)| end);
    let (kept, _) = intervals
        .iter()
        .fold((0, i32::MIN), |(kept, last_end), &(start, end)| {
            if start >= last_end {
                (kept + 1, end)
            } else {
                (kept, last_end)
            }
        });
    intervals.len() - kept
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
