use crate::{Index, IndexRange};
use alloc::vec::Vec;
use core::iter::FusedIterator;

#[derive(Debug, Clone)]
pub struct PostOrder<const N: usize> {
    stack: Vec<Frame<N>>,
    tree_len: usize,
}

impl<const N: usize> PostOrder<N> {
    pub fn new(tree_len: usize) -> Self {
        if tree_len == 0 {
            let stack = Vec::new();
            return Self { stack, tree_len };
        }
        let last = tree_len - 1;
        let tree_height = Index::<N>::from_flattened(last).depth();
        let capacity = tree_height + 1;
        let mut stack = Vec::with_capacity(capacity);
        let index = Index::root();
        let children = index.iter_children().cap(tree_len);
        let frame = Frame { index, children };
        stack.push(frame);
        Self { stack, tree_len }
    }
}

impl<const N: usize> Iterator for PostOrder<N> {
    type Item = Index<N>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let frame = self.stack.last_mut()?;
            if let Some(child) = frame.children.next() {
                let grandchildren = child.iter_children().cap(self.tree_len);
                let frame = Frame {
                    index: child,
                    children: grandchildren,
                };
                self.stack.push(frame);
                continue;
            }
            let Some(frame) = self.stack.pop() else {
                unreachable!()
            };
            return Some(frame.index);
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.stack.len(), Some(self.tree_len))
    }
}

impl<const N: usize> FusedIterator for PostOrder<N> {}

#[derive(Debug, Clone)]
struct Frame<const N: usize> {
    index: Index<N>,
    children: IndexRange<N>,
}
