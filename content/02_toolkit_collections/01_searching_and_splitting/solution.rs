// Searching and Splitting — reference solution.
// One slice method (occasionally two) per function; sub-slices are borrowed,
// never copied, and nothing indexes element by element.

// ── Windows and chunks ──────────────────────────────────────────────────────

pub fn has_adjacent_duplicate(xs: &[i32]) -> bool {
    xs.windows(2).any(|w| w[0] == w[1])
}

pub fn max_window_sum(xs: &[i32], k: usize) -> Option<i32> {
    xs.windows(k).map(|w| w.iter().sum()).max()
}

pub fn chunk_sums(xs: &[i32], size: usize) -> Vec<i32> {
    xs.chunks(size).map(|c| c.iter().sum()).collect()
}

pub fn full_chunks(xs: &[i32], size: usize) -> (Vec<&[i32]>, &[i32]) {
    let chunks = xs.chunks_exact(size);
    let rest = chunks.remainder();
    (chunks.collect(), rest)
}

pub fn chunks_from_end(xs: &[i32], size: usize) -> Vec<&[i32]> {
    xs.rchunks(size).collect()
}

pub fn ascending_runs(xs: &[i32]) -> Vec<&[i32]> {
    xs.chunk_by(|a, b| a < b).collect()
}

// ── Splitting ───────────────────────────────────────────────────────────────

pub fn split_on_zero(xs: &[i32]) -> Vec<&[i32]> {
    xs.split(|&x| x == 0).collect()
}

pub fn split_at_first(xs: &[i32], sep: i32) -> Vec<&[i32]> {
    xs.splitn(2, |&x| x == sep).collect()
}

pub fn head_and_tail(xs: &[i32]) -> Option<(i32, &[i32])> {
    xs.split_first().map(|(&head, tail)| (head, tail))
}

pub fn all_but_last(xs: &[i32]) -> &[i32] {
    xs.split_last().map_or(&[], |(_, init)| init)
}

pub fn take_prefix(xs: &[i32], n: usize) -> Option<(&[i32], &[i32])> {
    xs.split_at_checked(n)
}

pub fn swap_halves(xs: &mut [i32]) {
    let half = xs.len() / 2;
    let (front, rest) = xs.split_at_mut(half);
    let back_start = rest.len() - half;
    let (_, back) = rest.split_at_mut(back_start);
    front.swap_with_slice(back);
}

// ── Ends and membership ─────────────────────────────────────────────────────

pub fn first_and_last(xs: &[i32]) -> Option<(i32, i32)> {
    xs.first().copied().zip(xs.last().copied())
}

pub fn framed_by(xs: &[i32], prefix: &[i32], suffix: &[i32]) -> bool {
    xs.starts_with(prefix) && xs.ends_with(suffix)
}

pub fn contains_all(xs: &[i32], needles: &[i32]) -> bool {
    needles.iter().all(|n| xs.contains(n))
}

// ── Searching ───────────────────────────────────────────────────────────────

pub fn insertion_point(sorted: &[i32], x: i32) -> usize {
    sorted.binary_search(&x).unwrap_or_else(|i| i)
}

pub fn name_for_id<'a>(by_id: &[(u32, &'a str)], id: u32) -> Option<&'a str> {
    by_id
        .binary_search_by_key(&id, |&(k, _)| k)
        .ok()
        .map(|i| by_id[i].1)
}

pub fn count_in_range(sorted: &[i32], lo: i32, hi: i32) -> usize {
    let start = sorted.partition_point(|&x| x < lo);
    let end = sorted.partition_point(|&x| x < hi);
    end.saturating_sub(start)
}

// ── Joining ─────────────────────────────────────────────────────────────────

pub fn flatten_rows(rows: &[Vec<i32>]) -> Vec<i32> {
    rows.concat()
}

pub fn join_with(rows: &[Vec<i32>], sep: i32) -> Vec<i32> {
    rows.join(&sep)
}

// ── Slice patterns ──────────────────────────────────────────────────────────

pub fn inner(xs: &[i32]) -> &[i32] {
    match xs {
        [_, inner @ .., _] => inner,
        _ => &[],
    }
}

pub fn first_two_and_rest(xs: &[i32]) -> Option<(i32, i32, &[i32])> {
    match xs {
        [a, b, rest @ ..] => Some((*a, *b, rest)),
        _ => None,
    }
}

pub fn is_palindrome<T: PartialEq>(xs: &[T]) -> bool {
    match xs {
        [first, inner @ .., last] => first == last && is_palindrome(inner),
        _ => true,
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
