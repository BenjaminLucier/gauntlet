// Longest Substring Without Repeating Characters — reference solution.
// Sliding window over chars with the last position of each char; on a repeat
// inside the window, jump `left` past it (never backwards).
// Time O(n) expected, space O(distinct chars).

use std::collections::HashMap;

pub fn length_of_longest_substring(s: &str) -> usize {
    let mut last_seen: HashMap<char, usize> = HashMap::new();
    let mut left = 0;
    let mut best = 0;
    for (right, c) in s.chars().enumerate() {
        if let Some(prev) = last_seen.insert(c, right) {
            left = left.max(prev + 1);
        }
        best = best.max(right + 1 - left);
    }
    best
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
