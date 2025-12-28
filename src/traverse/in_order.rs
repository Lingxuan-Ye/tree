use crate::{CompleteTree, Index};
use alloc::vec::Vec;
use core::iter::FusedIterator;

#[derive(Debug, Clone)]
pub struct TraverseInOrder<'a, T>
where
    T: CompleteTree<2> + ?Sized,
{
    tree: &'a T,
    stack: Vec<Index<2>>,
    state: State,
}

impl<'a, T> TraverseInOrder<'a, T>
where
    T: CompleteTree<2> + ?Sized,
{
    pub fn new(tree: &'a T) -> Self {
        let stack = Vec::new();
        let state = if tree.is_empty() {
            State::Done
        } else {
            State::Left(Index::root())
        };
        Self { tree, stack, state }
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
                State::Left(index) => {
                    if let Some(left_child) = index.left_child()
                        && left_child.to_flattened() < self.tree.len()
                    {
                        self.stack.push(index);
                        self.state = State::Left(left_child);
                    } else {
                        self.state = State::Right(index);
                        return self.tree.get(index);
                    }
                }

                State::Right(index) => {
                    if let Some(right_child) = index.right_child()
                        && right_child.to_flattened() < self.tree.len()
                    {
                        self.state = State::Left(right_child);
                    } else if self.stack.is_empty() {
                        self.state = State::Done
                    } else {
                        self.state = State::Pop
                    }
                }

                State::Pop => {
                    let Some(index) = self.stack.pop() else {
                        unreachable!()
                    };
                    self.state = State::Right(index);
                    return self.tree.get(index);
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
    Left(Index<2>),
    Right(Index<2>),
    Pop,
    Done,
}
