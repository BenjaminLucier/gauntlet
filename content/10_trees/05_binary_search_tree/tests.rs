// Tests for Binary Search Tree — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::cell::Cell;
use std::collections::BTreeMap;
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

fn build(keys: &[i32]) -> Bst<i32, i32> {
    let mut m = Bst::new();
    for &k in keys {
        m.insert(k, k * 10);
    }
    m
}

fn keys<K: Ord + Clone, V>(m: &Bst<K, V>) -> Vec<K> {
    m.iter().map(|(k, _)| k.clone()).collect()
}

/// Counts its drops, to check every value is dropped exactly once.
#[derive(Debug)]
struct Counted(Rc<Cell<usize>>);

impl Drop for Counted {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[test]
fn readme_example() {
    let mut m = Bst::new();
    assert_eq!(m.insert(5, "five"), None);
    m.insert(2, "two");
    m.insert(8, "eight");
    assert_eq!(m.insert(5, "FIVE"), Some("five"));
    assert_eq!(m.len(), 3);
    assert_eq!(m.get(&5), Some(&"FIVE"));
    assert!(m.contains_key(&2));
    assert_eq!(m.min(), Some((&2, &"two")));
    assert_eq!(m.floor(&7), Some((&5, &"FIVE")));
    assert_eq!(m.ceiling(&9), None);
    assert_eq!(m.remove(&5), Some("FIVE"));
    assert_eq!(m.iter().collect::<Vec<_>>(), [(&2, &"two"), (&8, &"eight")]);

    let mut names: Bst<String, u32> = [("bo".to_string(), 1)].into_iter().collect();
    names.extend([("al".to_string(), 2), ("bo".to_string(), 3)]);
    assert_eq!(names.get("bo"), Some(&3));
    assert_eq!(names.len(), 2);
}

#[test]
fn new_tree_is_empty() {
    let mut m: Bst<i32, String> = Bst::new();
    assert_eq!(m.len(), 0);
    assert!(m.is_empty());
    assert_eq!(m.get(&1), None);
    assert_eq!(m.get_mut(&1), None);
    assert!(!m.contains_key(&1));
    assert_eq!(m.remove(&1), None);
    assert_eq!(m.min(), None);
    assert_eq!(m.max(), None);
    assert_eq!(m.floor(&1), None);
    assert_eq!(m.ceiling(&1), None);
    assert_eq!(m.iter().next(), None);
    let d: Bst<u8, u8> = Bst::default();
    assert!(d.is_empty());
}

#[test]
fn insert_replaces_and_returns_the_old_value() {
    let mut m = Bst::new();
    assert_eq!(m.insert("k", 1), None);
    assert_eq!(m.insert("k", 2), Some(1));
    assert_eq!(m.insert("k", 3), Some(2));
    assert_eq!(m.len(), 1, "replacing doesn't add an entry");
    assert_eq!(m.get(&"k"), Some(&3));
}

#[test]
fn get_mut_updates_in_place() {
    let mut m = build(&[4, 2, 6]);
    *m.get_mut(&2).expect("2 is present") += 1;
    if let Some(v) = m.get_mut(&6) {
        *v = -1;
    }
    assert_eq!(m.get_mut(&5), None);
    assert_eq!(m.iter().collect::<Vec<_>>(), [(&2, &21), (&4, &40), (&6, &-1)]);
}

#[test]
fn remove_a_leaf() {
    let mut m = build(&[50, 30, 70, 20]);
    assert_eq!(m.remove(&20), Some(200));
    assert_eq!(keys(&m), [30, 50, 70]);
    assert_eq!(m.remove(&70), Some(700));
    assert_eq!(keys(&m), [30, 50]);
    assert_eq!(m.len(), 2);
}

#[test]
fn remove_a_node_with_one_child() {
    // 30 has only a left child, 70 only a right child.
    let mut m = build(&[50, 30, 70, 20, 80, 10, 90]);
    assert_eq!(m.remove(&30), Some(300));
    assert_eq!(keys(&m), [10, 20, 50, 70, 80, 90]);
    assert_eq!(m.remove(&70), Some(700));
    assert_eq!(keys(&m), [10, 20, 50, 80, 90]);
    assert_eq!(m.get(&10), Some(&100), "the child subtree moved up intact");
    assert_eq!(m.get(&90), Some(&900));
}

#[test]
fn remove_a_node_with_two_children() {
    let mut m = build(&[50, 30, 70, 20, 40, 60, 80]);
    assert_eq!(m.remove(&30), Some(300));
    assert_eq!(keys(&m), [20, 40, 50, 60, 70, 80]);
    assert_eq!(m.remove(&50), Some(500)); // the root
    assert_eq!(keys(&m), [20, 40, 60, 70, 80]);
    assert_eq!(m.len(), 5);
    for k in [20, 40, 60, 70, 80] {
        assert_eq!(m.get(&k), Some(&(k * 10)), "key {k} lost");
    }
}

#[test]
fn successor_with_a_right_child_keeps_it() {
    // Removing 50: its successor 60 has a right child 65, which must be
    // spliced into 60's old place rather than dropped.
    let mut m = build(&[50, 30, 70, 60, 80, 65, 62]);
    assert_eq!(m.remove(&50), Some(500));
    assert_eq!(keys(&m), [30, 60, 62, 65, 70, 80]);
    assert_eq!(m.get(&62), Some(&620));
    assert_eq!(m.len(), 6);
}

#[test]
fn successor_is_the_direct_right_child() {
    // 70 has no left child, so it is 50's successor itself.
    let mut m = build(&[50, 30, 70, 80, 90]);
    assert_eq!(m.remove(&50), Some(500));
    assert_eq!(keys(&m), [30, 70, 80, 90]);
    assert_eq!(m.min(), Some((&30, &300)));
}

#[test]
fn removing_a_missing_key_changes_nothing() {
    let mut m = build(&[5, 3, 8]);
    assert_eq!(m.remove(&4), None);
    assert_eq!(m.remove(&9), None);
    assert_eq!(m.len(), 3);
    assert_eq!(m.remove(&3), Some(30));
    assert_eq!(m.remove(&3), None, "already gone");
    assert_eq!(m.len(), 2);
}

#[test]
fn remove_the_root_until_empty() {
    // Removing in insertion order removes whatever is the root each time.
    let order = [8, 4, 12, 2, 6, 10, 14, 1, 3, 5, 7, 9, 11, 13, 15];
    let mut m = build(&order);
    for (i, k) in order.iter().enumerate() {
        assert_eq!(m.remove(k), Some(k * 10));
        assert_eq!(m.len(), order.len() - i - 1);
        let mut rest = order[i + 1..].to_vec();
        rest.sort_unstable();
        assert_eq!(keys(&m), rest);
    }
    assert!(m.is_empty());
    m.insert(1, 1);
    assert_eq!(keys(&m), [1]);
}

#[test]
fn min_and_max() {
    let mut m = build(&[5, 9, 1, 7, 3]);
    assert_eq!(m.min(), Some((&1, &10)));
    assert_eq!(m.max(), Some((&9, &90)));
    m.remove(&1);
    m.remove(&9);
    assert_eq!(m.min(), Some((&3, &30)));
    assert_eq!(m.max(), Some((&7, &70)));
    let one = build(&[-4]);
    assert_eq!(one.min(), one.max());
}

#[test]
fn floor_and_ceiling() {
    let m = build(&[20, 10, 30, 5, 15, 25, 35]);
    let k = |e: Option<(&i32, &i32)>| e.map(|(k, _)| *k);
    assert_eq!(k(m.floor(&15)), Some(15), "exact match");
    assert_eq!(k(m.ceiling(&15)), Some(15), "exact match");
    assert_eq!(k(m.floor(&24)), Some(20));
    assert_eq!(k(m.ceiling(&24)), Some(25));
    assert_eq!(k(m.floor(&16)), Some(15), "answer is deeper than the last right turn");
    assert_eq!(k(m.ceiling(&21)), Some(25));
    assert_eq!(k(m.floor(&4)), None, "below every key");
    assert_eq!(k(m.ceiling(&4)), Some(5));
    assert_eq!(k(m.floor(&100)), Some(35));
    assert_eq!(k(m.ceiling(&36)), None, "above every key");
    assert_eq!(m.floor(&25), Some((&25, &250)));
}

#[test]
fn iteration_is_in_order_and_borrows() {
    let m = build(&[4, 2, 6, 1, 3, 5, 7]);
    let mut it = m.iter();
    assert_eq!(it.next(), Some((&1, &10)));
    let mut other = m.iter();
    assert_eq!(it.next(), Some((&2, &20)));
    assert_eq!(other.next(), Some((&1, &10)), "iterators are independent");
    let mut sum = 0;
    for (k, v) in &m {
        sum += k + v;
    }
    assert_eq!(sum, 28 * 11);
    assert_eq!(m.iter().count(), 7);
    assert_eq!(m.len(), 7, "iterating doesn't consume");
}

#[test]
fn string_keys_and_values_without_clone() {
    #[derive(Debug, PartialEq)]
    struct Token(u32);
    let mut m: Bst<String, Token> = Bst::new();
    for (i, w) in ["pear", "apple", "fig", "kiwi", "banana"].iter().enumerate() {
        m.insert(w.to_string(), Token(i as u32));
    }
    assert_eq!(keys(&m), ["apple", "banana", "fig", "kiwi", "pear"]);
    assert_eq!(m.remove(&"pear".to_string()), Some(Token(0)));
    assert_eq!(m.insert("fig".to_string(), Token(9)), Some(Token(2)));
    assert_eq!(m.ceiling(&"c".to_string()).map(|(k, _)| k.as_str()), Some("fig"));
    assert_eq!(m.get(&"fig".to_string()), Some(&Token(9)));
}

#[test]
fn string_keys_are_looked_up_by_str() {
    let mut m: Bst<String, usize> = Bst::new();
    for w in ["pear", "apple", "fig", "kiwi", "banana"] {
        m.insert(w.to_string(), w.len());
    }
    // Every lookup takes a borrowed form of the key: no String needed.
    assert_eq!(m.get("fig"), Some(&3));
    assert_eq!(m.get("grape"), None);
    assert!(m.contains_key("kiwi"));
    assert!(!m.contains_key("Kiwi"));
    *m.get_mut("apple").expect("apple is present") += 100;
    assert_eq!(m.get("apple"), Some(&105));
    assert_eq!(m.floor("c").map(|(k, _)| k.as_str()), Some("banana"));
    assert_eq!(m.ceiling("c").map(|(k, _)| k.as_str()), Some("fig"));
    assert_eq!(m.remove("pear"), Some(4));
    assert_eq!(m.remove("pear"), None);
    assert_eq!(keys(&m), ["apple", "banana", "fig", "kiwi"]);
    // Vec<u8> keys borrow as [u8].
    let mut bytes: Bst<Vec<u8>, ()> = Bst::new();
    bytes.insert(b"xy".to_vec(), ());
    assert!(bytes.contains_key(&b"xy"[..]));
}

#[test]
fn collect_and_extend() {
    let m: Bst<i32, char> = [(3, 'c'), (1, 'a'), (2, 'b')].into_iter().collect();
    assert_eq!(m.iter().collect::<Vec<_>>(), [(&1, &'a'), (&2, &'b'), (&3, &'c')]);
    // Later duplicates replace earlier values, like repeated insert.
    let mut m: Bst<i32, char> = [(1, 'x'), (1, 'y')].into_iter().collect();
    assert_eq!(m.len(), 1);
    assert_eq!(m.get(&1), Some(&'y'));
    m.extend([(5, 'e'), (1, 'z'), (0, 'o')]);
    assert_eq!(m.iter().collect::<Vec<_>>(), [(&0, &'o'), (&1, &'z'), (&5, &'e')]);
    m.extend(std::iter::empty());
    assert_eq!(m.len(), 3);
    let empty: Bst<u8, u8> = std::iter::empty().collect();
    assert!(empty.is_empty());
    // Matches BTreeMap on a bigger random batch.
    let mut rng = Rng(77);
    let pairs: Vec<(u64, u64)> = (0..2_000).map(|i| (rng.below(500), i)).collect();
    let tree: Bst<u64, u64> = pairs.iter().copied().collect();
    let model: BTreeMap<u64, u64> = pairs.iter().copied().collect();
    assert_eq!(tree.len(), model.len());
    assert!(tree.iter().eq(model.iter()));
}

#[test]
fn every_value_is_dropped_exactly_once() {
    let drops = Rc::new(Cell::new(0));
    {
        let mut m = Bst::new();
        for k in [50, 30, 70, 20, 40, 60, 80, 65] {
            m.insert(k, Counted(Rc::clone(&drops)));
        }
        drop(m.insert(40, Counted(Rc::clone(&drops)))); // the old value comes back
        assert_eq!(drops.get(), 1);
        drop(m.remove(&50)); // two children
        drop(m.remove(&20)); // leaf
        drop(m.remove(&60)); // one child
        assert_eq!(drops.get(), 4);
        assert!(m.remove(&50).is_none());
        assert_eq!(m.len(), 5);
    }
    assert_eq!(drops.get(), 9, "dropping the tree drops what's left");
}

/// Run `f` on a thread with a deliberately small stack: anything that
/// recurses once per level of a few-thousand-deep path overflows it.
fn small_stack(f: impl FnOnce() + Send + 'static) {
    let h = std::thread::Builder::new()
        .stack_size(128 << 10)
        .spawn(f)
        .expect("spawn test thread");
    if let Err(e) = h.join() {
        std::panic::resume_unwind(e);
    }
}

#[test]
fn sorted_inserts_make_a_deep_path_without_recursion() {
    small_stack(|| {
        let n = 6_000;
        for ascending in [true, false] {
            let mut m = Bst::new();
            let order: Vec<i32> = if ascending {
                (0..n).collect()
            } else {
                (0..n).rev().collect()
            };
            for &k in &order {
                assert_eq!(m.insert(k, k), None);
            }
            assert_eq!(m.len(), n as usize);
            assert_eq!(m.get(&(n - 1)), Some(&(n - 1)));
            assert_eq!(m.floor(&n), Some((&(n - 1), &(n - 1))));
            assert_eq!(m.ceiling(&-1), Some((&0, &0)));
            assert_eq!(m.min(), Some((&0, &0)));
            assert_eq!(m.max(), Some((&(n - 1), &(n - 1))));
            assert!(m.iter().map(|(k, _)| *k).eq(0..n));
            // The deepest node, then the root.
            assert_eq!(m.remove(order.last().expect("non-empty")), order.last().copied());
            assert_eq!(m.remove(&order[0]), Some(order[0]));
            assert_eq!(m.len(), n as usize - 2);
            // ...and the drop at the end of this scope must not recurse either.
        }
    });
}

#[test]
fn random_operations_match_btreemap() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let mut m = Bst::new();
    let mut model: BTreeMap<i32, u64> = BTreeMap::new();
    for step in 0..30_000 {
        let key = rng.below(300) as i32 - 150;
        match rng.below(8) {
            0..=2 => {
                let v = rng.next();
                assert_eq!(m.insert(key, v), model.insert(key, v), "step {step}: insert({key})");
            }
            3 | 4 => assert_eq!(m.remove(&key), model.remove(&key), "step {step}: remove({key})"),
            5 => {
                assert_eq!(m.get(&key), model.get(&key), "step {step}: get({key})");
                assert_eq!(m.contains_key(&key), model.contains_key(&key), "step {step}");
                if let (Some(a), Some(b)) = (m.get_mut(&key), model.get_mut(&key)) {
                    *a ^= 1;
                    *b ^= 1;
                }
            }
            6 => {
                assert_eq!(m.floor(&key), model.range(..=key).next_back(), "step {step}: floor({key})");
                assert_eq!(m.ceiling(&key), model.range(key..).next(), "step {step}: ceiling({key})");
            }
            _ => {
                assert_eq!(m.min(), model.iter().next(), "step {step}: min");
                assert_eq!(m.max(), model.iter().next_back(), "step {step}: max");
            }
        }
        assert_eq!(m.len(), model.len(), "step {step}");
        if step % 1000 == 0 {
            assert!(m.iter().eq(model.iter()), "step {step}: contents differ");
        }
    }
    assert!(m.iter().eq(model.iter()));
}

/// A key that counts its comparisons, so a lookup that scans the tree instead
/// of following one root-to-leaf path shows up.
#[derive(Debug)]
struct Probe(u64);

thread_local! {
    static COMPARISONS: Cell<usize> = const { Cell::new(0) };
}

fn comparisons() -> usize {
    COMPARISONS.with(Cell::get)
}

impl PartialEq for Probe {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}
impl Eq for Probe {}
impl PartialOrd for Probe {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Probe {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        COMPARISONS.with(|c| c.set(c.get() + 1));
        self.0.cmp(&other.0)
    }
}

#[test]
fn operations_compare_only_along_one_path() {
    // 4096 random keys: the tree is ~25 levels deep and an average search
    // path is ~17 nodes. Scanning the keys (with `iter`, or by visiting both
    // subtrees) costs thousands of comparisons per operation.
    let n = 4096;
    let mut rng = Rng(0x05B5_7000);
    let keys: Vec<u64> = (0..n).map(|_| 2 * rng.below(1 << 40)).collect();
    let mut m = Bst::new();
    let before = comparisons();
    for &k in &keys {
        m.insert(Probe(k), k);
    }
    let mut report = vec![("insert", comparisons() - before)];
    let mut measure = |name, op: &mut dyn FnMut(u64)| {
        let before = comparisons();
        for &k in &keys {
            op(k);
        }
        report.push((name, comparisons() - before));
    };
    measure("get", &mut |k| assert_eq!(m.get(&Probe(k)), Some(&k)));
    measure("contains_key", &mut |k| assert!(!m.contains_key(&Probe(k + 1))));
    measure("floor", &mut |k| assert!(m.floor(&Probe(k + 1)).is_some_and(|(f, _)| f.0 >= k)));
    measure("ceiling", &mut |k| assert!(m.ceiling(&Probe(k - 1)).is_some_and(|(c, _)| c.0 <= k)));
    measure("remove", &mut |k| _ = m.remove(&Probe(k)));
    assert!(m.is_empty(), "every key was removed");
    for (op, count) in report {
        // The reference makes ~16 comparisons per call here.
        assert!(count <= 60 * n as usize, "{op}: {count} comparisons for {n} calls on random keys");
    }
}

#[test]
fn large_tree_drained_from_both_ends() {
    // 2.2 * 10^4 random keys, then 1.1 * 10^4 rounds of taking the largest
    // and the smallest key. Finding either by iterating over the tree costs
    // ~n steps per call: ~2.4 * 10^8 in all. Walking one spine is ~20 steps.
    let mut rng = Rng(12345);
    let mut m = Bst::new();
    let n = 22_000u64;
    for _ in 0..n {
        let k = rng.below(1 << 40);
        m.insert(k, k);
    }
    let mut prev = (0, u64::MAX);
    for round in 0..n / 2 {
        let (&lo, &v) = m.min().expect("tree is not empty");
        assert_eq!(lo, v);
        let (&hi, _) = m.max().expect("tree is not empty");
        assert!(prev.0 <= lo && hi <= prev.1 && lo <= hi, "round {round}, n = {n}");
        assert_eq!(m.remove(&lo), Some(lo));
        if hi != lo {
            assert_eq!(m.remove(&hi), Some(hi));
        }
        prev = (lo, hi);
    }
    let mut last = None;
    for (&k, &v) in &m {
        assert_eq!(k, v);
        assert!(last < Some(k), "keys out of order");
        last = Some(k);
    }
}
