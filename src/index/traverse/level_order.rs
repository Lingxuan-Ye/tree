use crate::{Index, IndexRange};
use core::iter::FusedIterator;

#[derive(Debug, Clone)]
pub struct LevelOrder<const N: usize>(IndexRange<N>);

impl<const N: usize> LevelOrder<N> {
    pub fn new(tree_len: usize) -> Self {
        Self(IndexRange::from_flattened(0..tree_len))
    }
}

impl<const N: usize> Iterator for LevelOrder<N> {
    type Item = Index<N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len();
        (len, Some(len))
    }
}

impl<const N: usize> ExactSizeIterator for LevelOrder<N> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<const N: usize> DoubleEndedIterator for LevelOrder<N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

impl<const N: usize> FusedIterator for LevelOrder<N> {}
