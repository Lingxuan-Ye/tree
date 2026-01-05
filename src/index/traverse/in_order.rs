use crate::Index;
use alloc::vec::Vec;
use core::iter::FusedIterator;

#[derive(Debug, Clone)]
pub struct InOrder {
    state: State,
    stack: Vec<Index<2>>,
    tree_len: usize,
}

impl InOrder {
    pub fn new(tree_len: usize) -> Self {
        if tree_len == 0 {
            let state = State::Done;
            let stack = Vec::new();
            return Self {
                state,
                stack,
                tree_len,
            };
        }

        let root = Index::root();
        let state = State::Push(root);

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

impl Iterator for InOrder {
    type Item = Index<2>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                State::Push(index) => {
                    if let Some(left_child) = index.left_child()
                        && left_child.to_flattened() < self.tree_len
                    {
                        self.state = State::Push(left_child);
                        self.stack.push(index);
                        continue;
                    }
                    self.state = State::Pop;
                    return Some(index);
                }

                State::Pop => {
                    if let Some(index) = self.stack.pop() {
                        if let Some(right_child) = index.right_child()
                            && right_child.to_flattened() < self.tree_len
                        {
                            self.state = State::Push(right_child);
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

impl FusedIterator for InOrder {}

#[derive(Debug, Clone)]
enum State {
    Push(Index<2>),
    Pop,
    Done,
}
