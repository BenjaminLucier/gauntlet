// Searching and Splitting — press i in the gauntlet for the statement, t for the tests.
// Replace each `todo!()`. The gauntlet re-runs the tests every time you save.
// RESTRICTIONS: no for/while/loop; slice methods, iterators and slice patterns only.
#![allow(unused_imports, unused_variables, dead_code)]

// ── Windows and chunks ──────────────────────────────────────────────────────

/// Are two neighbouring elements equal? (`windows`)
pub fn has_adjacent_duplicate(xs: &[i32]) -> bool {
    todo!()
}

/// The largest sum of `k` consecutive elements; `None` if `xs` is shorter
/// than `k`. `k` is at least 1. (`windows`)
pub fn max_window_sum(xs: &[i32], k: usize) -> Option<i32> {
    todo!()
}

/// The sum of each group of `size` elements; the last group may be shorter.
/// (`chunks`)
pub fn chunk_sums(xs: &[i32], size: usize) -> Vec<i32> {
    todo!()
}

/// The groups of exactly `size` elements, and the leftover tail that didn't
/// fill a group. (`chunks_exact`, `remainder`)
pub fn full_chunks(xs: &[i32], size: usize) -> (Vec<&[i32]>, &[i32]) {
    todo!()
}

/// Groups of `size` counted from the end: the short group, if any, is the
/// first element of the slice and comes out last. (`rchunks`)
pub fn chunks_from_end(xs: &[i32], size: usize) -> Vec<&[i32]> {
    todo!()
}

/// The maximal strictly increasing runs: `[1, 2, 2, 5, 3]` is
/// `[[1, 2], [2, 5], [3]]`. (`chunk_by`)
pub fn ascending_runs(xs: &[i32]) -> Vec<&[i32]> {
    todo!()
}

// ── Splitting ───────────────────────────────────────────────────────────────

/// The runs between zeros (zeros themselves dropped; empty runs kept).
/// (`split`)
pub fn split_on_zero(xs: &[i32]) -> Vec<&[i32]> {
    todo!()
}

/// Split at the first `sep` only: at most two pieces. (`splitn`)
pub fn split_at_first(xs: &[i32], sep: i32) -> Vec<&[i32]> {
    todo!()
}

/// The first element and everything after it. (`split_first`)
pub fn head_and_tail(xs: &[i32]) -> Option<(i32, &[i32])> {
    todo!()
}

/// Everything except the last element (empty for an empty slice).
/// (`split_last`)
pub fn all_but_last(xs: &[i32]) -> &[i32] {
    todo!()
}

/// The first `n` elements and the rest, or `None` if there aren't `n`.
/// (`split_at_checked`)
pub fn take_prefix(xs: &[i32], n: usize) -> Option<(&[i32], &[i32])> {
    todo!()
}

/// Swap the first `len / 2` elements with the last `len / 2`, in place; an odd
/// middle element stays put. `[1, 2, 3, 4, 5]` → `[4, 5, 3, 1, 2]`.
/// (`split_at_mut`, `swap_with_slice`)
pub fn swap_halves(xs: &mut [i32]) {
    todo!()
}

// ── Ends and membership ─────────────────────────────────────────────────────

/// The first and last elements (the same one for a single element).
/// (`first`, `last`)
pub fn first_and_last(xs: &[i32]) -> Option<(i32, i32)> {
    todo!()
}

/// Does `xs` start with `prefix` and end with `suffix`?
/// (`starts_with`, `ends_with`)
pub fn framed_by(xs: &[i32], prefix: &[i32], suffix: &[i32]) -> bool {
    todo!()
}

/// Does `xs` contain every value in `needles`? (`contains`)
pub fn contains_all(xs: &[i32], needles: &[i32]) -> bool {
    todo!()
}

// ── Searching ───────────────────────────────────────────────────────────────

/// Where `x` is in the sorted slice, or where it would be inserted to keep it
/// sorted. (`binary_search`)
pub fn insertion_point(sorted: &[i32], x: i32) -> usize {
    todo!()
}

/// Look up a name in pairs sorted by id. (`binary_search_by_key`)
pub fn name_for_id<'a>(by_id: &[(u32, &'a str)], id: u32) -> Option<&'a str> {
    todo!()
}

/// How many elements of the sorted slice lie in `lo..hi` (half-open)?
/// 0 if `hi <= lo`. (`partition_point`)
pub fn count_in_range(sorted: &[i32], lo: i32, hi: i32) -> usize {
    todo!()
}

// ── Joining ─────────────────────────────────────────────────────────────────

/// All rows, end to end. (`concat`)
pub fn flatten_rows(rows: &[Vec<i32>]) -> Vec<i32> {
    todo!()
}

/// All rows, with `sep` between consecutive rows. (`join`)
pub fn join_with(rows: &[Vec<i32>], sep: i32) -> Vec<i32> {
    todo!()
}

// ── Slice patterns ──────────────────────────────────────────────────────────

/// Everything but the first and last elements; empty if there are fewer
/// than two. (`[_, inner @ .., _]`)
pub fn inner(xs: &[i32]) -> &[i32] {
    todo!()
}

/// The first two elements and the rest, if there are at least two.
/// (`[a, b, rest @ ..]`)
pub fn first_two_and_rest(xs: &[i32]) -> Option<(i32, i32, &[i32])> {
    todo!()
}

/// Does the slice read the same backwards? Recurse on the inside.
/// (`[first, inner @ .., last]`)
pub fn is_palindrome<T: PartialEq>(xs: &[T]) -> bool {
    todo!()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
