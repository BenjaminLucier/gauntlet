// Tests for Hash Map (Separate Chaining) — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::cell::Cell;
use std::collections::HashMap;
use std::hash::Hasher;
use std::rc::Rc;

/// Tiny deterministic PRNG (xorshift64) so random tests are reproducible.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

fn sorted_pairs<K: Clone + Ord + Hash, V: Clone>(m: &ChainedMap<K, V>) -> Vec<(K, V)> {
    let mut v: Vec<(K, V)> = m.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    v.sort_by(|a, b| a.0.cmp(&b.0));
    v
}

/// Deliberately not `Clone`, `Copy` or `Default`.
#[derive(Debug, PartialEq, Eq, Hash)]
struct Token(u32);

/// A key whose hash ignores its value: every `Collide` lands in the same
/// bucket, so lookups must compare keys, not just hashes.
#[derive(Debug, PartialEq, Eq)]
struct Collide(u32);

impl Hash for Collide {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u8(0);
    }
}

thread_local! {
    static EQ_CALLS: Cell<u64> = const { Cell::new(0) };
    static HASH_CALLS: Cell<u64> = const { Cell::new(0) };
}

/// Key comparisons made so far on this thread (by `Probe`).
fn eq_calls() -> u64 {
    EQ_CALLS.with(Cell::get)
}

/// Keys hashed so far on this thread (by `Probe`).
fn hash_calls() -> u64 {
    HASH_CALLS.with(Cell::get)
}

/// A key that counts every `==` it takes part in, to measure chain lengths,
/// and every time it is hashed, to measure rehashing.
#[derive(Debug)]
struct Probe(u64);

impl Hash for Probe {
    fn hash<H: Hasher>(&self, state: &mut H) {
        HASH_CALLS.with(|c| c.set(c.get() + 1));
        self.0.hash(state);
    }
}

impl PartialEq for Probe {
    fn eq(&self, other: &Self) -> bool {
        EQ_CALLS.with(|c| c.set(c.get() + 1));
        self.0 == other.0
    }
}

impl Eq for Probe {}

/// Counts its drops, to check every value is dropped exactly once.
struct Counted(Rc<Cell<usize>>);

impl Drop for Counted {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[test]
fn readme_example() {
    let mut m = ChainedMap::new();
    assert!(m.is_empty());
    assert_eq!(m.insert("apple", 3), None);
    assert_eq!(m.insert("pear", 5), None);
    assert_eq!(m.insert("apple", 4), Some(3));
    assert_eq!(m.len(), 2);
    assert_eq!(m.get("apple"), Some(&4));
    assert_eq!(m.get("plum"), None);
    *m.get_mut("pear").unwrap() += 1;
    assert_eq!(m.get("pear"), Some(&6));
    assert_eq!(m.remove("apple"), Some(4));
    assert!(!m.contains_key("apple"));

    let mut counts: ChainedMap<String, usize> = ChainedMap::new();
    for word in "to be or not to be".split(' ') {
        *counts.get_or_insert_with(word.to_string(), || 0) += 1;
    }
    assert_eq!(counts.get("be"), Some(&2));
    assert_eq!(counts.len(), 4);
}

#[test]
fn new_and_default_are_empty() {
    let m: ChainedMap<u32, u32> = ChainedMap::new();
    assert_eq!(m.len(), 0);
    assert!(m.is_empty());
    assert_eq!(m.get(&0), None);
    assert!(!m.contains_key(&0));
    assert_eq!(m.iter().next(), None);
    let d: ChainedMap<u32, u32> = ChainedMap::default();
    assert!(d.is_empty());
}

#[test]
fn insert_existing_key_replaces_without_growing_len() {
    let mut m = ChainedMap::new();
    assert_eq!(m.insert(7, "a"), None);
    assert_eq!(m.insert(7, "b"), Some("a"));
    assert_eq!(m.insert(7, "c"), Some("b"));
    assert_eq!(m.len(), 1, "overwriting must not add a second entry");
    assert_eq!(m.iter().count(), 1);
    assert_eq!(m.get(&7), Some(&"c"));
}

#[test]
fn get_mut_changes_the_stored_value() {
    let mut m = ChainedMap::new();
    m.insert(1, vec![1]);
    m.get_mut(&1).unwrap().push(2);
    assert_eq!(m.get(&1), Some(&vec![1, 2]));
    assert_eq!(m.get_mut(&2), None);
}

#[test]
fn remove_returns_the_value_once() {
    let mut m = ChainedMap::new();
    for i in 0..10 {
        m.insert(i, i * 10);
    }
    assert_eq!(m.remove(&4), Some(40));
    assert_eq!(m.remove(&4), None, "second remove of the same key");
    assert_eq!(m.remove(&99), None);
    assert_eq!(m.len(), 9);
    assert_eq!(m.get(&4), None);
    assert_eq!(m.get(&5), Some(&50), "neighbours survive a removal");
}

#[test]
fn remove_everything_then_reuse() {
    let mut m = ChainedMap::new();
    for i in 0..100 {
        m.insert(i, i);
    }
    for i in 0..100 {
        assert_eq!(m.remove(&i), Some(i));
    }
    assert!(m.is_empty());
    assert_eq!(m.iter().next(), None);
    m.insert(5, 50);
    assert_eq!(sorted_pairs(&m), [(5, 50)]);
}

#[test]
fn resize_rehashes_every_key() {
    // Growing the bucket array without moving pairs to their new bucket
    // (hash % new_count) loses most of them.
    let mut m = ChainedMap::new();
    for i in 0..5_000u64 {
        m.insert(i, i + 1);
        assert_eq!(m.get(&i), Some(&(i + 1)), "just inserted {i}");
    }
    for i in 0..5_000u64 {
        assert_eq!(m.get(&i), Some(&(i + 1)), "lost key {i} after resizing");
    }
    assert_eq!(m.len(), 5_000);
}

#[test]
fn keys_with_equal_hashes_are_told_apart() {
    let mut m = ChainedMap::new();
    for i in 0..50 {
        assert_eq!(m.insert(Collide(i), i), None, "Collide({i}) is a new key");
    }
    assert_eq!(m.len(), 50);
    assert_eq!(m.get(&Collide(17)), Some(&17));
    assert_eq!(m.insert(Collide(17), 1700), Some(17));
    assert_eq!(m.remove(&Collide(3)), Some(3));
    assert_eq!(m.get(&Collide(3)), None);
    assert_eq!(m.get(&Collide(49)), Some(&49));
    assert_eq!(m.len(), 49);
}

#[test]
fn string_keys_are_looked_up_by_str() {
    let mut m: ChainedMap<String, i32> = ChainedMap::new();
    m.insert("héllo".to_string(), 1);
    m.insert(String::new(), 2);
    // No String allocated for the lookups: K = String borrows as str.
    assert_eq!(m.get("héllo"), Some(&1));
    assert_eq!(m.get(""), Some(&2));
    assert!(m.contains_key("héllo"));
    assert!(!m.contains_key("hello"));
    *m.get_mut("").unwrap() = 3;
    assert_eq!(m.remove(""), Some(3));
    assert_eq!(m.len(), 1);
}

#[test]
fn keys_and_values_need_not_be_clone() {
    let mut m = ChainedMap::new();
    for i in 0..30 {
        m.insert(Token(i), Token(i * 2));
    }
    assert_eq!(m.get(&Token(7)), Some(&Token(14)));
    assert_eq!(m.insert(Token(7), Token(0)), Some(Token(14)));
    assert_eq!(m.remove(&Token(29)), Some(Token(58)));
    assert_eq!(m.len(), 29);
}

#[test]
fn get_or_insert_with_only_calls_default_when_missing() {
    let mut m = ChainedMap::new();
    let mut calls = 0;
    *m.get_or_insert_with('a', || {
        calls += 1;
        10
    }) += 1;
    *m.get_or_insert_with('a', || {
        calls += 1;
        10
    }) += 1;
    assert_eq!(calls, 1, "the second call must find the existing value");
    assert_eq!(m.get(&'a'), Some(&12));
    assert_eq!(m.len(), 1);
}

#[test]
fn get_or_insert_with_survives_resizes() {
    let mut m: ChainedMap<u32, Vec<u32>> = ChainedMap::new();
    for i in 0..2_000 {
        m.get_or_insert_with(i % 700, Vec::new).push(i);
    }
    assert_eq!(m.len(), 700);
    assert_eq!(m.get(&0), Some(&vec![0, 700, 1400]));
    assert_eq!(m.get(&699), Some(&vec![699, 1399]));
}

#[test]
fn iter_visits_every_pair_once() {
    let mut m = ChainedMap::new();
    for i in 0..200 {
        m.insert(i, i * i);
    }
    for i in (0..200).step_by(3) {
        m.remove(&i);
    }
    let expected: Vec<(i32, i32)> = (0..200).filter(|i| i % 3 != 0).map(|i| (i, i * i)).collect();
    assert_eq!(sorted_pairs(&m), expected);
}

#[test]
fn for_loops_borrow_the_map() {
    let mut m = ChainedMap::new();
    for i in 1..=10u64 {
        m.insert(i, i * 2);
    }
    let mut total = 0;
    for (k, v) in &m {
        total += k + v;
    }
    assert_eq!(total, 55 * 3);
    assert_eq!(m.len(), 10, "iterating doesn't consume");
}

#[test]
fn every_value_is_dropped_exactly_once() {
    let drops = Rc::new(Cell::new(0));
    {
        let mut m = ChainedMap::new();
        for i in 0..100 {
            m.insert(i, Counted(Rc::clone(&drops)));
        }
        drop(m.remove(&3));
        drop(m.insert(4, Counted(Rc::clone(&drops))));
        assert_eq!(drops.get(), 2, "remove and overwrite hand back the old value");
    }
    assert_eq!(drops.get(), 101, "dropping the map drops what's left");
}

#[test]
fn random_operations_match_std_hashmap() {
    let mut rng = Rng(0x2545_F491_4F6C_DD1D);
    let mut m = ChainedMap::new();
    let mut model: HashMap<u64, u64> = HashMap::new();
    for step in 0..30_000 {
        let key = rng.below(2_000);
        match rng.below(6) {
            0 | 1 => {
                let v = rng.next();
                assert_eq!(m.insert(key, v), model.insert(key, v), "step {step}: insert {key}");
            }
            2 => assert_eq!(m.remove(&key), model.remove(&key), "step {step}: remove {key}"),
            3 => assert_eq!(m.get(&key), model.get(&key), "step {step}: get {key}"),
            4 => {
                let got = *m.get_or_insert_with(key, || step);
                let want = *model.entry(key).or_insert(step);
                assert_eq!(got, want, "step {step}: get_or_insert_with {key}");
            }
            _ => {
                if let Some(v) = m.get_mut(&key) {
                    *v = v.wrapping_add(1);
                }
                if let Some(v) = model.get_mut(&key) {
                    *v = v.wrapping_add(1);
                }
            }
        }
        assert_eq!(m.len(), model.len(), "step {step}");
    }
    let mut expected: Vec<(u64, u64)> = model.into_iter().collect();
    expected.sort_unstable();
    assert_eq!(sorted_pairs(&m), expected);
}

#[test]
fn many_keys_keep_chains_short() {
    // Counts key comparisons rather than time. With the load factor bounded,
    // a lookup compares against one or two keys; a bucket array that stops
    // growing (fixed or capped) makes chains, and comparisons, pile up.
    let n = 40_000u64;
    let mut m = ChainedMap::new();
    let start = eq_calls();
    for i in 0..n {
        assert_eq!(m.insert(Probe(i), i), None);
        let done = i + 1;
        if done % 1_000 == 0 {
            let calls = eq_calls() - start;
            assert!(
                calls <= 2 * done,
                "{done} inserts made {calls} key comparisons: chains are too long — does the bucket array keep growing?"
            );
        }
    }
    assert_eq!(m.len(), n as usize);
    let start = eq_calls();
    for i in 0..2 * n {
        let want = if i < n { Some(&i) } else { None };
        assert_eq!(m.get(&Probe(i)), want);
        let done = i + 1;
        if done % 1_000 == 0 {
            let calls = eq_calls() - start;
            assert!(
                calls <= 3 * done,
                "{done} lookups made {calls} key comparisons: chains are too long"
            );
        }
    }
}

#[test]
fn resizing_rehashes_rarely() {
    // Counts hashes rather than time. Doubling the bucket array rehashes each
    // key a constant number of times on average; growing it by a fixed number
    // of buckets rehashes every key every few inserts, which is quadratic.
    let n = 20_000u64;
    let mut m = ChainedMap::new();
    let start = hash_calls();
    for i in 0..n {
        assert_eq!(m.insert(Probe(i), i), None);
        let done = i + 1;
        let calls = hash_calls() - start;
        assert!(calls <= 10 * done, "{done} inserts hashed keys {calls} times: is the bucket array rebuilt too often?");
    }
    assert_eq!(m.len(), n as usize);
    assert_eq!(m.get(&Probe(n - 1)), Some(&(n - 1)));
}
