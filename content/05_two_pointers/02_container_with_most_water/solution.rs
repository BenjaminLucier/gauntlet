// Container With Most Water — reference solution.
// Two pointers from the ends; record the area, then move the shorter side
// inward (the taller side can't do better with less width). Areas are
// computed in u64. Time O(n), space O(1).

pub fn max_area(heights: &[u32]) -> u64 {
    let (mut l, mut r) = (0, heights.len());
    let mut best = 0;
    // `r` is exclusive: the right line is heights[r - 1].
    while l + 1 < r {
        let (hl, hr) = (heights[l], heights[r - 1]);
        let width = (r - 1 - l) as u64;
        best = best.max(width * u64::from(hl.min(hr)));
        if hl < hr {
            l += 1;
        } else {
            r -= 1;
        }
    }
    best
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
