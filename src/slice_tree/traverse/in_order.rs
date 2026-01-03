use crate::index::Index;
use alloc::vec::Vec;
use core::iter::FusedIterator;
use core::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct InOrder<'a, T> {
    indices: InOrderIndices,
    root: *const T,
    marker: PhantomData<&'a T>,
}

impl<'a, T> InOrder<'a, T> {
    pub fn new(tree: &'a [T]) -> Self {
        let indices = InOrderIndices::new(tree.len());
        let root = tree.as_ptr();
        let marker = PhantomData;
        Self {
            indices,
            root,
            marker,
        }
    }
}

impl<'a, T> Iterator for InOrder<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.indices.next()?;
        let node = unsafe { &*self.root.add(index) };
        Some(node)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.indices.size_hint()
    }
}

impl<T> FusedIterator for InOrder<'_, T> {}

#[derive(Debug)]
pub struct InOrderMut<'a, T> {
    indices: InOrderIndices,
    root: *mut T,
    marker: PhantomData<&'a mut T>,
}

impl<'a, T> InOrderMut<'a, T> {
    pub fn new(tree: &'a mut [T]) -> Self {
        let indices = InOrderIndices::new(tree.len());
        let root = tree.as_mut_ptr();
        let marker = PhantomData;
        Self {
            indices,
            root,
            marker,
        }
    }
}

impl<'a, T> Iterator for InOrderMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.indices.next()?;
        let node = unsafe { &mut *self.root.add(index) };
        Some(node)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.indices.size_hint()
    }
}

impl<T> FusedIterator for InOrderMut<'_, T> {}

#[derive(Debug, Clone)]
struct InOrderIndices {
    state: State,
    stack: Vec<usize>,
    tree_len: usize,
}

impl InOrderIndices {
    fn new(tree_len: usize) -> Self {
        if tree_len == 0 {
            let state = State::Done;
            let stack = Vec::new();
            return Self {
                state,
                stack,
                tree_len,
            };
        }
        let index = const { Index::<2>::root().to_flattened() };
        let state = State::Push(index);
        let last = tree_len - 1;
        let tree_height = Index::<2>::from_flattened(last).depth();
        let capacity = tree_height;
        let stack = Vec::with_capacity(capacity);
        Self {
            state,
            stack,
            tree_len,
        }
    }
}

impl Iterator for InOrderIndices {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                State::Push(index) => {
                    if let Some(left_child) = Index::from_flattened(index).left_child() {
                        let left_child = left_child.to_flattened();
                        if left_child < self.tree_len {
                            self.state = State::Push(left_child);
                            self.stack.push(index);
                            continue;
                        }
                    }
                    self.state = State::Pop;
                    return Some(index);
                }

                State::Pop => {
                    if let Some(index) = self.stack.pop() {
                        if let Some(right_child) = Index::from_flattened(index).right_child() {
                            let right_child = right_child.to_flattened();
                            if right_child < self.tree_len {
                                self.state = State::Push(right_child);
                            }
                        }
                        return Some(index);
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
        (self.stack.len(), Some(self.tree_len))
    }
}

impl FusedIterator for InOrderIndices {}

#[derive(Debug, Clone)]
enum State {
    Push(usize),
    Pop,
    Done,
}
