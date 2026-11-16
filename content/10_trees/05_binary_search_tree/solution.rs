// Binary Search Tree — reference solution.
// Every operation walks a `&mut Link` cursor (or a shared one) down from the
// root, so nothing recurses. Removal splices the in-order successor out of the
// right subtree. Each operation is O(h); iteration and drop are O(n).

use std::borrow::Borrow;
use std::cmp::Ordering;

pub struct Bst<K, V> {
    root: Link<K, V>,
    len: usize,
}

struct Node<K, V> {
    key: K,
    value: V,
    left: Link<K, V>,
    right: Link<K, V>,
}

type Link<K, V> = Option<Box<Node<K, V>>>;

/// The link that holds `key`, or the empty link where it would be inserted.
fn find_link<'a, K, V, Q>(mut link: &'a mut Link<K, V>, key: &Q) -> &'a mut Link<K, V>
where
    K: Borrow<Q>,
    Q: Ord + ?Sized,
{
    loop {
        // Decide the direction with a shared look first: returning `link`
        // while a mutable borrow of the node is alive would not compile.
        let ord = match link.as_deref() {
            None => return link,
            Some(node) => key.cmp(node.key.borrow()),
        };
        if ord == Ordering::Equal {
            return link;
        }
        let node = link.as_mut().expect("checked non-empty above");
        link = if ord == Ordering::Less {
            &mut node.left
        } else {
            &mut node.right
        };
    }
}

/// Detach the minimum node of a non-empty subtree, splicing its right child
/// into its place.
fn take_min<K, V>(mut link: &mut Link<K, V>) -> Box<Node<K, V>> {
    while link.as_ref().is_some_and(|node| node.left.is_some()) {
        link = &mut link.as_mut().expect("has a left child").left;
    }
    let mut min = link.take().expect("subtree is non-empty");
    *link = min.right.take();
    min
}

impl<K: Ord, V> Bst<K, V> {
    pub fn new() -> Self {
        Bst { root: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let link = find_link(&mut self.root, &key);
        match link {
            Some(node) => Some(std::mem::replace(&mut node.value, value)),
            None => {
                *link = Some(Box::new(Node {
                    key,
                    value,
                    left: None,
                    right: None,
                }));
                self.len += 1;
                None
            }
        }
    }

    fn find<Q>(&self, key: &Q) -> Option<&Node<K, V>>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut cur = self.root.as_deref();
        while let Some(node) = cur {
            cur = match key.cmp(node.key.borrow()) {
                Ordering::Less => node.left.as_deref(),
                Ordering::Greater => node.right.as_deref(),
                Ordering::Equal => return Some(node),
            };
        }
        None
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.find(key).map(|node| &node.value)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        find_link(&mut self.root, key)
            .as_mut()
            .map(|node| &mut node.value)
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.find(key).is_some()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let link = find_link(&mut self.root, key);
        let mut node = link.take()?;
        *link = match (node.left.take(), node.right.take()) {
            (None, None) => None,
            (Some(child), None) | (None, Some(child)) => Some(child),
            (Some(left), Some(right)) => {
                // The successor (smallest key on the right) takes its place.
                let mut right = Some(right);
                let mut successor = take_min(&mut right);
                successor.left = Some(left);
                successor.right = right;
                Some(successor)
            }
        };
        self.len -= 1;
        Some(node.value)
    }

    pub fn min(&self) -> Option<(&K, &V)> {
        let mut node = self.root.as_deref()?;
        while let Some(left) = node.left.as_deref() {
            node = left;
        }
        Some((&node.key, &node.value))
    }

    pub fn max(&self) -> Option<(&K, &V)> {
        let mut node = self.root.as_deref()?;
        while let Some(right) = node.right.as_deref() {
            node = right;
        }
        Some((&node.key, &node.value))
    }

    pub fn floor<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut best = None;
        let mut cur = self.root.as_deref();
        while let Some(node) = cur {
            match key.cmp(node.key.borrow()) {
                Ordering::Less => cur = node.left.as_deref(),
                Ordering::Equal => return Some((&node.key, &node.value)),
                Ordering::Greater => {
                    best = Some((&node.key, &node.value));
                    cur = node.right.as_deref();
                }
            }
        }
        best
    }

    pub fn ceiling<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut best = None;
        let mut cur = self.root.as_deref();
        while let Some(node) = cur {
            match key.cmp(node.key.borrow()) {
                Ordering::Greater => cur = node.right.as_deref(),
                Ordering::Equal => return Some((&node.key, &node.value)),
                Ordering::Less => {
                    best = Some((&node.key, &node.value));
                    cur = node.left.as_deref();
                }
            }
        }
        best
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        let mut iter = Iter {
            stack: Vec::new(),
            remaining: self.len,
        };
        iter.push_left_spine(self.root.as_deref());
        iter
    }
}

impl<K: Ord, V> Default for Bst<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for Bst<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut tree = Bst::new();
        tree.extend(iter);
        tree
    }
}

impl<K: Ord, V> Extend<(K, V)> for Bst<K, V> {
    /// Like repeated `insert`: a later value for a key replaces an earlier one.
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        for (key, value) in iter {
            self.insert(key, value);
        }
    }
}

impl<K, V> Drop for Bst<K, V> {
    fn drop(&mut self) {
        // The default drop recurses once per level; a degenerate tree is n deep.
        let mut stack: Vec<Box<Node<K, V>>> = self.root.take().into_iter().collect();
        while let Some(mut node) = stack.pop() {
            stack.extend(node.left.take());
            stack.extend(node.right.take());
        }
    }
}

/// In-order iterator over `(&K, &V)`, smallest key first.
pub struct Iter<'a, K, V> {
    /// Nodes whose left subtree is done but which haven't been yielded yet.
    stack: Vec<&'a Node<K, V>>,
    remaining: usize,
}

impl<'a, K, V> Iter<'a, K, V> {
    fn push_left_spine(&mut self, mut cur: Option<&'a Node<K, V>>) {
        while let Some(node) = cur {
            self.stack.push(node);
            cur = node.left.as_deref();
        }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        self.push_left_spine(node.right.as_deref());
        self.remaining -= 1;
        Some((&node.key, &node.value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a, K: Ord, V> IntoIterator for &'a Bst<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Iter<'a, K, V> {
        self.iter()
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
