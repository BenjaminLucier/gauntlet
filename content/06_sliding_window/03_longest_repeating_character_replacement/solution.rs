// Longest Repeating Character Replacement — reference solution.
// Sliding window with letter counts; shrink while len - max_count > k.
// `max_count` never needs to decrease: only a larger one can improve the
// answer. Time O(n), space O(1).

pub fn character_replacement(s: &str, k: usize) -> usize {
    let bytes = s.as_bytes();
    let idx = |b: u8| usize::from(b - b'A');
    let mut count = [0usize; 26];
    let mut max_count = 0;
    let mut left = 0;
    let mut best = 0;
    for (right, &b) in bytes.iter().enumerate() {
        count[idx(b)] += 1;
        max_count = max_count.max(count[idx(b)]);
        // max_count <= window length, so this subtraction can't underflow.
        while right + 1 - left - max_count > k {
            count[idx(bytes[left])] -= 1;
            left += 1;
        }
        best = best.max(right + 1 - left);
    }
    best
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
