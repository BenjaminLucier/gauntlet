// Quicksort — reference solution.
// Random pivot (xorshift), Dijkstra 3-way partition so equal keys are settled in
// one pass, recursion into the smaller side and a loop on the larger: expected
// O(n log n) time, O(n) for all-equal input, O(log n) stack. Not stable.

use std::cmp::Ordering;
use std::mem;

/// Tiny xorshift64 generator for pivot choice. Fixed seed: deterministic, but
/// no realistic input lines up with its sequence.
struct Rng(u64);

impl Rng {
    fn below(&mut self, n: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % n as u64) as usize
    }
}

pub fn quicksort<T: Ord>(v: &mut [T]) {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    sort_rec(v, &mut rng);
}

fn sort_rec<T: Ord>(mut v: &mut [T], rng: &mut Rng) {
    while v.len() > 1 {
        let pivot = rng.below(v.len());
        let (lt, gt) = partition3(v, pivot);
        let (less, rest) = mem::take(&mut v).split_at_mut(lt);
        let greater = &mut rest[gt - lt..];
        // Recurse into the smaller part; loop on the larger one.
        if less.len() < greater.len() {
            sort_rec(less, rng);
            v = greater;
        } else {
            sort_rec(greater, rng);
            v = less;
        }
    }
}

pub fn partition3<T: Ord>(v: &mut [T], pivot_index: usize) -> (usize, usize) {
    v.swap(0, pivot_index);
    // v[..lt] < p, v[lt..i] == p, v[i..gt] unseen, v[gt..] > p.
    // v[lt] is always a copy of the pivot value, so we compare against it.
    let (mut lt, mut i, mut gt) = (0, 1, v.len());
    while i < gt {
        match v[i].cmp(&v[lt]) {
            Ordering::Less => {
                v.swap(lt, i);
                lt += 1;
                i += 1;
            }
            Ordering::Greater => {
                gt -= 1;
                v.swap(i, gt);
            }
            Ordering::Equal => i += 1,
        }
    }
    (lt, gt)
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
