use crate::{Index, IndexRange, index::IndexRangeIter};
use core::iter::FusedIterator;
use core::range::RangeInclusive;

#[derive(Debug, Clone)]
pub struct LevelOrder<const N: usize>(IndexRangeIter<N>);

impl<const N: usize> LevelOrder<N> {
    pub fn new(tree_len: usize) -> Self {
        if tree_len == 0 {
            let iter = IndexRange::empty().into_iter();
            return Self(iter);
        }

        let start = const { Index::<N>::root().to_linear() };
        let last = tree_len - 1;
        let range = RangeInclusive { start, last };
        let iter = IndexRange::from_linear(range).into_iter();
        Self(iter)
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
