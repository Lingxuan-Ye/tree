use self::traverse::in_order::{TraverseInOrder, TraverseInOrderMut};
use self::traverse::post_order::{TraversePostOrder, TraversePostOrderMut};
use self::traverse::pre_order::{TraversePreOrder, TraversePreOrderMut};
use crate::{CompleteBinaryTree, CompleteTree, Index, IndexRange};
use core::mem;

mod traverse;

#[derive(Debug)]
#[repr(transparent)]
pub struct SliceTree<const N: usize, T>([T]);

impl<const N: usize, T> SliceTree<N, T> {
    pub const fn new(slice: &[T]) -> &Self {
        unsafe { mem::transmute(slice) }
    }

    pub const fn new_mut(slice: &mut [T]) -> &mut Self {
        unsafe { mem::transmute(slice) }
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub const fn as_slice(&self) -> &[T] {
        &self.0
    }

    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.0
    }
}

impl<const N: usize, T> CompleteTree<N> for SliceTree<N, T> {
    type Node = T;

    fn len(&self) -> usize {
        CompleteTree::<N>::len(self.as_slice())
    }

    fn get(&self, index: Index<N>) -> Option<&Self::Node> {
        CompleteTree::<N>::get(self.as_slice(), index)
    }

    fn get_mut(&mut self, index: Index<N>) -> Option<&mut Self::Node> {
        CompleteTree::<N>::get_mut(self.as_mut_slice(), index)
    }

    fn iter_children(
        &self,
        index: Index<N>,
    ) -> Option<impl DoubleEndedIterator<Item = &Self::Node>> {
        CompleteTree::<N>::iter_children(self.as_slice(), index)
    }

    fn iter_children_mut(
        &mut self,
        index: Index<N>,
    ) -> Option<impl DoubleEndedIterator<Item = &mut Self::Node>> {
        CompleteTree::<N>::iter_children_mut(self.as_mut_slice(), index)
    }

    fn iter_level(&self, depth: usize) -> Option<impl DoubleEndedIterator<Item = &Self::Node>> {
        CompleteTree::<N>::iter_level(self.as_slice(), depth)
    }

    fn iter_level_mut(
        &mut self,
        depth: usize,
    ) -> Option<impl DoubleEndedIterator<Item = &mut Self::Node>> {
        CompleteTree::<N>::iter_level_mut(self.as_mut_slice(), depth)
    }

    fn traverse_level_order(&self) -> impl DoubleEndedIterator<Item = &Self::Node> {
        CompleteTree::<N>::traverse_level_order(self.as_slice())
    }

    fn traverse_level_order_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Self::Node> {
        CompleteTree::<N>::traverse_level_order_mut(self.as_mut_slice())
    }

    fn traverse_pre_order(&self) -> impl Iterator<Item = &Self::Node> {
        CompleteTree::<N>::traverse_pre_order(self.as_slice())
    }

    fn traverse_pre_order_mut(&mut self) -> impl Iterator<Item = &mut Self::Node> {
        CompleteTree::<N>::traverse_pre_order_mut(self.as_mut_slice())
    }

    fn traverse_post_order(&self) -> impl Iterator<Item = &Self::Node> {
        CompleteTree::<N>::traverse_post_order(self.as_slice())
    }

    fn traverse_post_order_mut(&mut self) -> impl Iterator<Item = &mut Self::Node> {
        CompleteTree::<N>::traverse_post_order_mut(self.as_mut_slice())
    }
}

impl<T> CompleteBinaryTree for SliceTree<2, T> {
    fn traverse_in_order(&self) -> impl Iterator<Item = &Self::Node> {
        CompleteBinaryTree::traverse_in_order(self.as_slice())
    }

    fn traverse_in_order_mut(&mut self) -> impl Iterator<Item = &mut Self::Node> {
        CompleteBinaryTree::traverse_in_order_mut(self.as_mut_slice())
    }
}

impl<const N: usize, T> CompleteTree<N> for [T] {
    type Node = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, index: Index<N>) -> Option<&Self::Node> {
        let index = index.to_flattened();
        self.get(index)
    }

    fn get_mut(&mut self, index: Index<N>) -> Option<&mut Self::Node> {
        let index = index.to_flattened();
        self.get_mut(index)
    }

    fn iter_children(
        &self,
        index: Index<N>,
    ) -> Option<impl DoubleEndedIterator<Item = &Self::Node>> {
        if index.to_flattened() >= self.len() {
            return None;
        }
        let children = index.iter_children().cap(self.len()).to_flattened();
        self.get(children).map(Self::iter)
    }

    fn iter_children_mut(
        &mut self,
        index: Index<N>,
    ) -> Option<impl DoubleEndedIterator<Item = &mut Self::Node>> {
        if index.to_flattened() >= self.len() {
            return None;
        }
        let children = index.iter_children().cap(self.len()).to_flattened();
        self.get_mut(children).map(Self::iter_mut)
    }

    fn iter_level(&self, depth: usize) -> Option<impl DoubleEndedIterator<Item = &Self::Node>> {
        if depth > CompleteTree::<N>::height(self) {
            return None;
        }
        let level = IndexRange::<N>::level(depth).cap(self.len()).to_flattened();
        self.get(level).map(Self::iter)
    }

    fn iter_level_mut(
        &mut self,
        depth: usize,
    ) -> Option<impl DoubleEndedIterator<Item = &mut Self::Node>> {
        if depth > CompleteTree::<N>::height(self) {
            return None;
        }
        let level = IndexRange::<N>::level(depth).cap(self.len()).to_flattened();
        self.get_mut(level).map(Self::iter_mut)
    }

    fn traverse_level_order(&self) -> impl DoubleEndedIterator<Item = &Self::Node> {
        self.iter()
    }

    fn traverse_level_order_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Self::Node> {
        self.iter_mut()
    }

    fn traverse_pre_order(&self) -> impl Iterator<Item = &Self::Node> {
        TraversePreOrder::<N, T>::new(self)
    }

    fn traverse_pre_order_mut(&mut self) -> impl Iterator<Item = &mut Self::Node> {
        TraversePreOrderMut::<N, T>::new(self)
    }

    fn traverse_post_order(&self) -> impl Iterator<Item = &Self::Node> {
        TraversePostOrder::<N, T>::new(self)
    }

    fn traverse_post_order_mut(&mut self) -> impl Iterator<Item = &mut Self::Node> {
        TraversePostOrderMut::<N, T>::new(self)
    }
}

impl<T> CompleteBinaryTree for [T] {
    fn traverse_in_order(&self) -> impl Iterator<Item = &Self::Node> {
        TraverseInOrder::new(self)
    }

    fn traverse_in_order_mut(&mut self) -> impl Iterator<Item = &mut Self::Node> {
        TraverseInOrderMut::new(self)
    }
}
