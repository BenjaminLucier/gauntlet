// Binary Heap — reference solution.
// A complete binary tree in a Vec (children of i at 2i+1, 2i+2). push sifts up,
// pop swaps the root out and sifts down: O(log n). From<Vec> sifts down every
// parent from the bottom up, O(n); iter / into_vec expose the array as is.

pub struct MaxHeap<T> {
    data: Vec<T>,
}

/// Restores the heap property below `i` within `data[..end]`.
fn sift_down<T: Ord>(data: &mut [T], mut i: usize, end: usize) {
    loop {
        let left = 2 * i + 1;
        if left >= end {
            return;
        }
        let right = left + 1;
        let child = if right < end && data[right] > data[left] { right } else { left };
        if data[child] <= data[i] {
            return;
        }
        data.swap(i, child);
        i = child;
    }
}

impl<T: Ord> MaxHeap<T> {
    pub fn new() -> Self {
        MaxHeap { data: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        let mut i = self.data.len() - 1;
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.data[i] <= self.data[parent] {
                break;
            }
            self.data.swap(i, parent);
            i = parent;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        // Moves the last item to the root and hands back the old root.
        let top = self.data.swap_remove(0);
        let end = self.data.len();
        sift_down(&mut self.data, 0, end);
        Some(top)
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            inner: self.data.iter(),
        }
    }

    pub fn into_vec(self) -> Vec<T> {
        self.data
    }
}

impl<T: Ord> Default for MaxHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> From<Vec<T>> for MaxHeap<T> {
    fn from(mut data: Vec<T>) -> Self {
        let end = data.len();
        // Leaves (indices >= end / 2) are already heaps; fix each parent bottom-up.
        for i in (0..end / 2).rev() {
            sift_down(&mut data, i, end);
        }
        MaxHeap { data }
    }
}

impl<T: Ord> FromIterator<T> for MaxHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from(iter.into_iter().collect::<Vec<T>>())
    }
}

impl<T: Ord> Extend<T> for MaxHeap<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

/// Borrowing iterator over a heap's items, in array (level) order.
pub struct Iter<'a, T> {
    inner: std::slice::Iter<'a, T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T: Ord> IntoIterator for &'a MaxHeap<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
