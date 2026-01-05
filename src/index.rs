use core::iter::FusedIterator;
use core::ops::RangeInclusive;

pub mod traverse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Index<const N: usize> {
    depth: usize,
    offset: usize,
}

impl<const N: usize> Index<N> {
    pub const MIN: Self = Self::from_flattened(usize::MIN);
    pub const MAX: Self = Self::from_flattened(usize::MAX);

    pub const fn depth(&self) -> usize {
        self.depth
    }

    pub const fn offset(&self) -> usize {
        self.offset
    }

    pub const fn new(depth: usize, offset: usize) -> Option<Self> {
        if depth == Self::MAX.depth && offset <= Self::MAX.offset
            || depth < Self::MAX.depth && offset < N.pow(depth as u32)
        {
            Some(Self { depth, offset })
        } else {
            None
        }
    }

    pub const fn root() -> Self {
        Self::MIN
    }

    pub const fn parent(&self) -> Option<Self> {
        if self.depth == Self::MIN.depth {
            return None;
        }

        let depth = self.depth - 1;
        let offset = self.offset / N;
        Some(Self { depth, offset })
    }

    pub const fn first_child(&self) -> Option<Self> {
        self.child(0)
    }

    pub const fn last_child(&self) -> Option<Self> {
        self.child(N - 1)
    }

    pub const fn child(&self, n: usize) -> Option<Self> {
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
            let end = offset.saturating_add(N - 1).min(Self::MAX.offset);
            return IndexRange::from_flattened(start..=end);
        }

        let depth = self.depth + 1;
        let offset = N * self.offset;
        let start = Self { depth, offset }.to_flattened();
        let end = start + N - 1;
        IndexRange::from_flattened(start..=end)
    }

    pub const fn from_flattened(index: usize) -> Self {
        const { assert!(N != 0) }

        match N {
            1 => {
                let depth = index;
                let offset = 0;
                Self { depth, offset }
            }

            2 => {
                if index == usize::MAX {
                    let depth = usize::BITS as usize;
                    let offset = 0;
                    return Self { depth, offset };
                }
                let next = index + 1;
                let depth = (const { usize::BITS - 1 } - next.leading_zeros()) as usize;
                let offset = next - (1 << depth);
                Self { depth, offset }
            }

            _ => {
                let mut count: usize = 0;
                let mut depth: usize = 0;
                while let Some(width) = N.checked_pow(depth as u32)
                    && let Some(next_count) = count.checked_add(width)
                    && index >= next_count
                {
                    count = next_count;
                    depth += 1;
                }
                let offset = index - count;
                Self { depth, offset }
            }
        }
    }

    pub const fn to_flattened(self) -> usize {
        match N {
            1 => self.depth,

            2 => {
                if self.depth == usize::BITS as usize {
                    usize::MAX
                } else {
                    (1 << self.depth) - 1 + self.offset
                }
            }

            _ => {
                // `((N.pow(depth) - 1) / (N - 1)) + offset` may overflow for large `N`.

                let mut count = 0;
                let mut depth = 0;
                while depth < self.depth {
                    let width = N.pow(depth as u32);
                    count += width;
                    depth += 1;
                }
                count + self.offset
            }
        }
    }
}

impl Index<2> {
    pub const fn left_child(&self) -> Option<Self> {
        self.first_child()
    }

    pub const fn right_child(&self) -> Option<Self> {
        self.last_child()
    }
}

#[derive(Debug, Clone)]
pub struct IndexRange<const N: usize>(RangeInclusive<usize>);

impl<const N: usize> IndexRange<N> {
    pub fn len(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            self.0.end() - self.0.start() + 1
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub const fn empty() -> Self {
        let start = usize::MAX;
        let end = usize::MIN;
        Self::from_flattened(start..=end)
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
                let offset = 0;
                let start = Index::<N> { depth, offset }.to_flattened();
                let end = usize::MAX;
                Self::from_flattened(start..=end)
            };
        }

        let offset = 0;
        let start = Index::<N> { depth, offset }.to_flattened();
        let end = start + N.pow(depth as u32) - 1;
        Self::from_flattened(start..=end)
    }

    pub const fn cap(self, upper: usize) -> Self {
        let start = *self.0.start();
        let end = *self.0.end();
        let end = if end < upper { end } else { upper - 1 };
        Self::from_flattened(start..=end)
    }

    const fn from_flattened(range: RangeInclusive<usize>) -> Self {
        const { assert!(N != 0) }

        debug_assert!(
            !(*range.start() == 0 && *range.end() == usize::MAX),
            "invalid range"
        );

        Self(range)
    }

    pub const fn to_flattened(&self) -> RangeInclusive<usize> {
        let start = *self.0.start();
        let end = *self.0.end();
        start..=end
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
        let len = self.len();
        (len, Some(len))
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
