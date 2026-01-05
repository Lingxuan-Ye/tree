use crate::Index;
use alloc::vec::Vec;
use core::iter::FusedIterator;

#[derive(Debug, Clone)]
pub struct PreOrder<const N: usize> {
    stack: Vec<Index<N>>,
    tree_len: usize,
}

impl<const N: usize> PreOrder<N> {
    pub fn new(tree_len: usize) -> Self {
        if tree_len == 0 {
            let stack = Vec::new();
            return Self { stack, tree_len };
        }

        let last = tree_len - 1;
        let tree_height = Index::<N>::from_flattened(last).depth();
        let capacity = tree_height.saturating_mul(N - 1).saturating_add(1);
        let mut stack = Vec::with_capacity(capacity);

        let root = Index::root();
        stack.push(root);

        Self { stack, tree_len }
    }
}

impl<const N: usize> Iterator for PreOrder<N> {
    type Item = Index<N>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.stack.pop()?;
        let children = index.iter_children().cap(self.tree_len);
        for child in children.rev() {
            self.stack.push(child);
        }
        Some(index)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.stack.len(), Some(self.tree_len))
    }
}

impl<const N: usize> FusedIterator for PreOrder<N> {}
