// Tests for BTreeMap and Ranges — shared by template.rs, solution.rs and your attempt.
use super::*;

fn set(xs: &[i32]) -> BTreeSet<i32> {
    xs.iter().copied().collect()
}

fn log(entries: &[(u64, &str)]) -> BTreeMap<u64, String> {
    entries.iter().map(|&(t, e)| (t, e.to_string())).collect()
}

// ── Ordered iteration ──

#[test]
fn sorted_unique_dedups_and_sorts() {
    assert_eq!(sorted_unique(&[3, -1, 3, 2, -1]), [-1, 2, 3]);
    assert_eq!(sorted_unique(&[]), Vec::<i32>::new());
}

#[test]
fn values_by_key_follows_key_order() {
    let map: BTreeMap<String, u32> = [("mia", 3), ("ana", 9), ("zoe", 1), ("lee", 5)]
        .iter()
        .map(|&(n, s)| (n.to_string(), s))
        .collect();
    assert_eq!(values_by_key(map), [9, 5, 3, 1], "ana, lee, mia, zoe");
    assert_eq!(values_by_key(BTreeMap::new()), Vec::<u32>::new());
}

#[test]
fn descending_keys_reverses() {
    let map = BTreeMap::from([(2, "b"), (-5, "z"), (9, "q")]);
    assert_eq!(descending_keys(&map), [9, 2, -5]);
}

#[test]
fn nth_smallest_counts_from_zero() {
    let s = set(&[50, 10, 30]);
    assert_eq!(nth_smallest(&s, 0), Some(10));
    assert_eq!(nth_smallest(&s, 2), Some(50));
    assert_eq!(nth_smallest(&s, 3), None);
}

// ── Ranges ──

#[test]
fn values_between_is_inclusive() {
    let map = BTreeMap::from([(1, "a"), (3, "c"), (5, "e"), (7, "g")]);
    assert_eq!(values_between(&map, 3, 5), ["c", "e"]);
    assert_eq!(values_between(&map, 2, 6), ["c", "e"]);
    assert_eq!(values_between(&map, 7, 7), ["g"]);
    assert_eq!(values_between(&map, 8, 100), Vec::<&str>::new());
}

#[test]
fn values_between_backwards_range_does_not_panic() {
    let map = BTreeMap::from([(1, "a"), (3, "c")]);
    assert_eq!(values_between(&map, 3, 1), Vec::<&str>::new());
}

#[test]
fn range_sum_is_half_open() {
    let map = BTreeMap::from([(-2, 10), (0, 1), (4, 100), (5, 1000)]);
    assert_eq!(range_sum(&map, -2, 5), 111, "5 is excluded");
    assert_eq!(range_sum(&map, 0, 0), 0);
    assert_eq!(range_sum(&map, 5, -2), 0, "backwards is empty, not a panic");
    assert_eq!(range_sum(&map, i32::MIN, i32::MAX), 1111);
}

#[test]
fn raise_between_is_inclusive() {
    let mut map = BTreeMap::from([(1, 10), (3, 30), (5, 50), (7, 70)]);
    raise_between(&mut map, 3, 5, -1);
    assert_eq!(map, BTreeMap::from([(1, 10), (3, 29), (5, 49), (7, 70)]));
    raise_between(&mut map, 6, u32::MAX, 100);
    assert_eq!(map, BTreeMap::from([(1, 10), (3, 29), (5, 49), (7, 170)]));
    raise_between(&mut map, 8, 100, 1);
    assert_eq!(map[&7], 170, "no key in range: nothing changes");
}

#[test]
fn raise_between_backwards_range_does_not_panic() {
    let mut map = BTreeMap::from([(1, 10), (3, 30)]);
    raise_between(&mut map, 3, 1, 5);
    assert_eq!(map, BTreeMap::from([(1, 10), (3, 30)]));
}

#[test]
fn count_below_is_strict() {
    let s = set(&[1, 4, 4, 6, 9]);
    assert_eq!(count_below(&s, 6), 2);
    assert_eq!(count_below(&s, 7), 3);
    assert_eq!(count_below(&s, 1), 0);
    assert_eq!(count_below(&s, 100), 4);
}

#[test]
fn events_from_starts_at_t() {
    let l = log(&[(5, "boot"), (10, "login"), (12, "click"), (20, "logout")]);
    assert_eq!(events_from(&l, 10, 2), ["login", "click"]);
    assert_eq!(events_from(&l, 11, 10), ["click", "logout"]);
    assert_eq!(events_from(&l, 0, 0), Vec::<&str>::new());
    assert_eq!(events_from(&l, 21, 3), Vec::<&str>::new());
}

#[test]
fn with_prefix_uses_str_range() {
    let words: BTreeSet<String> = ["car", "cart", "carbon", "cat", "ca", "dog", "bus"]
        .iter()
        .map(|w| w.to_string())
        .collect();
    assert_eq!(with_prefix(&words, "car"), ["car", "carbon", "cart"]);
    assert_eq!(with_prefix(&words, "ca"), ["ca", "car", "carbon", "cart", "cat"]);
    assert_eq!(with_prefix(&words, "x"), Vec::<&str>::new());
    assert_eq!(with_prefix(&words, "").len(), 7);
}

// ── Floor and ceiling ──

/// The even numbers `0, 2, …, 199_998`: 100 000 elements.
fn evens() -> BTreeSet<i32> {
    (0..100_000).map(|i| i * 2).collect()
}

/// 200 000 pseudo-random queries in `-10..200_010`.
fn queries() -> impl Iterator<Item = i32> {
    let mut state = 0x9e37_79b9_u32;
    (0..200_000).map(move |_| {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        i32::try_from(state % 200_020).expect("fits") - 10
    })
}

#[test]
fn floor_is_inclusive() {
    let s = set(&[10, 20, 30]);
    assert_eq!(floor(&s, 25), Some(20));
    assert_eq!(floor(&s, 20), Some(20), "inclusive");
    assert_eq!(floor(&s, 1_000), Some(30));
    assert_eq!(floor(&s, 5), None);
}

#[test]
fn floor_at_extremes() {
    assert_eq!(floor(&set(&[i32::MIN, i32::MAX]), i32::MIN), Some(i32::MIN));
    assert_eq!(floor(&set(&[i32::MAX]), i32::MAX - 1), None);
    assert_eq!(floor(&set(&[]), 0), None);
}

#[test]
fn floor_answers_200k_queries_without_scanning() {
    let s = evens();
    for q in queries() {
        let expected = (q >= 0).then(|| (q - q.rem_euclid(2)).min(199_998));
        assert_eq!(floor(&s, q), expected, "floor of {q}");
    }
}

#[test]
fn ceiling_is_inclusive() {
    let s = set(&[10, 20, 30]);
    assert_eq!(ceiling(&s, 25), Some(30));
    assert_eq!(ceiling(&s, 20), Some(20), "inclusive");
    assert_eq!(ceiling(&s, -1_000), Some(10));
    assert_eq!(ceiling(&s, 31), None);
}

#[test]
fn ceiling_at_extremes() {
    assert_eq!(ceiling(&set(&[i32::MIN, i32::MAX]), i32::MAX), Some(i32::MAX));
    assert_eq!(ceiling(&set(&[i32::MIN]), i32::MIN + 1), None);
    assert_eq!(ceiling(&set(&[]), 0), None);
}

#[test]
fn ceiling_answers_200k_queries_without_scanning() {
    let s = evens();
    for q in queries() {
        let expected = (q <= 199_998).then(|| (q + q.rem_euclid(2)).max(0));
        assert_eq!(ceiling(&s, q), expected, "ceiling of {q}");
    }
}

#[test]
fn strictly_after_skips_equal() {
    let s = set(&[1, 3, 5]);
    assert_eq!(strictly_after(&s, 3), Some(5));
    assert_eq!(strictly_after(&s, 2), Some(3));
    assert_eq!(strictly_after(&s, 5), None);
    assert_eq!(strictly_after(&set(&[i32::MAX]), i32::MAX), None);
}


// ── Ends ──

#[test]
fn first_and_last_names() {
    let scores: BTreeMap<String, u32> = [("mia", 3), ("ana", 9), ("zoe", 1), ("lee", 5)]
        .iter()
        .map(|&(n, s)| (n.to_string(), s))
        .collect();
    assert_eq!(first_and_last(&scores), Some(("ana", "zoe")));
    assert_eq!(first_and_last(&BTreeMap::new()), None);
}

#[test]
fn pop_smallest_removes_from_the_front() {
    let mut s = set(&[5, 1, 4, 2]);
    assert_eq!(pop_smallest(&mut s, 2), [1, 2]);
    assert_eq!(s, set(&[4, 5]));
    assert_eq!(pop_smallest(&mut s, 10), [4, 5]);
    assert!(s.is_empty());
    assert_eq!(pop_smallest(&mut s, 0), Vec::<i32>::new());
}

#[test]
fn pop_smallest_zero_takes_nothing() {
    let mut s = set(&[1, 2]);
    assert_eq!(pop_smallest(&mut s, 0), Vec::<i32>::new());
    assert_eq!(s.len(), 2, "k = 0 must not pop anything");
}

#[test]
fn sell_cheapest_decrements_then_removes() {
    let mut stock = BTreeMap::from([(30, 1), (10, 2), (20, 1)]);
    assert_eq!(sell_cheapest(&mut stock), Some(10));
    assert_eq!(stock, BTreeMap::from([(10, 1), (20, 1), (30, 1)]));
    assert_eq!(sell_cheapest(&mut stock), Some(10));
    assert_eq!(stock, BTreeMap::from([(20, 1), (30, 1)]), "a count of 0 is removed");
    assert_eq!(sell_cheapest(&mut stock), Some(20));
    assert_eq!(sell_cheapest(&mut stock), Some(30));
    assert!(stock.is_empty());
}

#[test]
fn sell_cheapest_on_empty_is_none() {
    let mut stock = BTreeMap::new();
    assert_eq!(sell_cheapest(&mut stock), None);
    assert!(stock.is_empty());
}

#[test]
fn pop_latest_takes_the_last() {
    let mut l = log(&[(1, "a"), (9, "z"), (5, "m")]);
    assert_eq!(pop_latest(&mut l).as_deref(), Some("z"));
    assert_eq!(pop_latest(&mut l).as_deref(), Some("m"));
    assert_eq!(pop_latest(&mut l).as_deref(), Some("a"));
    assert_eq!(pop_latest(&mut l), None);
}

// ── Splitting and editing ──

#[test]
fn split_at_key_moves_the_upper_part() {
    let mut map = BTreeMap::from([(1, 'a'), (3, 'c'), (5, 'e')]);
    let upper = split_at_key(&mut map, 3);
    assert_eq!(map, BTreeMap::from([(1, 'a')]));
    assert_eq!(upper, BTreeMap::from([(3, 'c'), (5, 'e')]), "the key itself goes up");
    let none = split_at_key(&mut map, 100);
    assert!(none.is_empty());
    assert_eq!(map.len(), 1);
}

#[test]
fn keep_even_keys_retains() {
    let mut map = BTreeMap::from([(-2, 'a'), (-1, 'b'), (0, 'c'), (3, 'd'), (4, 'e')]);
    keep_even_keys(&mut map);
    assert_eq!(map, BTreeMap::from([(-2, 'a'), (0, 'c'), (4, 'e')]));
}

#[test]
fn absorb_empties_the_other() {
    let mut a = set(&[1, 5]);
    let mut b = set(&[2, 5, 9]);
    absorb(&mut a, &mut b);
    assert_eq!(a, set(&[1, 2, 5, 9]));
    assert!(b.is_empty());
}
