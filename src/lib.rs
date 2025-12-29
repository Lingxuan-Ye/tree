#![no_std]

extern crate alloc;

pub use self::index::{Index, IndexRange};
pub use self::slice_tree::SliceTree;

pub mod index;
pub mod slice_tree;
pub mod traverse;

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

    fn node(&self, index: Index<N>) -> Option<&Self::Node>;

    fn node_mut(&mut self, index: Index<N>) -> Option<&mut Self::Node>;

    fn parent(&self, index: Index<N>) -> Option<&Self::Node> {
        let index = index.parent()?;
        self.node(index)
    }

    fn parent_mut(&mut self, index: Index<N>) -> Option<&mut Self::Node> {
        let index = index.parent()?;
        self.node_mut(index)
    }

    fn first_child(&self, index: Index<N>) -> Option<&Self::Node> {
        let index = index.first_child()?;
        self.node(index)
    }

    fn first_child_mut(&mut self, index: Index<N>) -> Option<&mut Self::Node> {
        let index = index.first_child()?;
        self.node_mut(index)
    }

    fn last_child(&self, index: Index<N>) -> Option<&Self::Node> {
        let index = index.last_child()?;
        self.node(index)
    }

    fn last_child_mut(&mut self, index: Index<N>) -> Option<&mut Self::Node> {
        let index = index.last_child()?;
        self.node_mut(index)
    }

    fn child(&self, index: Index<N>, n: usize) -> Option<&Self::Node> {
        let index = index.child(n)?;
        self.node(index)
    }

    fn child_mut(&mut self, index: Index<N>, n: usize) -> Option<&mut Self::Node> {
        let index = index.child(n)?;
        self.node_mut(index)
    }

    fn iter_children(
        &self,
        index: Index<N>,
    ) -> Option<impl DoubleEndedIterator<Item = &Self::Node>> {
        if index.to_flattened() >= self.len() {
            return None;
        }
        let children = index.iter_children();
        Some(children.flat_map(|index| self.node(index)))
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
        Some(level.flat_map(|index| self.node(index)))
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
    fn left_child(&self, index: Index<2>) -> Option<&Self::Node> {
        self.first_child(index)
    }

    fn left_child_mut(&mut self, index: Index<2>) -> Option<&mut Self::Node> {
        self.first_child_mut(index)
    }

    fn right_child(&self, index: Index<2>) -> Option<&Self::Node> {
        self.last_child(index)
    }

    fn right_child_mut(&mut self, index: Index<2>) -> Option<&mut Self::Node> {
        self.last_child_mut(index)
    }

    fn traverse_in_order(&self) -> impl Iterator<Item = &Self::Node>;

    fn traverse_in_order_mut(&mut self) -> impl Iterator<Item = &mut Self::Node>;
}
