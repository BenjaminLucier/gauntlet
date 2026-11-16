// Tests for mem::take, replace and swap — shared by template.rs, solution.rs and your attempt.
use super::*;

fn list(vals: &[i32]) -> Option<Box<Node>> {
    vals.iter().rev().fold(None, |next, &val| Some(Box::new(Node { val, next })))
}

// ── Taking and replacing ──

#[test]
fn take_string_leaves_empty() {
    let mut s = String::from("hello");
    assert_eq!(take_string(&mut s), "hello");
    assert_eq!(s, "");
    assert_eq!(take_string(&mut s), "");
}

#[test]
fn replace_name_returns_old() {
    let mut name = String::from("Ferris");
    assert_eq!(replace_name(&mut name, "Corro".into()), "Ferris");
    assert_eq!(name, "Corro");
}

#[test]
fn flush_moves_and_skips_empty() {
    let mut buf = b"abc".to_vec();
    let mut out = Vec::new();
    flush(&mut buf, &mut out);
    assert!(buf.is_empty());
    flush(&mut buf, &mut out);
    assert_eq!(out, vec![b"abc".to_vec()], "an empty buffer pushes nothing");
    buf.extend_from_slice(b"de");
    flush(&mut buf, &mut out);
    assert_eq!(out, vec![b"abc".to_vec(), b"de".to_vec()]);
}

// ── Swapping ──

#[test]
fn swap_buffers_exchanges_without_copying() {
    let mut front = vec![1u8, 2, 3];
    let mut back = Vec::with_capacity(64);
    back.push(9u8);
    let back_ptr = back.as_ptr();
    swap_buffers(&mut front, &mut back);
    assert_eq!(front, vec![9]);
    assert_eq!(back, vec![1, 2, 3]);
    assert_eq!(front.as_ptr(), back_ptr, "the allocation moved, not the bytes");
}

#[test]
fn rotate_three_rotates_left() {
    let (mut a, mut b, mut c) = ("a".to_string(), "b".to_string(), "c".to_string());
    rotate_three(&mut a, &mut b, &mut c);
    assert_eq!((a.as_str(), b.as_str(), c.as_str()), ("b", "c", "a"));
}

#[test]
fn normalize_swaps_fields_when_out_of_order() {
    let span = |s: &str, e: &str| Span { start: s.to_string(), end: e.to_string() };
    let mut backwards = span("m", "c");
    normalize(&mut backwards);
    assert_eq!(backwards, span("c", "m"));
    let mut ordered = span("a", "z");
    normalize(&mut ordered);
    assert_eq!(ordered, span("a", "z"), "already ordered: unchanged");
    let mut same = span("k", "k");
    normalize(&mut same);
    assert_eq!(same, span("k", "k"));
}

#[test]
fn normalize_moves_the_strings_not_their_bytes() {
    let mut s = Span { start: "zebra".to_string(), end: "ant".to_string() };
    let zebra_ptr = s.start.as_ptr();
    normalize(&mut s);
    assert_eq!(s.end.as_ptr(), zebra_ptr, "the String moved; nothing was copied");
}

// ── Option slots ──

#[test]
fn pop_front_walks_the_list() {
    let mut head = list(&[1, 2, 3]);
    assert_eq!(pop_front(&mut head), Some(1));
    assert_eq!(head, list(&[2, 3]));
    assert_eq!(pop_front(&mut head), Some(2));
    assert_eq!(pop_front(&mut head), Some(3));
    assert_eq!(pop_front(&mut head), None);
    assert_eq!(head, None);
}

#[test]
fn take_if_long_only_takes_long_strings() {
    let mut slot = Some("hi".to_string());
    assert_eq!(take_if_long(&mut slot, 3), None);
    assert_eq!(slot.as_deref(), Some("hi"), "a short string stays put");
    assert_eq!(take_if_long(&mut slot, 2).as_deref(), Some("hi"));
    assert_eq!(slot, None);
    assert_eq!(take_if_long(&mut slot, 0), None);
}

// ── Draining ──

#[test]
fn drain_keep_capacity_keeps_the_allocation() {
    let mut v: Vec<String> = Vec::with_capacity(16);
    v.extend(["x".to_string(), "y".to_string()]);
    let out = drain_keep_capacity(&mut v);
    assert_eq!(out, ["x", "y"]);
    assert!(v.is_empty());
    assert!(v.capacity() >= 16, "drain keeps the capacity; mem::take would drop it");
}

#[test]
fn move_all_appends_and_empties_the_source() {
    let mut from: Vec<String> = Vec::with_capacity(8);
    from.extend(["c".to_string(), "d".to_string()]);
    let mut to = vec!["a".to_string(), "b".to_string()];
    let c_ptr = from[0].as_ptr();
    move_all(&mut from, &mut to);
    assert_eq!(to, ["a", "b", "c", "d"]);
    assert!(from.is_empty());
    assert!(from.capacity() >= 8, "the source keeps its buffer");
    assert_eq!(to[2].as_ptr(), c_ptr, "the Strings moved, not cloned");
    move_all(&mut from, &mut to);
    assert_eq!(to.len(), 4, "appending an empty Vec changes nothing");
}

// ── A state machine ──

#[test]
fn client_happy_path() {
    let mut c = Client::new();
    assert_eq!(c.state, Conn::Idle);
    c.handle(Event::Dial);
    assert_eq!(c.state, Conn::Connecting { attempts: 1 });
    c.handle(Event::Accept("s-42".into()));
    assert_eq!(c.state, Conn::Connected { session: "s-42".into() });
    c.handle(Event::Hangup("bye".into()));
    assert_eq!(c.state, Conn::Closed { reason: "bye".into() });
    assert_eq!(c.log, ["s-42"], "the finished session is logged");
}

#[test]
fn client_gives_up_after_max_attempts() {
    let mut c = Client::new();
    c.handle(Event::Dial);
    c.handle(Event::Fail);
    assert_eq!(c.state, Conn::Connecting { attempts: 2 });
    c.handle(Event::Fail);
    assert_eq!(c.state, Conn::Connecting { attempts: 3 });
    c.handle(Event::Fail);
    assert_eq!(c.state, Conn::Closed { reason: "gave up".into() });
}

#[test]
fn client_ignores_events_that_dont_apply() {
    let mut c = Client::new();
    c.handle(Event::Accept("early".into()));
    assert_eq!(c.state, Conn::Idle);
    c.handle(Event::Dial);
    c.handle(Event::Accept("s".into()));
    c.handle(Event::Dial);
    assert_eq!(c.state, Conn::Connected { session: "s".into() }, "the state is put back, not lost");
    c.handle(Event::Hangup("done".into()));
    c.handle(Event::Dial);
    assert_eq!(c.state, Conn::Closed { reason: "done".into() });
}

#[test]
fn client_take_log_empties_it() {
    let mut c = Client::new();
    for s in ["a", "b"] {
        c.handle(Event::Dial);
        c.handle(Event::Accept(s.into()));
        c.handle(Event::Hangup("x".into()));
        c.state = Conn::Idle;
    }
    assert_eq!(c.take_log(), ["a", "b"]);
    assert!(c.log.is_empty());
}

// ── Slices ──

#[test]
fn pair_mut_gives_two_live_references() {
    let mut v = [10, 20, 30, 40];
    let (a, b) = pair_mut(&mut v, 1, 3);
    std::mem::swap(a, b);
    *a += 1;
    assert_eq!(v, [10, 41, 30, 20]);
    let (a, b) = pair_mut(&mut v, 0, 1);
    *b += *a;
    assert_eq!(v, [10, 51, 30, 20]);
}

#[test]
#[should_panic(expected = "pair_mut needs i < j")]
fn pair_mut_rejects_equal_indices() {
    let mut v = [1, 2, 3];
    pair_mut(&mut v, 1, 1);
}

#[test]
#[should_panic(expected = "pair_mut needs i < j")]
fn pair_mut_rejects_reversed_indices() {
    let mut v = [1, 2, 3];
    pair_mut(&mut v, 2, 0);
}

#[test]
fn swap_contents_swaps_slices() {
    let mut v = [1, 2, 3, 4, 5, 6];
    let (left, right) = v.split_at_mut(3);
    swap_contents(left, right);
    assert_eq!(v, [4, 5, 6, 1, 2, 3]);
}

// ── Sizes and discriminants ──

#[test]
fn option_box_is_pointer_sized() {
    let ptr = std::mem::size_of::<usize>();
    assert_eq!(box_sizes(), (ptr, ptr), "None uses the null niche: no extra tag");
}

#[test]
fn option_sizes_show_the_niche() {
    let (plain, nonzero, reference) = option_sizes();
    assert_eq!(plain, 8, "u32 has no spare bit pattern, so Option adds a tag (padded)");
    assert_eq!(nonzero, 4, "0 is the niche");
    assert_eq!(reference, std::mem::size_of::<usize>());
}

#[test]
fn same_variant_ignores_fields() {
    let a = Conn::Connecting { attempts: 1 };
    let b = Conn::Connecting { attempts: 9 };
    assert!(same_variant(&a, &b));
    assert!(!same_variant(&a, &Conn::Idle));
    assert!(same_variant(&Conn::Closed { reason: "x".into() }, &Conn::Closed { reason: "y".into() }));
}

#[test]
fn count_variants_dedups_by_variant() {
    let states = [
        Conn::Idle,
        Conn::Connecting { attempts: 1 },
        Conn::Connecting { attempts: 2 },
        Conn::Idle,
        Conn::Closed { reason: "r".into() },
    ];
    assert_eq!(count_variants(&states), 3);
    assert_eq!(count_variants(&[]), 0);
}
