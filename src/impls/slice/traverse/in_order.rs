use crate::{CompleteTree, Index};
use alloc::vec::Vec;
use core::iter::FusedIterator;
use core::marker::PhantomData;

#[derive(Debug, Clone)]
pub(in super::super) struct TraverseInOrder<'a, T> {
    state: State,
    stack: Vec<Index<2>>,
    tree: &'a [T],
}

impl<'a, T> TraverseInOrder<'a, T> {
    pub(in super::super) fn new(tree: &'a [T]) -> Self {
        let state = if tree.is_empty() {
            State::Done
        } else {
            State::Push(Index::root())
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
                State::Push(index) => {
                    if let Some(left_child) = index.left_child()
                        && left_child.to_flattened() < self.tree.len()
                    {
                        self.state = State::Push(left_child);
                        self.stack.push(index);
                        continue;
                    }
                    self.state = State::Pop;
                    let index = index.to_flattened();
                    return Some(unsafe { self.tree.get_unchecked(index) });
                }

                State::Pop => {
                    if let Some(index) = self.stack.pop() {
                        if let Some(right_child) = index.right_child()
                            && right_child.to_flattened() < self.tree.len()
                        {
                            self.state = State::Push(right_child);
                        }
                        let index = index.to_flattened();
                        return Some(unsafe { self.tree.get_unchecked(index) });
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

impl<T> FusedIterator for TraverseInOrder<'_, T> {}

#[derive(Debug, Clone)]
pub(in super::super) struct TraverseInOrderMut<'a, T> {
    state: State,
    stack: Vec<Index<2>>,
    tree: *mut [T],
    marker: PhantomData<&'a mut T>,
}

impl<'a, T> TraverseInOrderMut<'a, T> {
    pub(in super::super) fn new(tree: &'a mut [T]) -> Self {
        let state = if tree.is_empty() {
            State::Done
        } else {
            State::Push(Index::root())
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
                State::Push(index) => {
                    if let Some(left_child) = index.left_child()
                        && left_child.to_flattened() < self.tree.len()
                    {
                        self.state = State::Push(left_child);
                        self.stack.push(index);
                        continue;
                    }
                    self.state = State::Pop;
                    let index = index.to_flattened();
                    return Some(unsafe { (&mut *self.tree).get_unchecked_mut(index) });
                }

                State::Pop => {
                    if let Some(index) = self.stack.pop() {
                        if let Some(right_child) = index.right_child()
                            && right_child.to_flattened() < self.tree.len()
                        {
                            self.state = State::Push(right_child);
                        }
                        let index = index.to_flattened();
                        return Some(unsafe { (&mut *self.tree).get_unchecked_mut(index) });
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

impl<T> FusedIterator for TraverseInOrderMut<'_, T> {}

#[derive(Debug, Clone)]
enum State {
    Push(Index<2>),
    Pop,
    Done,
}
