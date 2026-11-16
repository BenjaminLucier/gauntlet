// Design Add and Search Words Data Structure — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

pub struct WordDictionary {
    // TODO: your fields here
}

impl WordDictionary {
    pub fn new() -> Self {
        todo!()
    }

    pub fn insert(&mut self, word: &str) {
        todo!()
    }

    /// Does some stored word match `pattern`, where `.` matches any one letter?
    pub fn search(&self, pattern: &str) -> bool {
        todo!()
    }
}

impl Default for WordDictionary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
