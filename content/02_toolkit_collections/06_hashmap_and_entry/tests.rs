// Tests for HashMap and Entry — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::cell::Cell;

fn set(xs: &[i32]) -> HashSet<i32> {
    xs.iter().copied().collect()
}

fn owned(pairs: &[(&str, u32)]) -> HashMap<String, u32> {
    pairs.iter().map(|&(k, v)| (k.to_string(), v)).collect()
}

// ── Counting with entry ──

#[test]
fn word_counts_counts_each_word() {
    let counts = word_counts("the cat and the  hat\nthe end");
    assert_eq!(counts.len(), 5);
    assert_eq!(counts["the"], 3);
    assert_eq!(counts["cat"], 1);
    assert_eq!(counts.get("dog"), None);
    assert!(word_counts("   ").is_empty());
}

#[test]
fn group_by_len_keeps_order_within_groups() {
    let groups = group_by_len(&["ox", "cat", "be", "dog", "a"]);
    assert_eq!(groups.len(), 3);
    assert_eq!(groups[&2], ["ox", "be"]);
    assert_eq!(groups[&3], ["cat", "dog"]);
    assert_eq!(groups[&1], ["a"]);
    assert!(group_by_len(&[]).is_empty());
}

#[test]
fn first_index_does_not_overwrite() {
    let first = first_index(&['a', 'b', 'a', 'c', 'b']);
    assert_eq!(first, HashMap::from([('a', 0), ('b', 1), ('c', 3)]));
    let nums = first_index(&[7, 7, 7]);
    assert_eq!(nums, HashMap::from([(7, 0)]));
}

#[test]
fn bump_starts_at_one_then_increments() {
    let mut map = HashMap::new();
    bump(&mut map, "x");
    bump(&mut map, "x");
    bump(&mut map, "y");
    assert_eq!(map, owned(&[("x", 2), ("y", 1)]));
}

#[test]
fn cached_len_computes_only_on_miss() {
    let mut cache = HashMap::new();
    let calls = Cell::new(0);
    let compute = |s: &str| {
        calls.set(calls.get() + 1);
        s.len() * 10
    };
    assert_eq!(cached_len(&mut cache, "abc", compute), 30);
    assert_eq!(cached_len(&mut cache, "abc", compute), 30);
    assert_eq!(calls.get(), 1, "the second call must hit the cache");
    assert_eq!(cached_len(&mut cache, "z", compute), 10);
    assert_eq!(calls.get(), 2);
    assert_eq!(cache.len(), 2);
}

#[test]
fn cached_len_trusts_the_cache() {
    let mut cache = HashMap::from([("k".to_string(), 99)]);
    assert_eq!(cached_len(&mut cache, "k", |_| 0), 99);
}

#[test]
fn merge_counts_adds_and_inserts() {
    let mut total = owned(&[("a", 1), ("b", 2)]);
    merge_counts(&mut total, owned(&[("b", 3), ("c", 4)]));
    assert_eq!(total, owned(&[("a", 1), ("b", 5), ("c", 4)]));
    merge_counts(&mut total, HashMap::new());
    assert_eq!(total.len(), 3);
}

#[test]
fn take_one_decrements_then_removes() {
    let mut stock = HashMap::from([('a', 2), ('b', 1)]);
    assert!(take_one(&mut stock, 'a'));
    assert_eq!(stock, HashMap::from([('a', 1), ('b', 1)]));
    assert!(take_one(&mut stock, 'a'));
    assert_eq!(stock, HashMap::from([('b', 1)]), "a count of 0 is removed, not kept");
    assert!(take_one(&mut stock, 'b'));
    assert!(stock.is_empty());
}

#[test]
fn take_one_missing_changes_nothing() {
    let mut stock = HashMap::from([('x', 3)]);
    assert!(!take_one(&mut stock, 'y'));
    assert_eq!(stock, HashMap::from([('x', 3)]), "no entry inserted for a miss");
    let mut empty = HashMap::new();
    assert!(!take_one(&mut empty, 'x'));
    assert!(empty.is_empty());
}

// ── Looking up ──

#[test]
fn age_of_looks_up_with_str() {
    let ages = owned(&[("ferris", 9), ("corro", 4)]);
    assert_eq!(age_of(&ages, "ferris"), Some(9));
    assert_eq!(age_of(&ages, "nobody"), None);
}

#[test]
fn add_if_present_never_inserts() {
    let mut map: HashMap<String, i32> = HashMap::from([("a".to_string(), 1)]);
    assert!(add_if_present(&mut map, "a", 5));
    assert!(!add_if_present(&mut map, "b", 5));
    assert_eq!(map, HashMap::from([("a".to_string(), 6)]));
}

#[test]
fn has_all_checks_every_key() {
    let map = HashMap::from([("x", 1), ("y", 2)]);
    assert!(has_all(&map, &["x", "y"]));
    assert!(!has_all(&map, &["x", "z"]));
    assert!(has_all(&map, &[]), "vacuously true");
}

// ── Removing ──

#[test]
fn take_value_removes() {
    let mut map = HashMap::from([("a".to_string(), vec![1, 2])]);
    assert_eq!(take_value(&mut map, "a"), Some(vec![1, 2]));
    assert_eq!(take_value(&mut map, "a"), None);
    assert!(map.is_empty());
}

#[test]
fn take_entry_returns_the_stored_key() {
    let mut map = owned(&[("key", 7), ("other", 1)]);
    assert_eq!(take_entry(&mut map, "key"), Some(("key".to_string(), 7)));
    assert_eq!(take_entry(&mut map, "key"), None);
    assert_eq!(map.len(), 1);
}

#[test]
fn drop_below_retains_the_rest() {
    let mut map = owned(&[("a", 1), ("b", 5), ("c", 3), ("d", 9)]);
    drop_below(&mut map, 3);
    assert_eq!(map, owned(&[("b", 5), ("c", 3), ("d", 9)]), "3 itself stays");
    drop_below(&mut map, 100);
    assert!(map.is_empty());
}

// ── Building ──

#[test]
fn roman_values_has_all_seven() {
    let r = roman_values();
    assert_eq!(r.len(), 7);
    assert_eq!(r[&'I'], 1);
    assert_eq!(r[&'L'], 50);
    assert_eq!(r[&'M'], 1000);
    assert_eq!(r.values().sum::<u32>(), 1666);
}

#[test]
fn invert_swaps_keys_and_values() {
    let map = HashMap::from([("one", 1), ("two", 2)]);
    assert_eq!(invert(&map), HashMap::from([(1, "one"), (2, "two")]));
    let empty: HashMap<u8, char> = HashMap::new();
    assert!(invert(&empty).is_empty());
}

// ── Deterministic output ──

#[test]
fn sorted_keys_are_ascending() {
    let map = HashMap::from([(30, 'c'), (10, 'a'), (20, 'b')]);
    assert_eq!(sorted_keys(&map), [10, 20, 30]);
    let words = owned(&[("pear", 1), ("apple", 2), ("fig", 3)]);
    assert_eq!(sorted_keys(&words), ["apple", "fig", "pear"]);
}

#[test]
fn ranked_breaks_ties_alphabetically() {
    let counts = HashMap::from([("b", 2), ("a", 2), ("c", 5), ("d", 1)]);
    assert_eq!(ranked(&counts), [("c", 5), ("a", 2), ("b", 2), ("d", 1)]);
    assert!(ranked(&HashMap::new()).is_empty());
}

// ── HashSet ──

#[test]
fn in_both_intersects_sorted() {
    assert_eq!(in_both(&set(&[4, 1, 2, 3]), &set(&[5, 4, 3])), [3, 4]);
    assert_eq!(in_both(&set(&[2, 1]), &set(&[])), Vec::<i32>::new());
}

#[test]
fn in_either_unions_sorted() {
    assert_eq!(in_either(&set(&[4, 1, 2, 3]), &set(&[5, 4, 3])), [1, 2, 3, 4, 5]);
    assert_eq!(in_either(&set(&[2, 1]), &set(&[])), [1, 2]);
}

#[test]
fn only_in_first_is_not_symmetric() {
    let a = set(&[1, 2, 3, 4]);
    let b = set(&[3, 4, 5]);
    assert_eq!(only_in_first(&a, &b), [1, 2]);
    assert_eq!(only_in_first(&b, &a), [5], "difference isn't symmetric");
    assert_eq!(only_in_first(&set(&[]), &a), Vec::<i32>::new());
}

#[test]
fn in_exactly_one_drops_the_overlap() {
    assert_eq!(in_exactly_one(&set(&[1, 2, 3, 4]), &set(&[3, 4, 5])), [1, 2, 5]);
    assert_eq!(in_exactly_one(&set(&[]), &set(&[2, 1])), [1, 2]);
    assert_eq!(in_exactly_one(&set(&[7]), &set(&[7])), Vec::<i32>::new());
}

#[test]
fn no_overlap_checks_disjointness() {
    assert!(no_overlap(&set(&[1, 2]), &set(&[3, 4])));
    assert!(!no_overlap(&set(&[1, 2, 3]), &set(&[3, 4])));
    assert!(no_overlap(&set(&[]), &set(&[])), "empty sets share nothing");
    assert!(no_overlap(&set(&[5]), &set(&[])));
}

#[test]
fn spellable_checks_subset() {
    assert!(spellable("tattoo", "toa"));
    assert!(!spellable("rust", "trs"));
    assert!(spellable("", "abc"), "the empty set is a subset of anything");
    assert!(spellable("été", "tée"));
}
