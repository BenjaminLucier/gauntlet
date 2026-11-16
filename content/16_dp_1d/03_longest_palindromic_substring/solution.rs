// Longest Palindromic Substring — reference solution.
// Expand around each of the 2n-1 centers in char positions, keep the widest
// range, then map it to byte offsets and return that slice of the input.
// Time O(n^2), space O(n) for the chars and their byte offsets.

pub fn longest_palindrome(s: &str) -> &str {
    let (starts, chars): (Vec<usize>, Vec<char>) = s.char_indices().unzip();
    let n = chars.len();

    // Grows the palindrome chars[lo..hi] while the chars just outside match.
    let expand = |mut lo: usize, mut hi: usize| {
        while lo > 0 && hi < n && chars[lo - 1] == chars[hi] {
            lo -= 1;
            hi += 1;
        }
        (lo, hi)
    };

    // Odd centers start as the single char i; even ones as the gap before i.
    let (lo, hi) = (0..n)
        .flat_map(|i| [expand(i, i + 1), expand(i, i)])
        .max_by_key(|&(lo, hi)| hi - lo)
        .unwrap_or((0, 0));

    let byte_offset = |i: usize| starts.get(i).copied().unwrap_or(s.len());
    &s[byte_offset(lo)..byte_offset(hi)]
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
