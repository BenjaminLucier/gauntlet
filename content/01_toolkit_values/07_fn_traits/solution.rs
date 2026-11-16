// Fn, FnMut and FnOnce — reference solution.
// Take the loosest bound the body allows: FnOnce if you call once, FnMut if you call
// repeatedly in sequence, Fn only when you need shared calls.

// ── FnOnce: called at most once ─────────────────────────────────────────────

pub fn apply<F: FnOnce(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

pub fn run_once<R>(f: impl FnOnce() -> R) -> R {
    f()
}

pub fn call_or_default<T>(f: Option<impl FnOnce() -> T>, default: T) -> T {
    f.map_or(default, |f| f())
}

pub fn fire_once(slot: &mut Option<Box<dyn FnOnce() -> String>>) -> Option<String> {
    slot.take().map(|f| f())
}

// ── FnMut: called repeatedly, one call at a time ────────────────────────────

pub fn apply_twice(mut f: impl FnMut(i32) -> i32, x: i32) -> i32 {
    let once = f(x);
    f(once)
}

pub fn apply_n(mut f: impl FnMut(i32) -> i32, n: usize, x: i32) -> i32 {
    (0..n).fold(x, |acc, _| f(acc))
}

pub fn count_where(xs: &[i32], mut pred: impl FnMut(&i32) -> bool) -> usize {
    xs.iter().filter(|x| pred(x)).count()
}

pub fn best_by<'a>(words: &[&'a str], mut score: impl FnMut(&str) -> usize) -> Option<&'a str> {
    words.iter().copied().rev().max_by_key(|w| score(w))
}

pub fn call_n(n: usize, mut f: impl FnMut()) {
    for _ in 0..n {
        f();
    }
}

pub fn visit(xs: &[i32], mut f: impl FnMut(i32)) {
    for &x in xs {
        f(x);
    }
}

pub fn running_totals(xs: &[i32]) -> Vec<i32> {
    let mut sum = 0;
    let mut out = Vec::with_capacity(xs.len());
    visit(xs, |x| {
        sum += x;
        out.push(sum);
    });
    out
}

pub fn retry<T, E>(attempts: usize, mut op: impl FnMut() -> Result<T, E>) -> Result<T, E> {
    let mut result = op();
    for _ in 1..attempts {
        if result.is_ok() {
            break;
        }
        result = op();
    }
    result
}

// ── Fn: called through a shared reference ───────────────────────────────────

pub fn map_all(xs: &[i32], f: &dyn Fn(i32) -> i32) -> Vec<i32> {
    xs.iter().map(|&x| f(x)).collect()
}

pub fn total_by(xs: &[i32], f: &impl Fn(i32) -> i32) -> i32 {
    xs.iter().map(|&x| f(x)).sum()
}

// ── move ────────────────────────────────────────────────────────────────────

/// A boxed greeting function, as returned by `greeters`.
pub type Greeter = Box<dyn Fn(&str) -> String>;

pub fn greeters(names: Vec<String>) -> Vec<Greeter> {
    names
        .into_iter()
        .map(|name| Box::new(move |greeting: &str| format!("{greeting}, {name}!")) as Greeter)
        .collect()
}

// ── Functions as values ─────────────────────────────────────────────────────

pub fn lengths(words: &[&str]) -> Vec<usize> {
    words.iter().copied().map(str::len).collect()
}

pub fn abs_all(xs: &[i32]) -> Vec<i32> {
    xs.iter().copied().map(i32::abs).collect()
}

pub fn parse_all(words: &[&str]) -> Vec<Option<i32>> {
    words.iter().copied().map(str::parse).map(Result::ok).collect()
}

// ── Collections of closures ─────────────────────────────────────────────────

pub fn apply_all(fs: &[Box<dyn Fn(i32) -> i32>], x: i32) -> Vec<i32> {
    fs.iter().map(|f| f(x)).collect()
}

pub fn standard_steps() -> Vec<Box<dyn Fn(i32) -> i32>> {
    vec![Box::new(|x| x + 1), Box::new(|x| x * 2), Box::new(|x| x - 3)]
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
