use crate::{CompleteTree, Index};
use alloc::vec::Vec;
use core::iter::FusedIterator;
use core::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct TraverseInOrder<'a, T> {
    state: State,
    stack: Vec<Index<2>>,
    tree: &'a [T],
}

impl<'a, T> TraverseInOrder<'a, T> {
    pub fn new(tree: &'a [T]) -> Self {
        let state = if tree.is_empty() {
            State::Done
        } else {
            State::Left(Index::root())
        };
        let capacity = CompleteTree::<2>::height(tree);
        let stack = Vec::with_capacity(capacity);
        Self { state, stack, tree }
    }
}

impl<'a, T> Iterator for TraverseInOrder<'a, T> {
    type Item = &'a T;

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
                        let index = index.to_flattened();
                        return Some(unsafe { self.tree.get_unchecked(index) });
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
                    let index = unsafe { self.stack.pop().unwrap_unchecked() };
                    self.state = State::Right(index);
                    let index = index.to_flattened();
                    return Some(unsafe { self.tree.get_unchecked(index) });
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

impl<T> FusedIterator for TraverseInOrder<'_, T> {}

#[derive(Debug, Clone)]
pub struct TraverseInOrderMut<'a, T> {
    state: State,
    stack: Vec<Index<2>>,
    tree: *mut [T],
    marker: PhantomData<&'a mut T>,
}

impl<'a, T> TraverseInOrderMut<'a, T> {
    pub fn new(tree: &'a mut [T]) -> Self {
        let state = if tree.is_empty() {
            State::Done
        } else {
            State::Left(Index::root())
        };
        let capacity = CompleteTree::<2>::height(tree);
        let stack = Vec::with_capacity(capacity);
        let tree = tree as *mut [T];
        let marker = PhantomData;
        Self {
            state,
            stack,
            tree,
            marker,
        }
    }
}

impl<'a, T> Iterator for TraverseInOrderMut<'a, T> {
    type Item = &'a mut T;

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
                        let index = index.to_flattened();
                        return Some(unsafe { (&mut *self.tree).get_unchecked_mut(index) });
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
                    let index = unsafe { self.stack.pop().unwrap_unchecked() };
                    self.state = State::Right(index);
                    let index = index.to_flattened();
                    return Some(unsafe { (&mut *self.tree).get_unchecked_mut(index) });
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

impl<T> FusedIterator for TraverseInOrderMut<'_, T> {}

#[derive(Debug, Clone)]
enum State {
    Left(Index<2>),
    Right(Index<2>),
    Pop,
    Done,
}
