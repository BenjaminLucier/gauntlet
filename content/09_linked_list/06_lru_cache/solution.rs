// LRU Cache — reference solution.
// A HashMap from key to slot index, plus a doubly linked list threaded through
// a Vec of slots by index (front = most recent); evictions reuse the oldest
// slot. Time O(1) average per operation, space O(capacity).

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

struct Entry<K, V> {
    key: K,
    value: V,
    prev: Option<usize>,
    next: Option<usize>,
}

pub struct LruCache<K, V> {
    capacity: usize,
    index: HashMap<K, usize>,
    slots: Vec<Entry<K, V>>,
    most_recent: Option<usize>,
    least_recent: Option<usize>,
}

impl<K: Hash + Eq + Clone, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        LruCache {
            capacity,
            index: HashMap::with_capacity(capacity),
            slots: Vec::with_capacity(capacity),
            most_recent: None,
            least_recent: None,
        }
    }

    pub fn get<Q>(&mut self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let i = *self.index.get(key)?;
        self.touch(i);
        Some(&self.slots[i].value)
    }

    pub fn put(&mut self, key: K, value: V) {
        if let Some(&i) = self.index.get(&key) {
            self.slots[i].value = value;
            self.touch(i);
            return;
        }
        if self.capacity == 0 {
            return;
        }
        let entry = Entry { key: key.clone(), value, prev: None, next: None };
        let i = if self.slots.len() < self.capacity {
            self.slots.push(entry);
            self.slots.len() - 1
        } else {
            let lru = self.least_recent.expect("a full, non-zero-capacity cache has an oldest entry");
            self.unlink(lru);
            let evicted = std::mem::replace(&mut self.slots[lru], entry);
            self.index.remove(&evicted.key);
            lru
        };
        self.push_front(i);
        self.index.insert(key, i);
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    /// Mark slot `i` as most recently used.
    fn touch(&mut self, i: usize) {
        if self.most_recent != Some(i) {
            self.unlink(i);
            self.push_front(i);
        }
    }

    fn unlink(&mut self, i: usize) {
        let (prev, next) = (self.slots[i].prev, self.slots[i].next);
        match prev {
            Some(p) => self.slots[p].next = next,
            None => self.most_recent = next,
        }
        match next {
            Some(n) => self.slots[n].prev = prev,
            None => self.least_recent = prev,
        }
    }

    fn push_front(&mut self, i: usize) {
        self.slots[i].prev = None;
        self.slots[i].next = self.most_recent;
        match self.most_recent {
            Some(old) => self.slots[old].prev = Some(i),
            None => self.least_recent = Some(i),
        }
        self.most_recent = Some(i);
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
