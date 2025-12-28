use self::traverse::{
    TraverseInOrder, TraverseInOrderMut, TraversePostOrder, TraversePostOrderMut, TraversePreOrder,
    TraversePreOrderMut,
};
use crate::{CompleteBinaryTree, CompleteTree, Index, IndexRange};

pub mod traverse;

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
