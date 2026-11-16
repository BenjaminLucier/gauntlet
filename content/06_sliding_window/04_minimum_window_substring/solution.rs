// Minimum Window Substring — reference solution.
// Sliding window over bytes; `need` holds what the window still lacks and
// `missing` how many characters of t are uncovered. Expand right; while
// nothing is missing, record and shrink left. Time O(|s| + |t|), space O(1).

pub fn min_window<'a>(s: &'a str, t: &str) -> Option<&'a str> {
    let bytes = s.as_bytes();
    // need[c] > 0: the window lacks that many c's; < 0: it has a surplus.
    let mut need = [0i64; 128];
    for b in t.bytes() {
        need[usize::from(b)] += 1;
    }
    let mut missing = t.len();
    if missing == 0 {
        return Some(&s[..0]);
    }
    // Best window so far as a half-open byte range.
    let mut best: Option<(usize, usize)> = None;
    let mut left = 0;
    for (right, &b) in bytes.iter().enumerate() {
        let c = usize::from(b);
        if need[c] > 0 {
            missing -= 1;
        }
        need[c] -= 1;
        while missing == 0 {
            let end = right + 1;
            if best.is_none_or(|(l, r)| end - left < r - l) {
                best = Some((left, end));
            }
            let d = usize::from(bytes[left]);
            need[d] += 1;
            if need[d] > 0 {
                missing += 1;
            }
            left += 1;
        }
    }
    best.map(|(l, r)| &s[l..r])
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
