use crate::index::Index;
use alloc::vec::Vec;
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::ops::Range;

#[derive(Debug, Clone)]
pub struct PostOrder<'a, const N: usize, T> {
    indices: PostOrderIndices<N>,
    base: *const T,
    marker: PhantomData<&'a T>,
}

impl<'a, const N: usize, T> PostOrder<'a, N, T> {
    pub fn new(tree: &'a [T]) -> Self {
        let indices = PostOrderIndices::new(tree.len());
        let base = tree.as_ptr();
        let marker = PhantomData;
        Self {
            indices,
            base,
            marker,
        }
    }
}

impl<'a, const N: usize, T> Iterator for PostOrder<'a, N, T> {
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

impl<const N: usize, T> FusedIterator for PostOrder<'_, N, T> {}

#[derive(Debug)]
pub struct PostOrderMut<'a, const N: usize, T> {
    indices: PostOrderIndices<N>,
    base: *mut T,
    marker: PhantomData<&'a mut T>,
}

impl<'a, const N: usize, T> PostOrderMut<'a, N, T> {
    pub fn new(tree: &'a mut [T]) -> Self {
        let indices = PostOrderIndices::new(tree.len());
        let base = tree.as_mut_ptr();
        let marker = PhantomData;
        Self {
            indices,
            base,
            marker,
        }
    }
}

impl<'a, const N: usize, T> Iterator for PostOrderMut<'a, N, T> {
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

impl<const N: usize, T> FusedIterator for PostOrderMut<'_, N, T> {}

#[derive(Debug, Clone)]
struct PostOrderIndices<const N: usize> {
    stack: Vec<Frame<N>>,
    tree_len: usize,
}

impl<const N: usize> PostOrderIndices<N> {
    fn new(tree_len: usize) -> Self {
        if tree_len == 0 {
            let stack = Vec::new();
            return Self { stack, tree_len };
        }
        let last = tree_len - 1;
        let tree_height = Index::<N>::from_flattened(last).depth();
        let capacity = tree_height + 1;
        let mut stack = Vec::with_capacity(capacity);
        let index = const { Index::<N>::root().to_flattened() };
        let children = Index::<N>::root()
            .iter_children()
            .cap(tree_len)
            .to_flattened();
        let frame = Frame { index, children };
        stack.push(frame);
        Self { stack, tree_len }
    }
}

impl<const N: usize> Iterator for PostOrderIndices<N> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let frame = self.stack.last_mut()?;
            if let Some(child) = frame.children.next() {
                let grandchildren = Index::<N>::from_flattened(child)
                    .iter_children()
                    .cap(self.tree_len)
                    .to_flattened();
                let frame = Frame {
                    index: child,
                    children: grandchildren,
                };
                self.stack.push(frame);
                continue;
            }
            let frame = unsafe { self.stack.pop().unwrap_unchecked() };
            return Some(frame.index);
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.stack.len(), Some(self.tree_len))
    }
}

impl<const N: usize> FusedIterator for PostOrderIndices<N> {}

#[derive(Debug, Clone)]
struct Frame<const N: usize> {
    index: usize,
    children: Range<usize>,
}
