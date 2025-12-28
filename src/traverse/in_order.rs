use crate::{CompleteTree, Index};
use alloc::vec::Vec;
use core::iter::FusedIterator;

#[derive(Debug, Clone)]
pub struct TraverseInOrder<'a, T>
where
    T: CompleteTree<2> + ?Sized,
{
    state: State,
    stack: Vec<Index<2>>,
    tree: &'a T,
}

impl<'a, T> TraverseInOrder<'a, T>
where
    T: CompleteTree<2> + ?Sized,
{
    pub fn new(tree: &'a T) -> Self {
        let state = if tree.is_empty() {
            State::Done
        } else {
            State::Push(Index::root())
        };
        let capacity = tree.height();
        let stack = Vec::with_capacity(capacity);
        Self { state, stack, tree }
    }
}

impl<'a, T> Iterator for TraverseInOrder<'a, T>
where
    T: CompleteTree<2> + ?Sized,
{
    type Item = &'a T::Node;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                State::Push(index) => {
                    if let Some(left_child) = index.left_child()
                        && left_child.to_flattened() < self.tree.len()
                    {
                        self.state = State::Push(left_child);
                        self.stack.push(index);
                        continue;
                    }
                    self.state = State::Pop;
                    return self.tree.get(index);
                }

                State::Pop => {
                    if let Some(index) = self.stack.pop() {
                        if let Some(right_child) = index.right_child()
                            && right_child.to_flattened() < self.tree.len()
                        {
                            self.state = State::Push(right_child);
                        }
                        return self.tree.get(index);
                    }
                    self.state = State::Done;
                    return None;
                }

                State::Done => {
                    return None;
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.stack.len(), Some(self.tree.len()))
    }
}

impl<T> FusedIterator for TraverseInOrder<'_, T> where T: CompleteTree<2> + ?Sized {}

#[derive(Debug, Clone)]
enum State {
    Push(Index<2>),
    Pop,
    Done,
}
