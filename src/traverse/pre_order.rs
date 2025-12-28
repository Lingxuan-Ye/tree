use super::super::CompleteTree;
use super::super::index::Index;
use alloc::vec::Vec;
use core::iter::FusedIterator;

#[derive(Debug, Clone)]
pub struct TraversePreOrder<'a, const N: usize, T>
where
    T: CompleteTree<N> + ?Sized,
{
    tree: &'a T,
    stack: Vec<Index<N>>,
}

impl<'a, const N: usize, T> TraversePreOrder<'a, N, T>
where
    T: CompleteTree<N> + ?Sized,
{
    pub fn new(tree: &'a T) -> Self {
        let mut stack = Vec::new();
        if !tree.is_empty() {
            stack.push(Index::root());
        }
        Self { tree, stack }
    }
}

impl<'a, const N: usize, T> Iterator for TraversePreOrder<'a, N, T>
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

impl<const N: usize, T> FusedIterator for TraversePreOrder<'_, N, T> where
    T: CompleteTree<N> + ?Sized
{
}
