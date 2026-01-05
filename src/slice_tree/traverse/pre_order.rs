use crate::index::Index;
use alloc::vec::Vec;
use core::iter::FusedIterator;
use core::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct PreOrder<'a, const N: usize, T> {
    indices: PreOrderIndices<N>,
    base: *const T,
    marker: PhantomData<&'a T>,
}

impl<'a, const N: usize, T> PreOrder<'a, N, T> {
    pub fn new(tree: &'a [T]) -> Self {
        let indices = PreOrderIndices::new(tree.len());
        let base = tree.as_ptr();
        let marker = PhantomData;
        Self {
            indices,
            base,
            marker,
        }
    }
}

impl<'a, const N: usize, T> Iterator for PreOrder<'a, N, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.indices.next()?;
        let node = unsafe { &*self.base.add(index) };
        Some(node)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.indices.size_hint()
    }
}

impl<const N: usize, T> FusedIterator for PreOrder<'_, N, T> {}

#[derive(Debug)]
pub struct PreOrderMut<'a, const N: usize, T> {
    indices: PreOrderIndices<N>,
    base: *mut T,
    marker: PhantomData<&'a mut T>,
}

impl<'a, const N: usize, T> PreOrderMut<'a, N, T> {
    pub fn new(tree: &'a mut [T]) -> Self {
        let indices = PreOrderIndices::new(tree.len());
        let base = tree.as_mut_ptr();
        let marker = PhantomData;
        Self {
            indices,
            base,
            marker,
        }
    }
}

impl<'a, const N: usize, T> Iterator for PreOrderMut<'a, N, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.indices.next()?;
        let node = unsafe { &mut *self.base.add(index) };
        Some(node)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.indices.size_hint()
    }
}

impl<const N: usize, T> FusedIterator for PreOrderMut<'_, N, T> {}

#[derive(Debug, Clone)]
struct PreOrderIndices<const N: usize> {
    stack: Vec<usize>,
    tree_len: usize,
}

impl<const N: usize> PreOrderIndices<N> {
    fn new(tree_len: usize) -> Self {
        if tree_len == 0 {
            let stack = Vec::new();
            return Self { stack, tree_len };
        }

        let last = tree_len - 1;
        let tree_height = Index::<N>::from_flattened(last).depth();
        let capacity = tree_height.saturating_mul(N - 1).saturating_add(1);
        let mut stack = Vec::with_capacity(capacity);

        let root = const { Index::<N>::root().to_flattened() };
        stack.push(root);

        Self { stack, tree_len }
    }
}

impl<const N: usize> Iterator for PreOrderIndices<N> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.stack.pop()?;
        let children = Index::<N>::from_flattened(index)
            .iter_children()
            .cap(self.tree_len)
            .to_flattened();
        for child in children.rev() {
            self.stack.push(child);
        }
        Some(index)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.stack.len(), Some(self.tree_len))
    }
}

impl<const N: usize> FusedIterator for PreOrderIndices<N> {}
