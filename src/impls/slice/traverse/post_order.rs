use crate::{CompleteTree, Index};
use alloc::vec::Vec;
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::ops::Range;

#[derive(Debug, Clone)]
pub(in super::super) struct PostOrder<'a, const N: usize, T> {
    stack: Vec<Frame<N>>,
    tree: &'a [T],
}

impl<'a, const N: usize, T> PostOrder<'a, N, T> {
    pub(in super::super) fn new(tree: &'a [T]) -> Self {
        let capacity = CompleteTree::<N>::height(tree) + 1;
        let mut stack = Vec::with_capacity(capacity);
        if !tree.is_empty() {
            let index = const { Index::<N>::root().to_flattened() };
            let children = Index::<N>::root()
                .iter_children()
                .cap(tree.len())
                .to_flattened();
            let frame = Frame { index, children };
            stack.push(frame);
        }
        Self { stack, tree }
    }
}

impl<'a, const N: usize, T> Iterator for PostOrder<'a, N, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let frame = self.stack.last_mut()?;
            if let Some(child) = frame.children.next() {
                let grandchildren = Index::<N>::from_flattened(child)
                    .iter_children()
                    .cap(self.tree.len())
                    .to_flattened();
                let frame = Frame {
                    index: child,
                    children: grandchildren,
                };
                self.stack.push(frame);
                continue;
            }
            let frame = unsafe { self.stack.pop().unwrap_unchecked() };
            return Some(unsafe { self.tree.get_unchecked(frame.index) });
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.stack.len(), Some(self.tree.len()))
    }
}

impl<const N: usize, T> FusedIterator for PostOrder<'_, N, T> {}

#[derive(Debug)]
pub(in super::super) struct PostOrderMut<'a, const N: usize, T> {
    stack: Vec<Frame<N>>,
    tree: *mut [T],
    marker: PhantomData<&'a mut T>,
}

impl<'a, const N: usize, T> PostOrderMut<'a, N, T> {
    pub(in super::super) fn new(tree: &'a mut [T]) -> Self {
        let capacity = CompleteTree::<N>::height(tree) + 1;
        let mut stack = Vec::with_capacity(capacity);
        if !tree.is_empty() {
            let index = const { Index::<N>::root().to_flattened() };
            let children = Index::<N>::root()
                .iter_children()
                .cap(tree.len())
                .to_flattened();
            let frame = Frame { index, children };
            stack.push(frame);
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

impl<'a, const N: usize, T> Iterator for PostOrderMut<'a, N, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let frame = self.stack.last_mut()?;
            if let Some(child) = frame.children.next() {
                let grandchildren = Index::<N>::from_flattened(child)
                    .iter_children()
                    .cap(self.tree.len())
                    .to_flattened();
                let frame = Frame {
                    index: child,
                    children: grandchildren,
                };
                self.stack.push(frame);
                continue;
            }
            let frame = unsafe { self.stack.pop().unwrap_unchecked() };
            return Some(unsafe { (&mut *self.tree).get_unchecked_mut(frame.index) });
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.stack.len(), Some(self.tree.len()))
    }
}

impl<const N: usize, T> FusedIterator for PostOrderMut<'_, N, T> {}

#[derive(Debug, Clone)]
struct Frame<const N: usize> {
    index: usize,
    children: Range<usize>,
}
