// Min Stack — reference solution.
// Alongside the elements, keep for each depth the index of the minimum of the
// stack up to that depth; popping restores the previous minimum automatically,
// and no `T: Clone` is needed. O(1) per operation, O(n) space.

pub struct MinStack<T> {
    items: Vec<T>,
    /// `min_at[i]` is the index of the smallest element in `items[..=i]`.
    min_at: Vec<usize>,
}

impl<T: Ord> MinStack<T> {
    pub fn new() -> Self {
        Self { items: Vec::new(), min_at: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        let min_index = match self.min_at.last() {
            Some(&m) if self.items[m] <= value => m,
            _ => self.items.len(),
        };
        self.items.push(value);
        self.min_at.push(min_index);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.min_at.pop();
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn min(&self) -> Option<&T> {
        self.min_at.last().map(|&m| &self.items[m])
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T: Ord> Default for MinStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
