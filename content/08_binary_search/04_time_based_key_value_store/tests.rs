// Tests for Time Based Key-Value Store — shared by template.rs, solution.rs and your attempt.
use super::*;

/// Model: a flat log of (key, value, timestamp); get scans for the best match.
fn brute_get<'a>(log: &'a [(String, String, u32)], key: &str, t: u32) -> Option<&'a String> {
    log.iter()
        .filter(|(k, _, ts)| k == key && *ts <= t)
        .max_by_key(|(_, _, ts)| *ts)
        .map(|(_, v, _)| v)
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

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

#[test]
fn example_1() {
    let mut tm = TimeMap::new();
    tm.set("foo", "bar", 1);
    assert_eq!(tm.get("foo", 1), Some(&"bar"));
    assert_eq!(tm.get("foo", 3), Some(&"bar"));
    tm.set("foo", "bar2", 4);
    assert_eq!(tm.get("foo", 4), Some(&"bar2"));
    assert_eq!(tm.get("foo", 5), Some(&"bar2"));
    assert_eq!(tm.get("foo", 0), None);
    assert_eq!(tm.get("baz", 9), None);
}

#[test]
fn example_2_numbers_via_default() {
    let mut prices: TimeMap<u64> = TimeMap::default();
    prices.set("btc", 30_000, 10);
    assert_eq!(prices.get("btc", 11), Some(&30_000));
}

#[test]
fn empty_store() {
    let tm: TimeMap<String> = TimeMap::new();
    assert_eq!(tm.get("anything", 0), None);
    assert_eq!(tm.get("anything", u32::MAX), None);
}

#[test]
fn unknown_key_is_none() {
    let mut tm = TimeMap::new();
    assert_eq!(tm.get("nope", 10), None);
    tm.set("a", "1", 5);
    assert_eq!(tm.get("b", 5), None);
}

#[test]
fn query_before_first_set_is_none() {
    let mut tm = TimeMap::new();
    tm.set("k", "v", 10);
    assert_eq!(tm.get("k", 9), None);
    assert_eq!(tm.get("k", 1), None);
    assert_eq!(tm.get("k", 10), Some(&"v"));
}

#[test]
fn exact_timestamp_is_inclusive() {
    let mut tm = TimeMap::new();
    tm.set("k", "a", 2);
    tm.set("k", "b", 4);
    tm.set("k", "c", 6);
    assert_eq!(tm.get("k", 2), Some(&"a"));
    assert_eq!(tm.get("k", 4), Some(&"b"));
    assert_eq!(tm.get("k", 6), Some(&"c"));
}

#[test]
fn between_timestamps_takes_floor_not_ceiling_trap() {
    let mut tm = TimeMap::new();
    tm.set("k", "a", 2);
    tm.set("k", "b", 4);
    tm.set("k", "c", 6);
    assert_eq!(tm.get("k", 3), Some(&"a"));
    assert_eq!(tm.get("k", 5), Some(&"b"));
    assert_eq!(tm.get("k", 7), Some(&"c"));
    assert_eq!(tm.get("k", 10_000_000), Some(&"c"));
}

#[test]
fn timestamp_extremes() {
    let mut tm = TimeMap::new();
    tm.set("k", "zero", 0);
    tm.set("k", "max", u32::MAX);
    assert_eq!(tm.get("k", 0), Some(&"zero"));
    assert_eq!(tm.get("k", u32::MAX - 1), Some(&"zero"));
    assert_eq!(tm.get("k", u32::MAX), Some(&"max"));
}

#[test]
fn keys_are_independent() {
    // Timestamps are global and interleave between keys.
    let mut tm = TimeMap::new();
    tm.set("a", "a1", 1);
    tm.set("b", "b2", 2);
    tm.set("a", "a3", 3);
    tm.set("b", "b4", 4);
    assert_eq!(tm.get("a", 2), Some(&"a1"));
    assert_eq!(tm.get("b", 1), None);
    assert_eq!(tm.get("b", 3), Some(&"b2"));
    assert_eq!(tm.get("a", 4), Some(&"a3"));
    assert_eq!(tm.get("b", 4), Some(&"b4"));
}

#[test]
fn similar_keys_are_distinct() {
    let mut tm = TimeMap::new();
    tm.set("key", 1, 1);
    tm.set("Key", 2, 2);
    tm.set("key ", 3, 3);
    assert_eq!(tm.get("key", 10), Some(&1));
    assert_eq!(tm.get("Key", 10), Some(&2));
    assert_eq!(tm.get("key ", 10), Some(&3));
    assert_eq!(tm.get("ke", 10), None);
}

#[test]
fn same_value_at_different_times() {
    let mut tm = TimeMap::new();
    tm.set("k", "x", 1);
    tm.set("k", "y", 2);
    tm.set("k", "x", 3);
    assert_eq!(tm.get("k", 1), Some(&"x"));
    assert_eq!(tm.get("k", 2), Some(&"y"));
    assert_eq!(tm.get("k", 3), Some(&"x"));
}

#[test]
fn long_and_non_ascii_keys_and_values() {
    let key = "k".repeat(100);
    let val = "9".repeat(100);
    let mut tm = TimeMap::new();
    tm.set(&key, val.clone(), 10_000_000);
    assert_eq!(tm.get(&key, 10_000_000), Some(&val));
    assert_eq!(tm.get(&key, 9_999_999), None);
    tm.set("ключ", "значение".to_string(), 10_000_001);
    assert_eq!(tm.get("ключ", 10_000_001).map(String::as_str), Some("значение"));
}

#[test]
fn get_does_not_mutate() {
    let mut tm = TimeMap::new();
    tm.set("k", "v", 5);
    for _ in 0..3 {
        assert_eq!(tm.get("k", 100), Some(&"v"));
        assert_eq!(tm.get("k", 4), None);
    }
}

#[test]
fn single_version_many_queries() {
    let mut tm = TimeMap::new();
    tm.set("only", "value", 500);
    for t in [0, 1, 250, 499] {
        assert_eq!(tm.get("only", t), None, "t = {t}");
    }
    for t in [500, 501, 10_000_000] {
        assert_eq!(tm.get("only", t), Some(&"value"), "t = {t}");
    }
}

#[test]
fn values_need_not_be_clone() {
    #[derive(Debug, PartialEq)]
    struct Snapshot(Vec<u8>);
    let mut tm = TimeMap::new();
    tm.set("doc", Snapshot(vec![1]), 1);
    tm.set("doc", Snapshot(vec![1, 2]), 2);
    assert_eq!(tm.get("doc", 1), Some(&Snapshot(vec![1])));
    assert_eq!(tm.get("doc", 9).map(|s| s.0.len()), Some(2));
}

#[test]
fn key_from_owned_string() {
    let mut tm = TimeMap::new();
    let key = String::from("dynamic");
    tm.set(&key, 'a', 1);
    drop(key); // the store keeps its own copy of the key
    assert_eq!(tm.get("dynamic", 1), Some(&'a'));
}

#[test]
fn randomized_vs_model() {
    let keys = ["a", "b", "c", "dd"];
    let mut rng = Rng(0x3301_0123_4567);
    for round in 0..30 {
        let mut tm = TimeMap::new();
        let mut log: Vec<(String, String, u32)> = Vec::new();
        let mut now = 0;
        for _ in 0..150 {
            let key = keys[rng.below(keys.len() as u64) as usize];
            if rng.below(3) == 0 {
                now += 1 + rng.below(4) as u32;
                let value = format!("v{}", rng.below(10));
                tm.set(key, value.clone(), now);
                log.push((key.to_string(), value, now));
            } else {
                let t = rng.below(u64::from(now) + 5) as u32;
                assert_eq!(
                    tm.get(key, t),
                    brute_get(&log, key, t),
                    "round {round}: get({key:?}, {t}) after sets {log:?}"
                );
            }
        }
    }
}

#[test]
fn long_history_on_one_key_many_gets() {
    // N sets on one key, then N gets at random times. A scan of the history
    // (from either end) averages N / 2 steps per get, about 5 * 10^8 steps in
    // total; a binary search takes about 15.
    let n: u32 = 30_000;
    let step = 10;
    let mut tm = TimeMap::new();
    for i in 0..n {
        tm.set("hot", i, step * i + 5);
    }
    let mut rng = Rng(0x3302_aaaa_bbbb);
    for _ in 0..n {
        let t = rng.below(u64::from(step * n + 10)) as u32;
        let expected = t.checked_sub(5).map(|d| (d / step).min(n - 1));
        assert_eq!(tm.get("hot", t).copied(), expected, "n = {n}, t = {t}");
    }
}

#[test]
fn large_many_keys() {
    let n: u32 = 50_000;
    let mut tm = TimeMap::new();
    for i in 0..n {
        tm.set(&format!("key{}", i % 100), format!("val{i}"), i + 1);
    }
    for i in 0..n {
        let key = format!("key{}", i % 100);
        // The latest set for this key at time i + 1 is the one with index i.
        assert_eq!(tm.get(&key, i + 1), Some(&format!("val{i}")));
        // Just before it, it's the previous set for the same key (index i - 100).
        let prev = i.checked_sub(100).map(|p| format!("val{p}"));
        assert_eq!(tm.get(&key, i), prev.as_ref(), "i = {i}");
    }
}
