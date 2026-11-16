// Tests for Ordering and Hashing — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeSet, HashMap};

fn hash_of<T: Hash>(x: &T) -> u64 {
    let mut h = DefaultHasher::new();
    x.hash(&mut h);
    h.finish()
}

fn ci(s: &str) -> CaseInsensitive {
    CaseInsensitive(s.to_string())
}

fn v(s: &str) -> Version {
    s.parse().expect("test versions are valid")
}

// ── CaseInsensitive ──

#[test]
fn case_insensitive_eq() {
    assert_eq!(ci("Ann"), ci("aNN"));
    assert_ne!(ci("Ann"), ci("Anne"));
    assert_ne!(ci("É"), ci("é"), "only ASCII case is folded");
}

#[test]
fn case_insensitive_hash_agrees_with_eq() {
    assert_eq!(hash_of(&ci("Hello World")), hash_of(&ci("hELLO wORLD")));
    let set: HashSet<CaseInsensitive> = ["Rust", "RUST", "rust", "Go"].map(ci).into_iter().collect();
    assert_eq!(set.len(), 2);
    assert!(set.contains(&ci("rUsT")));
}

#[test]
fn case_insensitive_ord() {
    assert!(ci("apple") < ci("Banana"), "derived/str order would put 'B' first");
    assert_eq!(ci("ZED").cmp(&ci("zed")), Ordering::Equal);
    assert!(ci("ab") < ci("ABC"));
    assert_ne!(ci("É").cmp(&ci("é")), Ordering::Equal, "only ASCII case is folded, as in eq");
    let mut names = [ci("bob"), ci("Alice"), ci("carol"), ci("Bea")];
    names.sort();
    let sorted: Vec<&str> = names.iter().map(|n| n.0.as_str()).collect();
    assert_eq!(sorted, ["Alice", "Bea", "bob", "carol"]);
}

#[test]
fn distinct_names_ignores_case() {
    assert_eq!(distinct_names(&["Ann", "ann", "Bob", "ANN", "bob"]), 2);
    assert_eq!(distinct_names(&[]), 0);
    assert_eq!(distinct_names(&["x"]), 1);
}

// ── Version ──

#[test]
fn version_trimmed_drops_trailing_zeros() {
    assert_eq!(v("1.2.0.0").trimmed(), [1, 2]);
    assert_eq!(v("1.0.2").trimmed(), [1, 0, 2], "inner zeros stay");
    assert_eq!(v("0").trimmed(), [] as [u32; 0]);
    assert_eq!(v("3").trimmed(), [3]);
}

#[test]
fn version_compares_numerically_not_as_text() {
    assert!(v("1.10") > v("1.9"), "\"1.10\" < \"1.9\" as strings");
    assert!(v("2") > v("1.99.99"));
    assert!(v("1.2.1") > v("1.2"));
    assert!(v("0.9") < v("1"));
}

#[test]
fn version_trailing_zeros_are_equal() {
    assert_eq!(v("1.2"), v("1.2.0"));
    assert_eq!(v("1.2.0.0").cmp(&v("1.2")), Ordering::Equal);
    assert_ne!(v("1.2"), v("1.2.1"));
}

#[test]
fn version_hash_agrees_with_eq() {
    assert_eq!(hash_of(&v("1.2")), hash_of(&v("1.2.0.0")));
    let set: HashSet<Version> = ["1", "1.0", "1.0.0", "1.1", "1.01"].map(v).into_iter().collect();
    assert_eq!(set.len(), 2, "1 == 1.0 == 1.0.0, and 1.01 parses as 1.1");
    let mut downloads: HashMap<Version, u32> = HashMap::new();
    *downloads.entry(v("2.0")).or_default() += 1;
    *downloads.entry(v("2")).or_default() += 1;
    assert_eq!(downloads.len(), 1);
    assert_eq!(downloads[&v("2.0.0")], 2);
}

#[test]
fn newest_picks_the_max() {
    let (a, b) = (v("1.9"), v("1.10"));
    assert_eq!(newest(&a, &b).parts, [1, 10]);
    assert_eq!(newest(&b, &a).parts, [1, 10]);
    let (c, d) = (v("2"), v("2.0"));
    assert_eq!(newest(&c, &d).parts, [2, 0], "on a tie, max returns the second");
}

#[test]
fn strictly_ascending_uses_version_order() {
    assert!(strictly_ascending(&["1.2", "1.9", "1.10", "2"].map(v)));
    assert!(!strictly_ascending(&["1.2", "1.2.0"].map(v)), "equal isn't strictly newer");
    assert!(!strictly_ascending(&["1.10", "1.9"].map(v)));
    assert!(strictly_ascending(&[]));
    assert!(strictly_ascending(&[v("5")]));
}

// ── Player and multi-key orders ──

fn p(name: &str, score: u32, time: u32) -> Player {
    Player { name: name.to_string(), score, time }
}

#[test]
fn player_order_score_then_time_then_name() {
    assert!(p("a", 90, 50) < p("b", 80, 10), "higher score first");
    assert!(p("b", 90, 10) < p("a", 90, 50), "then lower time");
    assert!(p("amy", 90, 10) < p("bob", 90, 10), "then name");
    assert_eq!(p("x", 1, 1).cmp(&p("x", 1, 1)), Ordering::Equal);
}

#[test]
fn player_sort_and_btreeset() {
    let mut board = [p("cat", 50, 30), p("ann", 70, 40), p("bob", 70, 20), p("dan", 50, 30)];
    board.sort();
    let names: Vec<&str> = board.iter().map(|pl| pl.name.as_str()).collect();
    assert_eq!(names, ["bob", "ann", "cat", "dan"]);
    let set: BTreeSet<Player> = board.into_iter().collect();
    assert_eq!(set.first().map(|pl| pl.name.as_str()), Some("bob"));
}

#[test]
fn by_len_then_alpha_orders() {
    assert_eq!(by_len_then_alpha("zz", "aaa"), Ordering::Less);
    assert_eq!(by_len_then_alpha("abc", "abd"), Ordering::Less);
    assert_eq!(by_len_then_alpha("same", "same"), Ordering::Equal);
    let mut words = ["pear", "fig", "apple", "kiwi", "date"];
    words.sort_by(|a, b| by_len_then_alpha(a, b));
    assert_eq!(words, ["fig", "date", "kiwi", "pear", "apple"]);
}

#[test]
fn most_urgent_uses_declaration_order() {
    use Priority::*;
    assert_eq!(most_urgent(&[Low, Critical, Medium]), Some(Critical));
    assert_eq!(most_urgent(&[Medium, Low]), Some(Medium));
    assert_eq!(most_urgent(&[]), None);
}

#[test]
fn ranked_by_score_checks_descending_scores_only() {
    assert!(ranked_by_score(&[p("a", 90, 50), p("b", 80, 10), p("c", 80, 99), p("d", 5, 1)]));
    assert!(
        ranked_by_score(&[p("z", 70, 90), p("a", 70, 10)]),
        "equal scores in any order, time and name ignored"
    );
    assert!(!ranked_by_score(&[p("a", 10, 1), p("b", 20, 1)]), "ascending isn't ranked");
    assert!(!ranked_by_score(&[p("a", 30, 1), p("b", 10, 1), p("c", 20, 1)]));
    assert!(ranked_by_score(&[]));
    assert!(ranked_by_score(&[p("solo", 1, 1)]));
}

// ── Reverse and *_by_key ──

#[test]
fn sort_longest_first_is_stable() {
    let mut words = ["a", "ccc", "bb", "ddd", "e"];
    sort_longest_first(&mut words);
    assert_eq!(words, ["ccc", "ddd", "bb", "a", "e"]);
}

#[test]
fn longest_returns_first_on_ties() {
    assert_eq!(longest(&["ab", "xyz", "q", "uvw"]), Some("xyz"), "max_by_key would give \"uvw\"");
    assert_eq!(longest(&["one"]), Some("one"));
    assert_eq!(longest(&[]), None);
}

#[test]
fn closer_to_zero_prefers_first_on_ties() {
    assert_eq!(closer_to_zero(-5, 2), 2);
    assert_eq!(closer_to_zero(4, -7), 4);
    assert_eq!(closer_to_zero(-3, 3), -3, "tie: the first argument");
    assert_eq!(closer_to_zero(3, -3), 3, "tie: the first argument");
    assert_eq!(closer_to_zero(i32::MIN, i32::MAX), i32::MAX, "abs(i32::MIN) overflows; unsigned_abs doesn't");
    assert_eq!(closer_to_zero(0, i32::MIN), 0);
}

// ── Floats ──

#[test]
fn max_f64_total_order() {
    assert_eq!(max_f64(&[1.5, -2.0, 0.25]), Some(1.5));
    assert_eq!(max_f64(&[f64::NEG_INFINITY, f64::INFINITY]), Some(f64::INFINITY));
    assert!(max_f64(&[1.0, f64::NAN, 3.0]).is_some_and(f64::is_nan), "NaN is above everything");
    assert_eq!(max_f64(&[]), None);
}

#[test]
fn sort_floats_total_order() {
    let mut xs = [3.0, f64::NAN, -0.0, 0.0, -1.5, f64::NEG_INFINITY];
    sort_floats(&mut xs);
    assert_eq!(xs[0], f64::NEG_INFINITY);
    assert_eq!(xs[1], -1.5);
    assert!(xs[2] == 0.0 && xs[2].is_sign_negative(), "-0.0 before 0.0");
    assert!(xs[3] == 0.0 && xs[3].is_sign_positive());
    assert_eq!(xs[4], 3.0);
    assert!(xs[5].is_nan());
}

#[test]
fn score_is_totally_ordered() {
    assert!(Score(1.0) < Score(2.0));
    assert_eq!(Score(f64::NAN), Score(f64::NAN), "NaN equals itself under total_cmp");
    assert_ne!(Score(0.0), Score(-0.0));
    let set: BTreeSet<Score> = [2.5, 1.0, 2.5, f64::NAN].map(Score).into_iter().collect();
    assert_eq!(set.len(), 3);
    assert_eq!(set.first().map(|s| s.0), Some(1.0));
}
