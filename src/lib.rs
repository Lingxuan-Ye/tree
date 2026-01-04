#![no_std]

extern crate alloc;

pub use self::index::{Index, IndexRange};
pub use self::slice_tree::SliceTree;

pub mod index;
pub mod slice_tree;

pub trait CompleteTree<const N: usize> {
    type Node;

    type IterChildren<'a>: Iterator<Item = &'a Self::Node>
    where
        Self: 'a;

    type IterChildrenMut<'a>: Iterator<Item = &'a mut Self::Node>
    where
        Self: 'a;

    type IterLevel<'a>: Iterator<Item = &'a Self::Node>
    where
        Self: 'a;

    type IterLevelMut<'a>: Iterator<Item = &'a mut Self::Node>
    where
        Self: 'a;

    type LevelOrder<'a>: Iterator<Item = &'a Self::Node>
    where
        Self: 'a;

    type LevelOrderMut<'a>: Iterator<Item = &'a mut Self::Node>
    where
        Self: 'a;

    type PreOrder<'a>: Iterator<Item = &'a Self::Node>
    where
        Self: 'a;

    type PreOrderMut<'a>: Iterator<Item = &'a mut Self::Node>
    where
        Self: 'a;

    type PostOrder<'a>: Iterator<Item = &'a Self::Node>
    where
        Self: 'a;

    type PostOrderMut<'a>: Iterator<Item = &'a mut Self::Node>
    where
        Self: 'a;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn height(&self) -> usize {
        if self.is_empty() {
            return 0;
        }
        let index = self.len() - 1;
        let index = Index::<N>::from_flattened(index);
        index.depth()
    }

    fn swap(&mut self, index_a: Index<N>, index_b: Index<N>) -> Option<()>;

    fn replace(&mut self, index: Index<N>, node: Self::Node) -> Option<Self::Node>;

    fn node(&self, index: Index<N>) -> Option<&Self::Node>;

    fn node_mut(&mut self, index: Index<N>) -> Option<&mut Self::Node>;

    fn root(&self) -> Option<&Self::Node> {
        let index = Index::root();
        self.node(index)
    }

    fn root_mut(&mut self) -> Option<&mut Self::Node> {
        let index = Index::root();
        self.node_mut(index)
    }

    fn last(&self) -> Option<&Self::Node> {
        if self.is_empty() {
            return None;
        }
        let index = self.len() - 1;
        let index = Index::<N>::from_flattened(index);
        self.node(index)
    }

    fn last_mut(&mut self) -> Option<&mut Self::Node> {
        if self.is_empty() {
            return None;
        }
        let index = self.len() - 1;
        let index = Index::<N>::from_flattened(index);
        self.node_mut(index)
    }

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

    fn iter_children(&self, index: Index<N>) -> Option<Self::IterChildren<'_>>;

    fn iter_children_mut(&mut self, index: Index<N>) -> Option<Self::IterChildrenMut<'_>>;

    fn iter_level(&self, depth: usize) -> Option<Self::IterLevel<'_>>;

    fn iter_level_mut(&mut self, depth: usize) -> Option<Self::IterLevelMut<'_>>;

    fn traverse_level_order(&self) -> Self::LevelOrder<'_>;

    fn traverse_level_order_mut(&mut self) -> Self::LevelOrderMut<'_>;

    fn traverse_pre_order(&self) -> Self::PreOrder<'_>;

    fn traverse_pre_order_mut(&mut self) -> Self::PreOrderMut<'_>;

    fn traverse_post_order(&self) -> Self::PostOrder<'_>;

    fn traverse_post_order_mut(&mut self) -> Self::PostOrderMut<'_>;
}

pub trait CompleteBinaryTree: CompleteTree<2> {
    type InOrder<'a>: Iterator<Item = &'a Self::Node>
    where
        Self: 'a;

    type InOrderMut<'a>: Iterator<Item = &'a mut Self::Node>
    where
        Self: 'a;

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

    fn traverse_in_order(&self) -> Self::InOrder<'_>;

    fn traverse_in_order_mut(&mut self) -> Self::InOrderMut<'_>;
}
