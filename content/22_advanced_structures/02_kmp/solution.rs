// Knuth–Morris–Pratt — reference solution.
// The prefix function is the pattern matched against itself; the scan reuses
// it to fall back after a mismatch without moving backwards in the text.
// O(m) to build the table, O(n) to scan; one private iterator serves the find_all
// and find_first variants.

pub fn prefix_function<T: Eq>(pattern: &[T]) -> Vec<usize> {
    let mut pi = vec![0; pattern.len()];
    let mut k = 0;
    for i in 1..pattern.len() {
        while k > 0 && pattern[i] != pattern[k] {
            k = pi[k - 1];
        }
        if pattern[i] == pattern[k] {
            k += 1;
        }
        pi[i] = k;
    }
    pi
}

/// Lazily yields the start of every match of a non-empty pattern.
struct Matches<'a, T> {
    text: &'a [T],
    pattern: &'a [T],
    pi: Vec<usize>,
    /// Next text index to read.
    pos: usize,
    /// Length of the pattern prefix currently matched.
    k: usize,
}

impl<'a, T: Eq> Matches<'a, T> {
    fn new(text: &'a [T], pattern: &'a [T]) -> Self {
        assert!(
            !pattern.is_empty(),
            "the empty pattern is handled by the callers"
        );
        Self {
            text,
            pattern,
            pi: prefix_function(pattern),
            pos: 0,
            k: 0,
        }
    }
}

impl<T: Eq> Iterator for Matches<'_, T> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let m = self.pattern.len();
        while let Some(c) = self.text.get(self.pos) {
            self.pos += 1;
            while self.k > 0 && self.pattern[self.k] != *c {
                self.k = self.pi[self.k - 1];
            }
            if self.pattern[self.k] == *c {
                self.k += 1;
            }
            if self.k == m {
                // Keep the longest border so overlapping matches are found.
                self.k = self.pi[m - 1];
                return Some(self.pos - m);
            }
        }
        None
    }
}

pub fn find_all_in<T: Eq>(text: &[T], pattern: &[T]) -> Vec<usize> {
    if pattern.is_empty() {
        return (0..=text.len()).collect();
    }
    Matches::new(text, pattern).collect()
}

pub fn find_all(text: &str, pattern: &str) -> Vec<usize> {
    find_all_in(text.as_bytes(), pattern.as_bytes())
}

pub fn find_first_in<T: Eq>(text: &[T], pattern: &[T]) -> Option<usize> {
    if pattern.is_empty() {
        return Some(0);
    }
    Matches::new(text, pattern).next()
}

pub fn find_first(text: &str, pattern: &str) -> Option<usize> {
    find_first_in(text.as_bytes(), pattern.as_bytes())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
