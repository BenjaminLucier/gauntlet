// Hash Map (Separate Chaining) — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no HashMap, HashSet, BTreeMap or BTreeSet — buckets are a Vec<Vec<(K, V)>>. No unsafe.
#![allow(unused_imports, unused_variables, dead_code)]

use std::borrow::Borrow;
use std::hash::{BuildHasher, Hash, RandomState};
use std::marker::PhantomData;
use std::mem;

pub struct ChainedMap<K, V> {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: PhantomData<(K, V)>,
}

impl<K: Hash + Eq, V> ChainedMap<K, V> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Insert or overwrite. Returns the previous value for `key`, if any.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        todo!()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        todo!()
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        todo!()
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        todo!()
    }

    /// Remove `key` and return its value, if it was present.
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        todo!()
    }

    /// The value for `key`, inserting `default()` first if it's missing.
    /// `default` is only called when the key is absent.
    pub fn get_or_insert_with<F: FnOnce() -> V>(&mut self, key: K, default: F) -> &mut V {
        todo!()
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        todo!()
    }
}

/// Borrowing iterator over the pairs of a `ChainedMap`, in no particular order.
pub struct Iter<'a, K, V> {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: PhantomData<(&'a K, &'a V)>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        todo!()
    }
}

impl<K: Hash + Eq, V> Default for ChainedMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, K: Hash + Eq, V> IntoIterator for &'a ChainedMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Iter<'a, K, V> {
        todo!()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
