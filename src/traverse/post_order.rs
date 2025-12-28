use crate::{CompleteTree, Index, IndexRange};
use alloc::vec::Vec;
use core::iter::FusedIterator;

#[derive(Debug, Clone)]
pub struct TraversePostOrder<'a, const N: usize, T>
where
    T: CompleteTree<N> + ?Sized,
{
    tree: &'a T,
    stack: Vec<Frame<N>>,
}

impl<'a, const N: usize, T> TraversePostOrder<'a, N, T>
where
    T: CompleteTree<N> + ?Sized,
{
    pub fn new(tree: &'a T) -> Self {
        let mut stack = Vec::new();
        if !tree.is_empty() {
            let index = Index::root();
            let children = index.iter_children().cap(tree.len());
            let frame = Frame { index, children };
            stack.push(frame);
        }
        Self { tree, stack }
    }
}

impl<'a, const N: usize, T> Iterator for TraversePostOrder<'a, N, T>
where
    T: CompleteTree<N> + ?Sized,
{
    type Item = &'a T::Node;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let frame = self.stack.last_mut()?;
            if let Some(child) = frame.children.next() {
                let grandchildren = child.iter_children().cap(self.tree.len());
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
            return self.tree.get(frame.index);
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.stack.len(), Some(self.tree.len()))
    }
}

impl<const N: usize, T> FusedIterator for TraversePostOrder<'_, N, T> where
    T: CompleteTree<N> + ?Sized
{
}

#[derive(Debug, Clone)]
struct Frame<const N: usize> {
    index: Index<N>,
    children: IndexRange<N>,
}
