// Regular Expression Matching — reference solution.
// Parse the pattern into (atom, starred) tokens, rejecting a `*` with nothing to repeat.
// Then dp[i][j] = s[i..] matches tokens[j..], filled from the ends with two rolling rows:
// a starred token matches nothing (dp[i][j+1]) or eats one char and stays (dp[i+1][j]).
// Time O(|s| * |p|), space O(|p|).

use std::fmt;
use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternError {
    /// A `*` with nothing to repeat; `at` is its byte offset in the pattern.
    NothingToRepeat { at: usize },
}

impl fmt::Display for PatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatternError::NothingToRepeat { at } => {
                write!(f, "`*` at byte {at} has nothing to repeat")
            }
        }
    }
}

impl std::error::Error for PatternError {}

#[derive(Debug, Clone, Copy)]
enum Atom {
    Any,
    Char(char),
}

impl Atom {
    fn matches(self, c: char) -> bool {
        match self {
            Atom::Any => true,
            Atom::Char(want) => want == c,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Token {
    atom: Atom,
    starred: bool,
}

fn parse(p: &str) -> Result<Vec<Token>, PatternError> {
    let mut tokens: Vec<Token> = Vec::new();
    for (at, ch) in p.char_indices() {
        match ch {
            '*' => match tokens.last_mut() {
                Some(prev) if !prev.starred => prev.starred = true,
                _ => return Err(PatternError::NothingToRepeat { at }),
            },
            '.' => tokens.push(Token { atom: Atom::Any, starred: false }),
            c => tokens.push(Token { atom: Atom::Char(c), starred: false }),
        }
    }
    Ok(tokens)
}

pub fn is_match(s: &str, p: &str) -> Result<bool, PatternError> {
    let tokens = parse(p)?;
    let s: Vec<char> = s.chars().collect();
    let n = tokens.len();
    // `next` is row i + 1, `cur` is row i; row s.len() + 1 doesn't exist (all false).
    let mut next = vec![false; n + 1];
    let mut cur = vec![false; n + 1];
    for i in (0..=s.len()).rev() {
        cur[n] = i == s.len();
        for (j, token) in tokens.iter().enumerate().rev() {
            let first = s.get(i).is_some_and(|&c| token.atom.matches(c));
            cur[j] = if token.starred {
                cur[j + 1] || (first && next[j])
            } else {
                first && next[j + 1]
            };
        }
        mem::swap(&mut cur, &mut next);
    }
    Ok(next[0])
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
