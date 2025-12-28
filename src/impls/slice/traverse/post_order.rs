use crate::Index;
use alloc::vec::Vec;
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::ops::Range;

#[derive(Debug, Clone)]
pub struct TraversePostOrder<'a, const N: usize, T> {
    stack: Vec<Frame<N>>,
    tree: &'a [T],
}

impl<'a, const N: usize, T> TraversePostOrder<'a, N, T> {
    pub fn new(tree: &'a [T]) -> Self {
        let mut stack = Vec::new();
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

impl<'a, const N: usize, T> Iterator for TraversePostOrder<'a, N, T> {
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

impl<const N: usize, T> FusedIterator for TraversePostOrder<'_, N, T> {}

#[derive(Debug)]
pub struct TraversePostOrderMut<'a, const N: usize, T> {
    stack: Vec<Frame<N>>,
    tree: *mut [T],
    marker: PhantomData<&'a mut T>,
}

impl<'a, const N: usize, T> TraversePostOrderMut<'a, N, T> {
    pub fn new(tree: &'a mut [T]) -> Self {
        let mut stack = Vec::new();
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

impl<'a, const N: usize, T> Iterator for TraversePostOrderMut<'a, N, T> {
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

impl<const N: usize, T> FusedIterator for TraversePostOrderMut<'_, N, T> {}

#[derive(Debug, Clone)]
struct Frame<const N: usize> {
    index: usize,
    children: Range<usize>,
}
