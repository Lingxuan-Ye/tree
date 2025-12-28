use core::iter::FusedIterator;
use core::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Index<const N: usize> {
    depth: usize,
    offset: usize,
}

impl<const N: usize> Index<N> {
    pub const MIN: Self = Self::from_flattened(usize::MIN);
    pub const MAX: Self = Self::from_flattened(usize::MAX - 1);

    pub const fn new(depth: usize, offset: usize) -> Option<Self> {
        if depth < Self::MAX.depth && offset < N.pow(depth as u32)
            || (depth == Self::MAX.depth && offset <= Self::MAX.offset)
        {
            Some(Self { depth, offset })
        } else {
            None
        }
    }

    pub const fn root() -> Self {
        Self::MIN
    }

    pub const fn depth(&self) -> usize {
        self.depth
    }

    pub const fn offset(&self) -> usize {
        self.offset
    }

    pub const fn parent(&self) -> Option<Self> {
        if self.depth == Self::root().depth {
            return None;
        }

        let depth = self.depth - 1;
        let offset = self.offset / N;
        Some(Self { depth, offset })
    }

    pub const fn left_most_child(&self) -> Option<Self> {
        self.nth_child(0)
    }

    pub const fn right_most_child(&self) -> Option<Self> {
        self.nth_child(N - 1)
    }

    pub const fn nth_child(&self, n: usize) -> Option<Self> {
        if n >= N || self.depth == Self::MAX.depth {
            return None;
        }

        if self.depth == const { Self::MAX.depth - 1 } {
            let offset = N.saturating_mul(self.offset).saturating_add(n);
            if offset > Self::MAX.offset {
                return None;
            }
            let depth = Self::MAX.depth;
            return Some(Self { depth, offset });
        }

        let depth = self.depth + 1;
        let offset = N * self.offset + n;
        Some(Self { depth, offset })
    }

    pub fn iter_children(&self) -> IndexRange<N> {
        if self.depth == Self::MAX.depth {
            return IndexRange::empty();
        }

        if self.depth == const { Self::MAX.depth - 1 } {
            let offset = N.saturating_mul(self.offset);
            if offset > Self::MAX.offset {
                return IndexRange::empty();
            }
            let depth = Self::MAX.depth;
            let start = Self { depth, offset }.to_flattened();
            let end = offset.saturating_add(N).min(Self::MAX.offset + 1);
            return IndexRange::from_flattened(start..end);
        }

        let depth = self.depth + 1;
        let offset = N * self.offset;
        let start = Self { depth, offset }.to_flattened();
        let end = start + N;
        IndexRange::from_flattened(start..end)
    }

    pub const fn from_flattened(index: usize) -> Self {
        const { assert!(N != 0) }
        assert!(index < usize::MAX);

        match N {
            1 => {
                let depth = index;
                let offset = 0;
                Self { depth, offset }
            }

            2 => {
                let next = index + 1;
                let depth = (const { usize::BITS - 1 } - next.leading_zeros()) as usize;
                let offset = next - (1 << depth);
                Self { depth, offset }
            }

            _ => {
                let mut depth: usize = 0;
                let mut count: usize = 0;
                while let Some(width) = N.checked_pow(depth as u32) {
                    match count.checked_add(width) {
                        Some(next_count) if index >= next_count => {
                            depth += 1; // Will never overflow since `depth <= count`.
                            count = next_count;
                        }
                        _ => break,
                    }
                }
                let offset = index - count;
                Self { depth, offset }
            }
        }
    }

    pub const fn to_flattened(self) -> usize {
        match N {
            1 => self.depth,

            2 => (1 << self.depth) - 1 + self.offset,

            _ => {
                // Could cause intermediate overflow for large N.
                // ((N.pow(self.depth as u32) - 1) / (N - 1)) + self.offset

                let mut count = 0;
                let mut depth = 0;
                while depth < self.depth {
                    let width = N.pow(depth as u32);
                    count += width;
                    depth += 1;
                }
                count += self.offset;
                count
            }
        }
    }
}

impl Index<2> {
    pub const fn left_child(&self) -> Option<Self> {
        self.left_most_child()
    }

    pub const fn right_child(&self) -> Option<Self> {
        self.right_most_child()
    }
}

#[derive(Debug, Clone)]
pub struct IndexRange<const N: usize>(Range<usize>);

impl<const N: usize> IndexRange<N> {
    pub const fn empty() -> Self {
        Self::from_flattened(0..0)
    }

    pub const fn root() -> Self {
        Self::level(0)
    }

    pub const fn level(depth: usize) -> Self {
        if depth > Index::<N>::MAX.depth {
            return Self::empty();
        }

        if depth == Index::<N>::MAX.depth {
            return const {
                let depth = Index::<N>::MAX.depth;
                let start = Index::<N> { depth, offset: 0 }.to_flattened();
                let end = usize::MAX;
                Self::from_flattened(start..end)
            };
        }

        let start = Index::<N> { depth, offset: 0 }.to_flattened();
        let end = start + N.pow(depth as u32);
        Self::from_flattened(start..end)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub const fn cap(mut self, max: usize) -> Self {
        self.0.end = if self.0.end < max { self.0.end } else { max };
        self
    }

    pub const fn from_flattened(range: Range<usize>) -> Self {
        const { assert!(N != 0) }

        Self(range)
    }

    pub const fn to_flattened(&self) -> Range<usize> {
        self.0.start..self.0.end
    }
}

impl<const N: usize> Iterator for IndexRange<N> {
    type Item = Index<N>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.0.next()?;
        let index = Index::<N>::from_flattened(index);
        Some(index)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<const N: usize> ExactSizeIterator for IndexRange<N> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<const N: usize> DoubleEndedIterator for IndexRange<N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = self.0.next_back()?;
        let index = Index::<N>::from_flattened(index);
        Some(index)
    }
}

impl<const N: usize> FusedIterator for IndexRange<N> {}
