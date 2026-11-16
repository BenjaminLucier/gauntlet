// Trapping Rain Water — reference solution.
// Two pointers with running maxima; the side with the smaller max is bounded
// by it, so settle that side and move inward. Time O(n), space O(1).

pub fn trap(heights: &[u32]) -> u64 {
    // `r` is exclusive: the right bar is heights[r - 1].
    let (mut l, mut r) = (0, heights.len());
    let (mut left_max, mut right_max) = (0, 0);
    let mut water = 0;
    while l < r {
        left_max = left_max.max(heights[l]);
        right_max = right_max.max(heights[r - 1]);
        // Each max includes the bar it's subtracted from, so no underflow.
        if left_max < right_max {
            water += u64::from(left_max - heights[l]);
            l += 1;
        } else {
            water += u64::from(right_max - heights[r - 1]);
            r -= 1;
        }
    }
    water
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
