// Word Break — reference solution.
// ok[i] = the first i bytes of s can be segmented; ok[i] holds if some piece
// s[j..i] no longer than the longest word is in the word set and ok[j].
// `str::get` skips split points inside multi-byte characters.
// Time O(n * k^2) for words of at most k bytes, space O(n + dictionary).

use std::collections::HashSet;

pub fn word_break(s: &str, words: &[&str]) -> bool {
    let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
    let dictionary: HashSet<&str> = words.iter().copied().collect();
    let n = s.len();
    let mut ok = vec![false; n + 1];
    ok[0] = true;
    for i in 1..=n {
        ok[i] = (i.saturating_sub(max_len)..i)
            .any(|j| ok[j] && s.get(j..i).is_some_and(|piece| dictionary.contains(piece)));
    }
    ok[n]
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
