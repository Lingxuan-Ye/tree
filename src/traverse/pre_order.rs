use crate::{CompleteTree, Index};
use alloc::vec::Vec;
use core::iter::FusedIterator;

#[derive(Debug, Clone)]
pub struct PreOrder<'a, const N: usize, T>
where
    T: CompleteTree<N> + ?Sized,
{
    stack: Vec<Index<N>>,
    tree: &'a T,
}

impl<'a, const N: usize, T> PreOrder<'a, N, T>
where
    T: CompleteTree<N> + ?Sized,
{
    pub fn new(tree: &'a T) -> Self {
        let capacity = tree.height().saturating_mul(N - 1).saturating_add(1);
        let mut stack = Vec::with_capacity(capacity);
        if !tree.is_empty() {
            stack.push(Index::root());
        }
        Self { stack, tree }
    }
}

impl<'a, const N: usize, T> Iterator for PreOrder<'a, N, T>
where
    T: CompleteTree<N> + ?Sized,
{
    type Item = &'a T::Node;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.stack.pop()?;
        let children = index.iter_children().cap(self.tree.len()).to_flattened();
        for child in children.rev() {
            self.stack.push(Index::<N>::from_flattened(child));
        }
        self.tree.get(index)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.stack.len(), Some(self.tree.len()))
    }
}

impl<const N: usize, T> FusedIterator for PreOrder<'_, N, T> where T: CompleteTree<N> + ?Sized {}
