use super::super::CompleteTree;
use super::super::index::{Index, IndexRange};
use alloc::vec::Vec;
use core::iter::FusedIterator;

#[derive(Debug, Clone)]
pub struct TraversePostOrder<'a, const N: usize, T>
where
    T: CompleteTree<N> + ?Sized,
{
    tree: &'a T,
    len: usize,
    stack: Vec<Frame<N>>,
}

impl<'a, const N: usize, T> TraversePostOrder<'a, N, T>
where
    T: CompleteTree<N> + ?Sized,
{
    pub fn new(tree: &'a T) -> Self {
        let len = tree.len();
        let mut stack = Vec::new();
        if !tree.is_empty() {
            let index = Index::root();
            let children = index.iter_children().cap(tree.len());
            stack.push(Frame { index, children });
        }
        Self { tree, len, stack }
    }
}

impl<'a, const N: usize, T> Iterator for TraversePostOrder<'a, N, T>
where
    T: CompleteTree<N> + ?Sized,
{
    type Item = &'a T::Node;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mut frame) = self.stack.pop() {
            if let Some(child) = frame.children.next() {
                self.stack.push(frame);
                let grandchildren = child.iter_children().cap(self.tree.len());
                self.stack.push(Frame {
                    index: child,
                    children: grandchildren,
                });
                continue;
            }
            self.len -= 1;
            return self.tree.get(frame.index);
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<const N: usize, T> ExactSizeIterator for TraversePostOrder<'_, N, T>
where
    T: CompleteTree<N> + ?Sized,
{
    fn len(&self) -> usize {
        self.len
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
