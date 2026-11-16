// Binary Search Tree — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no BTreeMap/BTreeSet. No recursion on the tree's height (drop included). No unsafe.
#![allow(unused_imports, unused_variables, dead_code)]

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::marker::PhantomData;
use std::mem;

pub struct Bst<K, V> {
    root: Link<K, V>,
    len: usize,
}

struct Node<K, V> {
    key: K,
    value: V,
    left: Link<K, V>,
    right: Link<K, V>,
}

type Link<K, V> = Option<Box<Node<K, V>>>;

impl<K: Ord, V> Bst<K, V> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Insert or replace. Returns the previous value for `key`, if any.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        todo!()
    }

    /// Like `BTreeMap::get`: `key` may be any borrowed form of `K` (a `&str`
    /// for `String` keys), as long as it orders the same way.
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        todo!()
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        todo!()
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        todo!()
    }

    /// Remove `key` and return its value. Handles 0, 1 or 2 children.
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        todo!()
    }

    /// The entry with the smallest key.
    pub fn min(&self) -> Option<(&K, &V)> {
        todo!()
    }

    /// The entry with the largest key.
    pub fn max(&self) -> Option<(&K, &V)> {
        todo!()
    }

    /// The entry with the largest key `<= key`.
    pub fn floor<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        todo!()
    }

    /// The entry with the smallest key `>= key`.
    pub fn ceiling<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        todo!()
    }

    /// In-order: keys ascending.
    pub fn iter(&self) -> Iter<'_, K, V> {
        todo!()
    }
}

impl<K: Ord, V> Default for Bst<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for Bst<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        todo!()
    }
}

impl<K: Ord, V> Extend<(K, V)> for Bst<K, V> {
    /// Like repeated `insert`: a later value for a key replaces an earlier one.
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        todo!()
    }
}

impl<K, V> Drop for Bst<K, V> {
    fn drop(&mut self) {
        // TODO: take the tree apart without recursion. Until you do, the
        // fields are dropped the default way after this runs — recursively,
        // one stack frame per level, which a deep tree will overflow.
    }
}

/// In-order iterator over `(&K, &V)`, smallest key first.
pub struct Iter<'a, K, V> {
    // TODO: your fields here (then remove this placeholder)
    _placeholder: PhantomData<&'a (K, V)>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a, K: Ord, V> IntoIterator for &'a Bst<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Iter<'a, K, V> {
        todo!()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
