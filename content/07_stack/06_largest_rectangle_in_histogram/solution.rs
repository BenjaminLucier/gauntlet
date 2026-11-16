// Largest Rectangle in Histogram — reference solution.
// Monotonic increasing stack of indices; when a shorter bar (or a final
// height-0 bar past the end) arrives, each popped bar's maximal width is known.
// Areas are computed in u64. Time O(n), space O(n).

pub fn largest_rectangle_area(heights: &[u32]) -> u64 {
    let mut rising: Vec<usize> = Vec::with_capacity(heights.len());
    let mut best = 0;
    for right in 0..=heights.len() {
        let h = heights.get(right).copied().unwrap_or(0);
        while let Some(&top) = rising.last() {
            if heights[top] < h {
                break;
            }
            rising.pop();
            let left = rising.last().map_or(0, |&l| l + 1);
            best = best.max(u64::from(heights[top]) * (right - left) as u64);
        }
        rising.push(right);
    }
    best
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
