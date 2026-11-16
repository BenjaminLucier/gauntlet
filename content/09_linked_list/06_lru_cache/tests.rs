// Tests for LRU Cache — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::cell::Cell;
use std::hash::Hasher;

#[derive(Clone, Copy, Debug)]
enum Op {
    Get(i32, Option<i32>), // key, expected result
    Put(i32, i32),         // key, value
}

use Op::{Get, Put};

fn run(capacity: usize, ops: &[Op]) {
    let mut cache = LruCache::new(capacity);
    for (step, &op) in ops.iter().enumerate() {
        match op {
            Put(k, v) => cache.put(k, v),
            Get(k, expected) => {
                let got = cache.get(&k).copied();
                assert_eq!(
                    got,
                    expected,
                    "capacity {capacity}, step {step}: get({k}) — ops so far: {:?}",
                    &ops[..=step]
                );
            }
        }
    }
}

/// Simple O(capacity) model: front of the Vec = most recently used.
struct BruteLru {
    cap: usize,
    items: Vec<(i32, i32)>,
}

impl BruteLru {
    fn get(&mut self, key: i32) -> Option<i32> {
        let p = self.items.iter().position(|&(k, _)| k == key)?;
        let e = self.items.remove(p);
        self.items.insert(0, e);
        Some(e.1)
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(p) = self.items.iter().position(|&(k, _)| k == key) {
            self.items.remove(p);
        } else if self.cap == 0 {
            return;
        } else if self.items.len() == self.cap {
            self.items.pop();
        }
        self.items.insert(0, (key, value));
    }
}

/// Tiny deterministic PRNG (xorshift64) so random tests are reproducible.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        lo + (self.next() % (hi - lo + 1) as u64) as i32
    }
}

fn randomized(seed: u64, capacity: usize, key_max: i32, ops: usize) {
    let mut rng = Rng(seed);
    let mut cache = LruCache::new(capacity);
    let mut model = BruteLru { cap: capacity, items: Vec::new() };
    let mut log: Vec<String> = Vec::new();
    for step in 0..ops {
        let key = rng.range(0, key_max);
        if rng.range(0, 1) == 0 {
            let value = rng.range(0, 100_000);
            cache.put(key, value);
            model.put(key, value);
            log.push(format!("put({key}, {value})"));
        } else {
            let got = cache.get(&key).copied();
            let expected = model.get(key);
            log.push(format!("get({key})"));
            if got != expected {
                let start = log.len().saturating_sub(12);
                panic!(
                    "capacity {capacity}, step {step}: get({key}) returned {got:?}, expected {expected:?}.\n\
                     last ops: {:?}\nmodel (most recent first): {:?}",
                    &log[start..],
                    model.items
                );
            }
        }
        assert_eq!(cache.len(), model.items.len(), "len() after {:?}", log.last());
    }
}

#[test]
fn example_1() {
    run(
        2,
        &[
            Put(1, 1),
            Put(2, 2),
            Get(1, Some(1)),
            Put(3, 3),
            Get(2, None),
            Put(4, 4),
            Get(1, None),
            Get(3, Some(3)),
            Get(4, Some(4)),
        ],
    );
}

#[test]
fn example_2_string_keys_borrowed_lookup() {
    let mut names: LruCache<String, Vec<u8>> = LruCache::new(1);
    names.put("rust".to_string(), vec![1, 2]);
    assert_eq!(names.get("rust"), Some(&vec![1, 2]));
    names.put("go".to_string(), vec![3]);
    assert_eq!(names.get("rust"), None);
    assert_eq!(names.get(&"go".to_string()), Some(&vec![3]));
}

#[test]
fn get_on_empty_cache() {
    run(3, &[Get(0, None), Get(5, None)]);
    let cache: LruCache<i32, i32> = LruCache::new(3);
    assert!(cache.is_empty());
    assert_eq!(cache.len(), 0);
}

#[test]
fn capacity_one() {
    run(
        1,
        &[
            Put(1, 10),
            Get(1, Some(10)),
            Put(2, 20),
            Get(1, None),
            Get(2, Some(20)),
            Put(2, 21),
            Get(2, Some(21)),
        ],
    );
}

#[test]
fn capacity_zero_stores_nothing() {
    let mut cache = LruCache::new(0);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.get(&2), None);
    assert!(cache.is_empty());
}

#[test]
fn get_refreshes_recency() {
    // Without updating recency on get, key 1 would be evicted instead of key 2.
    run(
        2,
        &[Put(1, 1), Put(2, 2), Get(1, Some(1)), Put(3, 3), Get(1, Some(1)), Get(2, None), Get(3, Some(3))],
    );
}

#[test]
fn put_existing_updates_value_and_recency() {
    run(
        2,
        &[Put(1, 1), Put(2, 2), Put(1, 10), Put(3, 3), Get(1, Some(10)), Get(2, None), Get(3, Some(3))],
    );
}

#[test]
fn overwrite_when_full_does_not_evict() {
    // Updating an existing key must not count as a new insertion.
    run(2, &[Put(1, 1), Put(2, 2), Put(2, 5), Get(1, Some(1)), Get(2, Some(5))]);
    let mut cache = LruCache::new(2);
    cache.put('a', 1);
    cache.put('b', 2);
    cache.put('b', 3);
    assert_eq!(cache.len(), 2);
}

#[test]
fn missed_get_does_not_change_order() {
    run(
        2,
        &[Put(1, 1), Put(2, 2), Get(9, None), Put(3, 3), Get(1, None), Get(2, Some(2)), Get(3, Some(3))],
    );
}

#[test]
fn zero_key_and_zero_value() {
    // Zero is a real key and a real value, not "missing".
    run(
        2,
        &[Put(0, 0), Get(0, Some(0)), Put(1, 0), Get(1, Some(0)), Put(2, 7), Get(0, None), Get(1, Some(0))],
    );
}

#[test]
fn negative_keys_and_values() {
    run(2, &[Put(-1, -10), Put(i32::MIN, i32::MAX), Get(-1, Some(-10)), Get(i32::MIN, Some(i32::MAX))]);
}

#[test]
fn eviction_order_is_insertion_order_without_gets() {
    run(
        3,
        &[
            Put(1, 1),
            Put(2, 2),
            Put(3, 3),
            Put(4, 4),
            Get(1, None),
            Put(5, 5),
            Get(2, None),
            Get(3, Some(3)),
            Get(4, Some(4)),
            Get(5, Some(5)),
        ],
    );
}

#[test]
fn evicted_key_can_return() {
    run(
        2,
        &[Put(1, 1), Put(2, 2), Put(3, 3), Get(1, None), Put(1, 100), Get(1, Some(100)), Get(2, None), Get(3, Some(3))],
    );
}

#[test]
fn len_tracks_entries() {
    let mut cache = LruCache::new(3);
    assert!(cache.is_empty());
    for (k, expected_len) in [(1, 1), (2, 2), (1, 2), (3, 3), (4, 3)] {
        cache.put(k, k);
        assert_eq!(cache.len(), expected_len, "after put({k})");
    }
    assert!(!cache.is_empty());
}

#[test]
fn values_need_not_be_clone() {
    // Values are stored once and lent out; they are never cloned.
    struct Blob(Vec<u8>);
    let mut cache = LruCache::new(2);
    cache.put("a", Blob(vec![1]));
    cache.put("b", Blob(vec![2, 2]));
    assert_eq!(cache.get("a").map(|b| b.0.len()), Some(1));
    cache.put("c", Blob(vec![3, 3, 3]));
    assert!(cache.get("b").is_none());
    assert_eq!(cache.get("c").map(|b| b.0.len()), Some(3));
}

#[test]
fn get_lends_the_stored_value() {
    let mut cache = LruCache::new(1);
    cache.put(7u64, String::from("seven"));
    let first: *const String = cache.get(&7).expect("present");
    let second: *const String = cache.get(&7).expect("present");
    assert_eq!(first, second, "get should return a reference into the cache, not a copy");
}

#[test]
fn random_small_capacities() {
    for cap in 0..=6 {
        randomized(0x1000 + cap as u64 * 7919, cap, 10, 3000);
    }
}

#[test]
fn random_long_sequence() {
    randomized(0xFEED_FACE_CAFE_BEEF, 50, 120, 50_000);
}

#[test]
fn large_max_calls_full_cache() {
    // 2 * 10^5 operations on a full cache of 1000, then check the eviction order.
    let cap = 1000;
    let mut cache = LruCache::new(cap);
    for k in 0..cap {
        cache.put(k, k);
    }
    // 7 is coprime with 1000, so each block of 1000 gets touches every key once,
    // in the order 0, 7, 14, ... (mod 1000).
    for i in 0..195_000 {
        let k = (i * 7) % cap;
        assert_eq!(cache.get(&k), Some(&k), "get({k}) at op {i}");
    }
    // The least recently used key is now 0 (touched first in the last round).
    cache.put(5000, 1);
    assert_eq!(cache.get(&0), None, "key 0 was least recently used and should be evicted");
    assert_eq!(cache.get(&7), Some(&7));
    assert_eq!(cache.get(&5000), Some(&1));
    cache.put(5001, 2);
    assert_eq!(cache.get(&14), None, "key 14 should be evicted next");
}

thread_local! {
    static EQ_CALLS: Cell<usize> = const { Cell::new(0) };
}

/// A key that counts how often it is compared for equality (on this thread).
#[derive(Clone, Debug)]
struct Counted(u32);

impl PartialEq for Counted {
    fn eq(&self, other: &Self) -> bool {
        EQ_CALLS.with(|c| c.set(c.get() + 1));
        self.0 == other.0
    }
}

impl Eq for Counted {}

impl Hash for Counted {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[test]
fn operations_do_not_scan_the_keys() {
    // A hash lookup compares a key about once. Finding it in a list of keys
    // (a Vec or VecDeque in recency order) compares it with about half the
    // cache on every call: ~500 per call here, 10^7 in all.
    let cap = 1000;
    let ops = 20_000;
    let mut cache = LruCache::new(cap);
    for k in 0..cap as u32 {
        cache.put(Counted(k), k);
    }
    let mut rng = Rng(0x00C0_FFEE_1234_ABCD);
    let before = EQ_CALLS.with(Cell::get);
    for _ in 0..ops {
        // Every key is present: half the calls read it, half overwrite it.
        let k = rng.range(0, cap as i32 - 1) as u32;
        if rng.range(0, 1) == 0 {
            assert_eq!(cache.get(&Counted(k)), Some(&k), "get({k})");
        } else {
            cache.put(Counted(k), k);
        }
    }
    let calls = EQ_CALLS.with(Cell::get) - before;
    assert!(
        calls <= 10 * ops,
        "{calls} key comparisons for {ops} calls on a full cache of {cap}: \
         find entries through the hash map, not by scanning the keys"
    );
    assert_eq!(cache.len(), cap);
}

#[test]
fn every_put_evicts_on_a_full_cache() {
    // A full cache of 3000 given 40_000 new keys, so every put after the
    // first 3000 evicts. Searching all entries for the oldest (a timestamp per
    // entry, say) costs 3000 steps per eviction, about 10^8 in all; taking the
    // oldest from the end of a recency list is one step.
    let cap = 3000;
    let n = 40_000u32;
    let mut cache = LruCache::new(cap);
    for k in 0..n {
        cache.put(k, k);
    }
    assert_eq!(cache.len(), cap, "n = {n}");
    // Only the last 3000 keys survive.
    let oldest_kept = n - cap as u32;
    for k in 0..oldest_kept {
        assert_eq!(cache.get(&k), None, "key {k} should have been evicted, n = {n}");
    }
    for k in oldest_kept..n {
        assert_eq!(cache.get(&k), Some(&k), "key {k} should be present, n = {n}");
    }
}
