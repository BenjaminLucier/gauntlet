// Merge Sort — reference solution.
// Top-down: sort both halves, then merge by parking the left half in a scratch
// buffer (cloned once, then reused via swaps) and filling the slice from the
// front, taking from the left run on ties for stability. O(n log n) time, O(n) space.

use std::cmp::Ordering;
use std::mem;

pub fn merge_sort<T: Ord + Clone>(v: &mut [T]) {
    merge_sort_by(v, T::cmp);
}

pub fn merge_sort_by<T: Clone, F>(v: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    if v.len() < 2 {
        return;
    }
    // The left half of any split is at most n / 2 long. Cloning is only to
    // give the buffer valid contents; from here on elements are swapped.
    let mut buf = v[..v.len() / 2].to_vec();
    sort_rec(v, &mut buf, &mut compare);
}

fn sort_rec<T, F>(v: &mut [T], buf: &mut [T], compare: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    if v.len() < 2 {
        return;
    }
    let mid = v.len() / 2;
    {
        let (left, right) = v.split_at_mut(mid);
        sort_rec(left, buf, compare);
        sort_rec(right, buf, compare);
        if compare(&left[mid - 1], &right[0]) != Ordering::Greater {
            return; // already in order
        }
    }
    merge(v, mid, &mut buf[..mid], compare);
}

/// Merge the sorted runs `v[..mid]` and `v[mid..]`, using `buf` (length `mid`).
fn merge<T, F>(v: &mut [T], mid: usize, buf: &mut [T], compare: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    buf.swap_with_slice(&mut v[..mid]);
    let (mut i, mut j, mut k) = (0, mid, 0);
    // Invariant: k = i + (j - mid) <= j, so slot k never holds an unread
    // right-run element.
    while i < buf.len() && j < v.len() {
        if compare(&v[j], &buf[i]) == Ordering::Less {
            v.swap(k, j);
            j += 1;
        } else {
            mem::swap(&mut v[k], &mut buf[i]);
            i += 1;
        }
        k += 1;
    }
    // Leftover right-run elements are already in place; move the rest of buf.
    v[k..k + buf.len() - i].swap_with_slice(&mut buf[i..]);
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
