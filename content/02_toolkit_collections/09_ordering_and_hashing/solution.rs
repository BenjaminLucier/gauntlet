// Ordering and Hashing — reference solution.
// Hand-written Eq/Ord/Hash all go through one normalized view of the value
// (lower-cased bytes, trimmed parts, total_cmp), which keeps them consistent.

use std::cmp::{Ordering, Reverse};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::num::ParseIntError;
use std::str::FromStr;

// ── A case-insensitive string ───────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CaseInsensitive(pub String);

impl CaseInsensitive {
    fn folded(&self) -> impl Iterator<Item = u8> + '_ {
        self.0.bytes().map(|b| b.to_ascii_lowercase())
    }
}

impl PartialEq for CaseInsensitive {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl Eq for CaseInsensitive {}

impl Hash for CaseInsensitive {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for b in self.folded() {
            state.write_u8(b);
        }
        // A terminator, like `str`'s own Hash, so ("ab", "c") and ("a", "bc")
        // hash differently inside a tuple.
        state.write_u8(0xff);
    }
}

impl Ord for CaseInsensitive {
    fn cmp(&self, other: &Self) -> Ordering {
        self.folded().cmp(other.folded())
    }
}

impl PartialOrd for CaseInsensitive {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ── Versions ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Version {
    pub parts: Vec<u32>,
}

impl FromStr for Version {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('.').map(str::parse).collect::<Result<_, _>>()?;
        Ok(Version { parts })
    }
}

impl Version {
    pub fn trimmed(&self) -> &[u32] {
        let end = self.parts.iter().rposition(|&p| p != 0).map_or(0, |i| i + 1);
        &self.parts[..end]
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.trimmed() == other.trimmed()
    }
}

impl Eq for Version {}

impl Hash for Version {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.trimmed().hash(state);
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.trimmed().cmp(other.trimmed())
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn newest<'a>(a: &'a Version, b: &'a Version) -> &'a Version {
    a.max(b)
}

pub fn strictly_ascending(vs: &[Version]) -> bool {
    vs.windows(2).all(|w| w[0].cmp(&w[1]).is_lt())
}

// ── Several keys ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub time: u32,
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score
            .cmp(&other.score)
            .reverse()
            .then(self.time.cmp(&other.time))
            .then_with(|| self.name.cmp(&other.name))
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn by_len_then_alpha(a: &str, b: &str) -> Ordering {
    a.len().cmp(&b.len()).then_with(|| a.cmp(b))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

pub fn most_urgent(ps: &[Priority]) -> Option<Priority> {
    ps.iter().copied().max()
}

pub fn ranked_by_score(players: &[Player]) -> bool {
    players.is_sorted_by_key(|p| Reverse(p.score))
}

// ── Reverse and the *_by_key family ─────────────────────────────────────────

pub fn sort_longest_first(words: &mut [&str]) {
    words.sort_by_key(|w| Reverse(w.len()));
}

pub fn longest<'a>(words: &[&'a str]) -> Option<&'a str> {
    // min_by_key keeps the first minimum; Reverse turns "longest" into a minimum.
    words.iter().copied().min_by_key(|w| Reverse(w.len()))
}

pub fn closer_to_zero(a: i32, b: i32) -> i32 {
    // The free function returns its first argument on a tie.
    std::cmp::min_by_key(a, b, |x| x.unsigned_abs())
}

// ── Floats ──────────────────────────────────────────────────────────────────

pub fn max_f64(xs: &[f64]) -> Option<f64> {
    xs.iter().copied().max_by(f64::total_cmp)
}

pub fn sort_floats(xs: &mut [f64]) {
    xs.sort_by(f64::total_cmp);
}

#[derive(Debug, Clone, Copy)]
pub struct Score(pub f64);

impl PartialEq for Score {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for Score {}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ── Hashing in practice ─────────────────────────────────────────────────────

pub fn distinct_names(names: &[&str]) -> usize {
    names
        .iter()
        .map(|n| CaseInsensitive(n.to_string()))
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
