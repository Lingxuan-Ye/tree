#![no_std]

extern crate alloc;

pub use self::index::{Index, IndexRange};

pub mod traverse;

mod impls;
mod index;

pub trait CompleteTree<const N: usize> {
    type Node;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn height(&self) -> usize {
        let index = self.len() - 1;
        Index::<N>::from_flattened(index).depth()
    }

    fn get(&self, index: Index<N>) -> Option<&Self::Node>;

    fn get_mut(&mut self, index: Index<N>) -> Option<&mut Self::Node>;

    fn get_parent(&self, index: Index<N>) -> Option<&Self::Node> {
        let index = index.parent()?;
        self.get(index)
    }

    fn get_parent_mut(&mut self, index: Index<N>) -> Option<&mut Self::Node> {
        let index = index.parent()?;
        self.get_mut(index)
    }

    fn get_left_most_child(&self, index: Index<N>) -> Option<&Self::Node> {
        let index = index.left_most_child()?;
        self.get(index)
    }

    fn get_left_most_child_mut(&mut self, index: Index<N>) -> Option<&mut Self::Node> {
        let index = index.left_most_child()?;
        self.get_mut(index)
    }

    fn get_right_most_child(&self, index: Index<N>) -> Option<&Self::Node> {
        let index = index.right_most_child()?;
        self.get(index)
    }

    fn get_right_most_child_mut(&mut self, index: Index<N>) -> Option<&mut Self::Node> {
        let index = index.right_most_child()?;
        self.get_mut(index)
    }

    fn get_nth_child(&self, index: Index<N>, n: usize) -> Option<&Self::Node> {
        let index = index.nth_child(n)?;
        self.get(index)
    }

    fn get_nth_child_mut(&mut self, index: Index<N>, n: usize) -> Option<&mut Self::Node> {
        let index = index.nth_child(n)?;
        self.get_mut(index)
    }

    fn iter_children(
        &self,
        index: Index<N>,
    ) -> Option<impl DoubleEndedIterator<Item = &Self::Node>> {
        if index.to_flattened() >= self.len() {
            return None;
        }
        let children = index.iter_children();
        Some(children.flat_map(|index| self.get(index)))
    }

    fn iter_children_mut(
        &mut self,
        index: Index<N>,
    ) -> Option<impl DoubleEndedIterator<Item = &mut Self::Node>>;

    fn iter_level(&self, depth: usize) -> Option<impl DoubleEndedIterator<Item = &Self::Node>> {
        if depth > self.height() {
            return None;
        }
        let level = IndexRange::<N>::level(depth);
        Some(level.flat_map(|index| self.get(index)))
    }

    fn iter_level_mut(
        &mut self,
        depth: usize,
    ) -> Option<impl DoubleEndedIterator<Item = &mut Self::Node>>;

    fn traverse_level_order(&self) -> impl DoubleEndedIterator<Item = &Self::Node>;

    fn traverse_level_order_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Self::Node>;

    fn traverse_pre_order(&self) -> impl Iterator<Item = &Self::Node>;

    fn traverse_pre_order_mut(&mut self) -> impl Iterator<Item = &mut Self::Node>;

    fn traverse_post_order(&self) -> impl Iterator<Item = &Self::Node>;

    fn traverse_post_order_mut(&mut self) -> impl Iterator<Item = &mut Self::Node>;
}

pub trait CompleteBinaryTree: CompleteTree<2> {
    fn get_left_child(&self, index: Index<2>) -> Option<&Self::Node> {
        self.get_left_most_child(index)
    }

    fn get_left_child_mut(&mut self, index: Index<2>) -> Option<&mut Self::Node> {
        self.get_left_most_child_mut(index)
    }

    fn get_right_child(&self, index: Index<2>) -> Option<&Self::Node> {
        self.get_right_most_child(index)
    }

    fn get_right_child_mut(&mut self, index: Index<2>) -> Option<&mut Self::Node> {
        self.get_right_most_child_mut(index)
    }

    fn traverse_in_order(&self) -> impl Iterator<Item = &Self::Node>;

    fn traverse_in_order_mut(&mut self) -> impl Iterator<Item = &mut Self::Node>;
}
