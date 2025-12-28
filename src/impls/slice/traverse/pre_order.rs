use super::super::Index;
use alloc::vec::Vec;
use core::iter::FusedIterator;
use core::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct TraversePreOrder<'a, const N: usize, T> {
    tree: &'a [T],
    stack: Vec<usize>,
}

impl<'a, const N: usize, T> TraversePreOrder<'a, N, T> {
    pub fn new(tree: &'a [T]) -> Self {
        let mut stack = Vec::new();
        if !tree.is_empty() {
            stack.push(const { Index::<N>::root().to_flattened() });
        }
        Self { tree, stack }
    }
}

impl<'a, const N: usize, T> Iterator for TraversePreOrder<'a, N, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.stack.pop()?;
        let children = Index::<N>::from_flattened(index)
            .iter_children()
            .cap(self.tree.len())
            .to_flattened();
        for child in children.rev() {
            self.stack.push(child);
        }
        Some(unsafe { self.tree.get_unchecked(index) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.stack.len(), Some(self.tree.len()))
    }
}

impl<const N: usize, T> FusedIterator for TraversePreOrder<'_, N, T> {}

#[derive(Debug)]
pub struct TraversePreOrderMut<'a, const N: usize, T> {
    tree: *mut [T],
    stack: Vec<usize>,
    marker: PhantomData<&'a mut T>,
}

impl<'a, const N: usize, T> TraversePreOrderMut<'a, N, T> {
    pub fn new(tree: &'a mut [T]) -> Self {
        let tree = tree as *mut [T];
        let mut stack = Vec::new();
        if !tree.is_empty() {
            stack.push(const { Index::<N>::root().to_flattened() });
        }
        let marker = PhantomData;
        Self {
            tree,
            stack,
            marker,
        }
    }
}

impl<'a, const N: usize, T> Iterator for TraversePreOrderMut<'a, N, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.stack.pop()?;
        let children = Index::<N>::from_flattened(index)
            .iter_children()
            .cap(self.tree.len())
            .to_flattened();
        for child in children.rev() {
            self.stack.push(child);
        }
        Some(unsafe { (&mut *self.tree).get_unchecked_mut(index) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.stack.len(), Some(self.tree.len()))
    }
}

impl<const N: usize, T> FusedIterator for TraversePreOrderMut<'_, N, T> {}
