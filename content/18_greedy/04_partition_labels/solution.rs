// Partition Labels — reference solution.
// Record each byte's last index; extend the current piece's end to the last
// index of every byte seen, cutting a borrowed slice when i reaches it.
// Time O(n), space O(1) besides the output.

pub fn partition_labels(s: &str) -> Vec<&str> {
    let bytes = s.as_bytes();
    let mut last = [0usize; 256];
    for (i, &b) in bytes.iter().enumerate() {
        last[usize::from(b)] = i;
    }
    let mut pieces = Vec::new();
    let (mut start, mut end) = (0, 0);
    for (i, &b) in bytes.iter().enumerate() {
        end = end.max(last[usize::from(b)]);
        if i == end {
            pieces.push(&s[start..=end]);
            start = i + 1;
        }
    }
    pieces
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
