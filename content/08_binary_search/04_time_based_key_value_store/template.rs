// Time Based Key-Value Store — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
#![allow(unused_imports, unused_variables, dead_code)]

use std::collections::HashMap;
use std::marker::PhantomData;

pub struct TimeMap<V> {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: PhantomData<V>,
}

impl<V> TimeMap<V> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn set(&mut self, key: &str, value: V, timestamp: u32) {
        todo!()
    }

    pub fn get(&self, key: &str, timestamp: u32) -> Option<&V> {
        todo!()
    }
}

impl<V> Default for TimeMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
