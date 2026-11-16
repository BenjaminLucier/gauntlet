// Hash Map (Separate Chaining) — reference solution.
// A power-of-two number of buckets, each a small Vec of (key, value) pairs.
// When len would exceed 3/4 of the bucket count, the bucket count doubles and
// every pair is rehashed, so chains stay O(1) long: every operation is O(1)
// expected (insert amortized). Space O(peak n): buckets never shrink.

use std::borrow::Borrow;
use std::hash::{BuildHasher, Hash, RandomState};
use std::iter::Flatten;
use std::mem;
use std::slice;

const INITIAL_BUCKETS: usize = 8;

pub struct ChainedMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    len: usize,
    hasher: RandomState,
}

fn empty_buckets<K, V>(n: usize) -> Vec<Vec<(K, V)>> {
    (0..n).map(|_| Vec::new()).collect()
}

impl<K: Hash + Eq, V> ChainedMap<K, V> {
    pub fn new() -> Self {
        ChainedMap {
            buckets: empty_buckets(INITIAL_BUCKETS),
            len: 0,
            hasher: RandomState::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// The bucket count is a power of two, so masking is a cheap modulo.
    fn bucket_of<Q: Hash + ?Sized>(&self, key: &Q) -> usize {
        self.hasher.hash_one(key) as usize & (self.buckets.len() - 1)
    }

    /// Make room for one more pair: double the buckets and rehash everything
    /// once the load factor would pass 3/4.
    fn grow_if_needed(&mut self) {
        if (self.len + 1) * 4 <= self.buckets.len() * 3 {
            return;
        }
        let doubled = empty_buckets(self.buckets.len() * 2);
        let old = mem::replace(&mut self.buckets, doubled);
        for (key, value) in old.into_iter().flatten() {
            let i = self.bucket_of(&key);
            self.buckets[i].push((key, value));
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let i = self.bucket_of(&key);
        if let Some((_, old)) = self.buckets[i].iter_mut().find(|(k, _)| *k == key) {
            return Some(mem::replace(old, value));
        }
        self.grow_if_needed();
        let i = self.bucket_of(&key);
        self.buckets[i].push((key, value));
        self.len += 1;
        None
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let i = self.bucket_of(key);
        self.buckets[i]
            .iter()
            .find(|(k, _)| k.borrow() == key)
            .map(|(_, v)| v)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let i = self.bucket_of(key);
        self.buckets[i]
            .iter_mut()
            .find(|(k, _)| k.borrow() == key)
            .map(|(_, v)| v)
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get(key).is_some()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let i = self.bucket_of(key);
        let bucket = &mut self.buckets[i];
        let pos = bucket.iter().position(|(k, _)| k.borrow() == key)?;
        self.len -= 1;
        // Order within a chain doesn't matter, so swap_remove is O(1).
        Some(bucket.swap_remove(pos).1)
    }

    pub fn get_or_insert_with<F: FnOnce() -> V>(&mut self, key: K, default: F) -> &mut V {
        // Find the position first and only borrow the value at the very end:
        // returning a borrow from inside the search would keep `self` borrowed
        // across the insert branch.
        let mut i = self.bucket_of(&key);
        let pos = match self.buckets[i].iter().position(|(k, _)| *k == key) {
            Some(pos) => pos,
            None => {
                self.grow_if_needed();
                i = self.bucket_of(&key);
                self.buckets[i].push((key, default()));
                self.len += 1;
                self.buckets[i].len() - 1
            }
        };
        &mut self.buckets[i][pos].1
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            inner: self.buckets.iter().flatten(),
        }
    }
}

/// Borrowing iterator over the pairs of a `ChainedMap`, in no particular order.
pub struct Iter<'a, K, V> {
    inner: Flatten<slice::Iter<'a, Vec<(K, V)>>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        self.inner.next().map(|(k, v)| (k, v))
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
        self.iter()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
