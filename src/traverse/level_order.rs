use crate::{CompleteTree, IndexRange};
use core::iter::FusedIterator;

#[derive(Debug, Clone)]
pub struct TraverseLevelOrder<'a, const N: usize, T>
where
    T: CompleteTree<N> + ?Sized,
{
    tree: &'a T,
    range: IndexRange<N>,
}

impl<'a, const N: usize, T> TraverseLevelOrder<'a, N, T>
where
    T: CompleteTree<N> + ?Sized,
{
    pub fn new(tree: &'a T) -> Self {
        let range = IndexRange::from_flattened(0..tree.len());
        Self { tree, range }
    }
}

impl<'a, const N: usize, T> Iterator for TraverseLevelOrder<'a, N, T>
where
    T: CompleteTree<N> + ?Sized,
{
    type Item = &'a T::Node;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.range.next()?;
        self.tree.get(index)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<const N: usize, T> ExactSizeIterator for TraverseLevelOrder<'_, N, T>
where
    T: CompleteTree<N> + ?Sized,
{
    fn len(&self) -> usize {
        self.range.len()
    }
}

impl<const N: usize, T> DoubleEndedIterator for TraverseLevelOrder<'_, N, T>
where
    T: CompleteTree<N> + ?Sized,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = self.range.next_back()?;
        self.tree.get(index)
    }
}

impl<const N: usize, T> FusedIterator for TraverseLevelOrder<'_, N, T> where
    T: CompleteTree<N> + ?Sized
{
}
