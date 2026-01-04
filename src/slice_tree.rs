use self::traverse::{InOrder, InOrderMut, PostOrder, PostOrderMut, PreOrder, PreOrderMut};
use crate::{CompleteBinaryTree, CompleteTree, Index, IndexRange};
use core::mem;
use core::ops::{Deref, DerefMut};
use core::ptr;
use core::slice::{Iter, IterMut};

pub mod traverse;

#[derive(Debug)]
#[repr(transparent)]
pub struct SliceTree<const N: usize, T>([T]);

impl<'a, const N: usize, T> From<&'a [T]> for &'a SliceTree<N, T> {
    fn from(value: &'a [T]) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl<'a, const N: usize, T> From<&'a mut [T]> for &'a mut SliceTree<N, T> {
    fn from(value: &'a mut [T]) -> Self {
        unsafe { mem::transmute(value) }
    }
}

impl<'a, const N: usize, T> From<&'a SliceTree<N, T>> for &'a [T] {
    fn from(value: &'a SliceTree<N, T>) -> Self {
        &value.0
    }
}

impl<'a, const N: usize, T> From<&'a mut SliceTree<N, T>> for &'a mut [T] {
    fn from(value: &'a mut SliceTree<N, T>) -> Self {
        &mut value.0
    }
}

impl<const N: usize, T> Deref for SliceTree<N, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize, T> DerefMut for SliceTree<N, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize, T> AsRef<[T]> for SliceTree<N, T> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}

impl<const N: usize, T> AsMut<[T]> for SliceTree<N, T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.0
    }
}

impl<const N: usize, T> CompleteTree<N> for SliceTree<N, T> {
    type Node = T;

    type IterChildren<'a>
        = Iter<'a, T>
    where
        Self: 'a;

    type IterChildrenMut<'a>
        = IterMut<'a, T>
    where
        Self: 'a;

    type IterLevel<'a>
        = Iter<'a, T>
    where
        Self: 'a;

    type IterLevelMut<'a>
        = IterMut<'a, T>
    where
        Self: 'a;

    type LevelOrder<'a>
        = Iter<'a, T>
    where
        Self: 'a;

    type LevelOrderMut<'a>
        = IterMut<'a, T>
    where
        Self: 'a;

    type PreOrder<'a>
        = PreOrder<'a, N, T>
    where
        Self: 'a;

    type PreOrderMut<'a>
        = PreOrderMut<'a, N, T>
    where
        Self: 'a;

    type PostOrder<'a>
        = PostOrder<'a, N, T>
    where
        Self: 'a;

    type PostOrderMut<'a>
        = PostOrderMut<'a, N, T>
    where
        Self: 'a;

    fn len(&self) -> usize {
        CompleteTree::<N>::len(self.as_ref())
    }

    fn swap(&mut self, index_a: Index<N>, index_b: Index<N>) -> Option<()> {
        CompleteTree::<N>::swap(self.as_mut(), index_a, index_b)
    }

    fn replace(&mut self, index: Index<N>, node: Self::Node) -> Option<Self::Node> {
        CompleteTree::<N>::replace(self.as_mut(), index, node)
    }

    fn node(&self, index: Index<N>) -> Option<&Self::Node> {
        CompleteTree::<N>::node(self.as_ref(), index)
    }

    fn node_mut(&mut self, index: Index<N>) -> Option<&mut Self::Node> {
        CompleteTree::<N>::node_mut(self.as_mut(), index)
    }

    fn iter_children(&self, index: Index<N>) -> Option<Self::IterChildren<'_>> {
        CompleteTree::<N>::iter_children(self.as_ref(), index)
    }

    fn iter_children_mut(&mut self, index: Index<N>) -> Option<Self::IterChildrenMut<'_>> {
        CompleteTree::<N>::iter_children_mut(self.as_mut(), index)
    }

    fn iter_level(&self, depth: usize) -> Option<Self::IterLevel<'_>> {
        CompleteTree::<N>::iter_level(self.as_ref(), depth)
    }

    fn iter_level_mut(&mut self, depth: usize) -> Option<Self::IterLevelMut<'_>> {
        CompleteTree::<N>::iter_level_mut(self.as_mut(), depth)
    }

    fn traverse_level_order(&self) -> Self::LevelOrder<'_> {
        CompleteTree::<N>::traverse_level_order(self.as_ref())
    }

    fn traverse_level_order_mut(&mut self) -> Self::LevelOrderMut<'_> {
        CompleteTree::<N>::traverse_level_order_mut(self.as_mut())
    }

    fn traverse_pre_order(&self) -> Self::PreOrder<'_> {
        CompleteTree::<N>::traverse_pre_order(self.as_ref())
    }

    fn traverse_pre_order_mut(&mut self) -> Self::PreOrderMut<'_> {
        CompleteTree::<N>::traverse_pre_order_mut(self.as_mut())
    }

    fn traverse_post_order(&self) -> Self::PostOrder<'_> {
        CompleteTree::<N>::traverse_post_order(self.as_ref())
    }

    fn traverse_post_order_mut(&mut self) -> Self::PostOrderMut<'_> {
        CompleteTree::<N>::traverse_post_order_mut(self.as_mut())
    }
}

impl<T> CompleteBinaryTree for SliceTree<2, T> {
    type InOrder<'a>
        = InOrder<'a, T>
    where
        Self: 'a;

    type InOrderMut<'a>
        = InOrderMut<'a, T>
    where
        Self: 'a;

    fn traverse_in_order(&self) -> InOrder<'_, T> {
        CompleteBinaryTree::traverse_in_order(self.as_ref())
    }

    fn traverse_in_order_mut(&mut self) -> InOrderMut<'_, T> {
        CompleteBinaryTree::traverse_in_order_mut(self.as_mut())
    }
}

impl<const N: usize, T> CompleteTree<N> for [T] {
    type Node = T;

    type IterChildren<'a>
        = Iter<'a, T>
    where
        Self: 'a;

    type IterChildrenMut<'a>
        = IterMut<'a, T>
    where
        Self: 'a;

    type IterLevel<'a>
        = Iter<'a, T>
    where
        Self: 'a;

    type IterLevelMut<'a>
        = IterMut<'a, T>
    where
        Self: 'a;

    type LevelOrder<'a>
        = Iter<'a, T>
    where
        Self: 'a;

    type LevelOrderMut<'a>
        = IterMut<'a, T>
    where
        Self: 'a;

    type PreOrder<'a>
        = PreOrder<'a, N, T>
    where
        Self: 'a;

    type PreOrderMut<'a>
        = PreOrderMut<'a, N, T>
    where
        Self: 'a;

    type PostOrder<'a>
        = PostOrder<'a, N, T>
    where
        Self: 'a;

    type PostOrderMut<'a>
        = PostOrderMut<'a, N, T>
    where
        Self: 'a;

    fn len(&self) -> usize {
        self.len()
    }

    fn swap(&mut self, index_a: Index<N>, index_b: Index<N>) -> Option<()> {
        let index_a = index_a.to_flattened();
        let index_b = index_b.to_flattened();
        if index_a >= self.len() || index_b >= self.len() {
            return None;
        }
        if index_a != index_b {
            let base = self.as_mut_ptr();
            unsafe {
                let ptr_a = base.add(index_a);
                let ptr_b = base.add(index_b);
                ptr::swap_nonoverlapping(ptr_a, ptr_b, 1);
            }
        }
        Some(())
    }

    fn replace(&mut self, index: Index<N>, node: Self::Node) -> Option<Self::Node> {
        let index = index.to_flattened();
        if index >= self.len() {
            return None;
        }
        let old = unsafe { self.get_unchecked_mut(index) };
        Some(mem::replace(old, node))
    }

    fn node(&self, index: Index<N>) -> Option<&Self::Node> {
        let index = index.to_flattened();
        self.get(index)
    }

    fn node_mut(&mut self, index: Index<N>) -> Option<&mut Self::Node> {
        let index = index.to_flattened();
        self.get_mut(index)
    }

    fn iter_children(&self, index: Index<N>) -> Option<Self::IterChildren<'_>> {
        if index.to_flattened() >= self.len() {
            return None;
        }
        let children = index.iter_children().cap(self.len()).to_flattened();
        self.get(children).map(Self::iter)
    }

    fn iter_children_mut(&mut self, index: Index<N>) -> Option<Self::IterChildrenMut<'_>> {
        if index.to_flattened() >= self.len() {
            return None;
        }
        let children = index.iter_children().cap(self.len()).to_flattened();
        self.get_mut(children).map(Self::iter_mut)
    }

    fn iter_level(&self, depth: usize) -> Option<Self::IterLevel<'_>> {
        if depth > CompleteTree::<N>::height(self) {
            return None;
        }
        let level = IndexRange::<N>::level(depth).cap(self.len()).to_flattened();
        self.get(level).map(Self::iter)
    }

    fn iter_level_mut(&mut self, depth: usize) -> Option<Self::IterLevelMut<'_>> {
        if depth > CompleteTree::<N>::height(self) {
            return None;
        }
        let level = IndexRange::<N>::level(depth).cap(self.len()).to_flattened();
        self.get_mut(level).map(Self::iter_mut)
    }

    fn traverse_level_order(&self) -> Self::LevelOrder<'_> {
        self.iter()
    }

    fn traverse_level_order_mut(&mut self) -> Self::LevelOrderMut<'_> {
        self.iter_mut()
    }

    fn traverse_pre_order(&self) -> Self::PreOrder<'_> {
        PreOrder::<N, T>::new(self)
    }

    fn traverse_pre_order_mut(&mut self) -> Self::PreOrderMut<'_> {
        PreOrderMut::<N, T>::new(self)
    }

    fn traverse_post_order(&self) -> Self::PostOrder<'_> {
        PostOrder::<N, T>::new(self)
    }

    fn traverse_post_order_mut(&mut self) -> Self::PostOrderMut<'_> {
        PostOrderMut::<N, T>::new(self)
    }
}

impl<T> CompleteBinaryTree for [T] {
    type InOrder<'a>
        = InOrder<'a, T>
    where
        Self: 'a;

    type InOrderMut<'a>
        = InOrderMut<'a, T>
    where
        Self: 'a;

    fn traverse_in_order(&self) -> InOrder<'_, T> {
        InOrder::new(self)
    }

    fn traverse_in_order_mut(&mut self) -> InOrderMut<'_, T> {
        InOrderMut::new(self)
    }
}
