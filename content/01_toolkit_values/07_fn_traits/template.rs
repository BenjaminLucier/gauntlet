// Fn, FnMut and FnOnce — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: in "Functions as values", pass a path (`str::len`), not a closure;
// `running_totals` must be written with `visit`.
#![allow(unused_imports, unused_variables, dead_code)]

// ── FnOnce: called at most once ─────────────────────────────────────────────

/// Call `f` on `x`. `f` is called once, so the loosest bound — `FnOnce` —
/// accepts every closure. (a generic `F: FnOnce(i32) -> i32`)
pub fn apply<F: FnOnce(i32) -> i32>(f: F, x: i32) -> i32 {
    todo!()
}

/// Call `f` (once) and return its result. (`impl FnOnce`)
pub fn run_once<R>(f: impl FnOnce() -> R) -> R {
    todo!()
}

/// `f()` if there is an `f`, otherwise `default`. (`FnOnce` inside an
/// `Option`)
pub fn call_or_default<T>(f: Option<impl FnOnce() -> T>, default: T) -> T {
    todo!()
}

/// Take the stored callback out of `slot` and call it; `None` if the slot is
/// already empty. Afterwards the slot is empty, so the callback can never run
/// twice. (a `Box<dyn FnOnce>` is called by value; `Option::take`)
pub fn fire_once(slot: &mut Option<Box<dyn FnOnce() -> String>>) -> Option<String> {
    todo!()
}

// ── FnMut: called repeatedly, one call at a time ────────────────────────────

/// `f(f(x))`. (`FnMut` — note the `mut` binding it needs)
pub fn apply_twice(f: impl FnMut(i32) -> i32, x: i32) -> i32 {
    todo!()
}

/// `f` applied `n` times; `n = 0` gives `x` back. (`FnMut` called in a loop
/// or `fold`)
pub fn apply_n(f: impl FnMut(i32) -> i32, n: usize, x: i32) -> i32 {
    todo!()
}

/// How many elements satisfy `pred`. (`FnMut(&i32) -> bool`, `filter`)
pub fn count_where(xs: &[i32], pred: impl FnMut(&i32) -> bool) -> usize {
    todo!()
}

/// The word with the highest `score`; on a tie, the earliest one. `None` if
/// there are no words. (`max_by_key` — check which one it keeps on a tie)
pub fn best_by<'a>(words: &[&'a str], score: impl FnMut(&str) -> usize) -> Option<&'a str> {
    todo!()
}

/// Call `f` `n` times. `f` may change what it captured. (`FnMut`)
pub fn call_n(n: usize, f: impl FnMut()) {
    todo!()
}

/// Call `f` on every element, in order. (`FnMut(i32)`)
pub fn visit(xs: &[i32], f: impl FnMut(i32)) {
    todo!()
}

/// Prefix sums: `[1, 2, 3]` → `[1, 3, 6]`. Use `visit` with a closure that
/// captures a running sum and the output `Vec` by mutable reference.
pub fn running_totals(xs: &[i32]) -> Vec<i32> {
    todo!()
}

/// Call `op` until it returns `Ok` or it has been called `attempts` times;
/// return the last result. `op` is always called at least once.
/// (`FnMut() -> Result<T, E>`)
pub fn retry<T, E>(attempts: usize, op: impl FnMut() -> Result<T, E>) -> Result<T, E> {
    todo!()
}

// ── Fn: called through a shared reference ───────────────────────────────────

/// `f` applied to every element. `f` arrives as `&dyn Fn`: a shared
/// reference, which only lets you call an `Fn`. (`&dyn Fn` — a trait object,
/// no generics)
pub fn map_all(xs: &[i32], f: &dyn Fn(i32) -> i32) -> Vec<i32> {
    todo!()
}

/// The sum of `f(x)` over every element. `f` is borrowed (`&F`), so it must
/// be `Fn`; the caller keeps it and can use it again. (`&impl Fn`, `map`,
/// `sum`)
pub fn total_by(xs: &[i32], f: &impl Fn(i32) -> i32) -> i32 {
    todo!()
}

// ── move ────────────────────────────────────────────────────────────────────

/// A boxed greeting function, as returned by `greeters`.
pub type Greeter = Box<dyn Fn(&str) -> String>;

/// One greeter per name, in order: called with `"Hi"`, it returns
/// `"Hi, <name>!"`, as many times as you like. Each boxed closure is
/// `'static` and outlives this call, so it must own its `name`.
/// (`move`, `Box<dyn Fn>`)
pub fn greeters(names: Vec<String>) -> Vec<Greeter> {
    todo!()
}

// ── Functions as values ─────────────────────────────────────────────────────

/// The byte length of each word. (`map(str::len)`)
pub fn lengths(words: &[&str]) -> Vec<usize> {
    todo!()
}

/// The absolute value of each number. (`map(i32::abs)`)
pub fn abs_all(xs: &[i32]) -> Vec<i32> {
    todo!()
}

/// Parse each word, `None` where it isn't a number. (`map(str::parse)`,
/// `map(Result::ok)`)
pub fn parse_all(words: &[&str]) -> Vec<Option<i32>> {
    todo!()
}

// ── Collections of closures ─────────────────────────────────────────────────

/// Every function applied to `x` on its own (not chained), in order.
/// (`Box<dyn Fn>` in a slice)
pub fn apply_all(fs: &[Box<dyn Fn(i32) -> i32>], x: i32) -> Vec<i32> {
    todo!()
}

/// Three functions: add 1, double, subtract 3. (`Vec<Box<dyn Fn>>` — three
/// closures, three different types, one element type)
pub fn standard_steps() -> Vec<Box<dyn Fn(i32) -> i32>> {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
