use crate::{CompleteTree, Index};
use alloc::vec::Vec;
use core::iter::FusedIterator;
use core::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct PreOrder<'a, const N: usize, T> {
    stack: Vec<usize>,
    tree: &'a [T],
}

impl<'a, const N: usize, T> PreOrder<'a, N, T> {
    pub fn new(tree: &'a [T]) -> Self {
        let capacity = CompleteTree::<N>::height(tree)
            .saturating_mul(N - 1)
            .saturating_add(1);
        let mut stack = Vec::with_capacity(capacity);
        if !tree.is_empty() {
            stack.push(const { Index::<N>::root().to_flattened() });
        }
        Self { stack, tree }
    }
}

impl<'a, const N: usize, T> Iterator for PreOrder<'a, N, T> {
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

impl<const N: usize, T> FusedIterator for PreOrder<'_, N, T> {}

#[derive(Debug)]
pub struct PreOrderMut<'a, const N: usize, T> {
    stack: Vec<usize>,
    tree: *mut [T],
    marker: PhantomData<&'a mut T>,
}

impl<'a, const N: usize, T> PreOrderMut<'a, N, T> {
    pub fn new(tree: &'a mut [T]) -> Self {
        let capacity = CompleteTree::<N>::height(tree)
            .saturating_mul(N - 1)
            .saturating_add(1);
        let mut stack = Vec::with_capacity(capacity);
        if !tree.is_empty() {
            stack.push(const { Index::<N>::root().to_flattened() });
        }
        let tree = tree as *mut [T];
        let marker = PhantomData;
        Self {
            stack,
            tree,
            marker,
        }
    }
}

impl<'a, const N: usize, T> Iterator for PreOrderMut<'a, N, T> {
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

impl<const N: usize, T> FusedIterator for PreOrderMut<'_, N, T> {}
