// Tests for Fn, FnMut and FnOnce — shared by template.rs, solution.rs and your attempt.
use super::*;
use std::cell::Cell;

fn triple(x: i32) -> i32 {
    x * 3
}

// ── FnOnce ──

#[test]
fn apply_takes_any_closure() {
    assert_eq!(apply(|x| x + 1, 4), 5);
    assert_eq!(apply(triple, 4), 12);
    assert_eq!(apply(i32::abs, -7), 7);
    let owned = vec![1, 2, 3];
    // `into_iter` consumes `owned`: this closure is only FnOnce.
    assert_eq!(apply(move |x| owned.into_iter().sum::<i32>() + x, 10), 16);
}

#[test]
fn run_once_accepts_consuming_closures() {
    let s = String::from("owned");
    let v = run_once(move || s.into_bytes());
    assert_eq!(v, b"owned");
    assert_eq!(run_once(|| 42), 42);
}

#[test]
fn call_or_default_is_lazy() {
    let calls = Cell::new(0);
    let make = || {
        calls.set(calls.get() + 1);
        String::from("made")
    };
    assert_eq!(call_or_default(Some(make), "default".to_string()), "made");
    assert_eq!(call_or_default(None::<fn() -> String>, "default".to_string()), "default");
    assert_eq!(calls.get(), 1);
}

#[test]
fn fire_once_runs_the_callback_once() {
    let farewell = String::from("bye");
    let mut slot: Option<Box<dyn FnOnce() -> String>> = Some(Box::new(move || farewell + "!"));
    assert_eq!(fire_once(&mut slot), Some("bye!".to_string()));
    assert!(slot.is_none(), "the callback is gone from the slot");
    assert_eq!(fire_once(&mut slot), None, "and can't run again");
}

// ── FnMut ──

#[test]
fn apply_twice_composes_with_itself() {
    assert_eq!(apply_twice(|x| x * x, 3), 81);
    assert_eq!(apply_twice(triple, 1), 9);
    let mut calls = Vec::new();
    assert_eq!(apply_twice(|x| { calls.push(x); x + 1 }, 5), 7, "a closure that mutates its captures");
    assert_eq!(calls, [5, 6]);
}

#[test]
fn apply_n_repeats() {
    assert_eq!(apply_n(|x| x * 2, 10, 1), 1024);
    assert_eq!(apply_n(|x| x + 5, 0, 7), 7, "zero times is the identity");
    let mut step = 0;
    let stepped = apply_n(|x| { step += 1; x + step }, 3, 0);
    assert_eq!((stepped, step), (6, 3), "0 + 1 + 2 + 3, and FnMut state survives between calls");
}

#[test]
fn count_where_counts() {
    let xs = [1, -2, 3, -4, 5, 0];
    assert_eq!(count_where(&xs, |&x| x > 0), 3);
    assert_eq!(count_where(&[], |_| true), 0);
    let mut seen = Vec::new();
    assert_eq!(count_where(&xs, |&x| { seen.push(x); x % 2 == 0 }), 3);
    assert_eq!(seen, xs, "pred sees every element once, in order");
}

#[test]
fn best_by_prefers_the_earliest_on_ties() {
    let words = ["aa", "bbb", "ccc", "d"];
    assert_eq!(best_by(&words, str::len), Some("bbb"), "max_by_key alone keeps the last");
    assert_eq!(best_by(&words, |w| w.matches('d').count()), Some("d"));
    assert_eq!(best_by(&[], str::len), None);
    let mut scored = 0;
    assert_eq!(best_by(&words, |w| { scored += 1; w.len() }), Some("bbb"));
    assert_eq!(scored, 4, "each word scored once");
}

#[test]
fn call_n_mutates_captures() {
    let mut count = 0;
    call_n(5, || count += 1);
    assert_eq!(count, 5);
    let mut log = Vec::new();
    call_n(3, || log.push(log.len()));
    assert_eq!(log, [0, 1, 2]);
    call_n(0, || panic!("must not be called"));
}

#[test]
fn visit_sees_every_element_in_order() {
    let mut seen = Vec::new();
    visit(&[3, 1, 2], |x| seen.push(x));
    assert_eq!(seen, [3, 1, 2]);
    let mut total = 0;
    visit(&[], |x| total += x);
    assert_eq!(total, 0);
}

#[test]
fn running_totals_accumulates() {
    assert_eq!(running_totals(&[1, 2, 3, 4]), [1, 3, 6, 10]);
    assert_eq!(running_totals(&[5, -5, 5]), [5, 0, 5]);
    assert!(running_totals(&[]).is_empty());
}

#[test]
fn retry_stops_at_first_success() {
    let mut calls = 0;
    let r: Result<u32, &str> = retry(5, || {
        calls += 1;
        if calls < 3 { Err("not yet") } else { Ok(calls) }
    });
    assert_eq!(r, Ok(3));
    assert_eq!(calls, 3);
}

#[test]
fn retry_gives_up_with_the_last_error() {
    let mut calls = 0;
    let r: Result<(), u32> = retry(4, || {
        calls += 1;
        Err(calls)
    });
    assert_eq!(r, Err(4));
    let mut once = 0;
    let r: Result<(), ()> = retry(0, || {
        once += 1;
        Err(())
    });
    assert_eq!((r, once), (Err(()), 1), "always called at least once");
}

// ── Fn ──

#[test]
fn map_all_through_a_trait_object() {
    let k = 10;
    assert_eq!(map_all(&[1, 2, 3], &|x| x * k), [10, 20, 30]);
    assert_eq!(map_all(&[-1, 2], &i32::abs), [1, 2]);
    assert!(map_all(&[], &triple).is_empty());
}

#[test]
fn total_by_borrows_f() {
    let offset = 1;
    let shift = |x: i32| x + offset;
    assert_eq!(total_by(&[1, 2, 3], &shift), 9);
    assert_eq!(total_by(&[10], &shift), 11, "still ours to use");
    assert_eq!(total_by(&[], &triple), 0);
    assert_eq!(total_by(&[-1, 2], &i32::abs), 3);
}

// ── move ──

#[test]
fn greeters_own_their_names() {
    let gs = {
        let names = vec![String::from("Ferris"), String::from("Corro")];
        greeters(names)
    };
    assert_eq!(gs.len(), 2);
    assert_eq!(gs[0]("Hi"), "Hi, Ferris!");
    assert_eq!(gs[0]("Bye"), "Bye, Ferris!", "callable again");
    assert_eq!(gs[1]("Hi"), "Hi, Corro!");
    assert!(greeters(Vec::new()).is_empty());
}

// ── Functions as values ──

#[test]
fn lengths_in_bytes() {
    assert_eq!(lengths(&["a", "bb", "", "é"]), [1, 2, 0, 2]);
}

#[test]
fn abs_all_handles_negatives() {
    assert_eq!(abs_all(&[-3, 0, 4, -1]), [3, 0, 4, 1]);
}

#[test]
fn parse_all_keeps_failures_as_none() {
    assert_eq!(parse_all(&["1", "x", "-7", ""]), [Some(1), None, Some(-7), None]);
}

// ── Collections of closures ──

#[test]
fn apply_all_applies_each_separately() {
    let k = 5;
    let fs: Vec<Box<dyn Fn(i32) -> i32>> = vec![Box::new(move |x| x * k), Box::new(triple), Box::new(|x| x - 1)];
    assert_eq!(apply_all(&fs, 2), [10, 6, 1], "not chained: each gets 2");
    assert!(apply_all(&[], 2).is_empty());
}

#[test]
fn standard_steps_add_double_subtract() {
    let steps = standard_steps();
    assert_eq!(steps.len(), 3);
    assert_eq!([steps[0](4), steps[1](4), steps[2](4)], [5, 8, 1]);
}
