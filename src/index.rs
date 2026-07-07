use core::iter::FusedIterator;
use core::range::RangeInclusive;

pub mod traverse;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Index<const N: usize> {
    depth: usize,
    offset: usize,
}

impl<const N: usize> Index<N> {
    pub const MIN: Self = Self::from_linear(usize::MIN);
    pub const MAX: Self = Self::from_linear(usize::MAX);

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

    pub const fn iter_children(&self) -> IndexRange<N> {
        if self.depth == Self::MAX.depth {
            return IndexRange::empty();
        }

        if self.depth == const { Self::MAX.depth - 1 } {
            let offset = N.saturating_mul(self.offset);
            if offset > Self::MAX.offset {
                return IndexRange::empty();
            }
            let depth = Self::MAX.depth;
            let start = Self { depth, offset }.to_linear();
            let mut last = offset.saturating_add(N - 1);
            if last > Self::MAX.offset {
                last = Self::MAX.offset;
            }
            let range = RangeInclusive { start, last };
            return IndexRange::from_linear(range);
        }

        let depth = self.depth + 1;
        let offset = N * self.offset;
        let start = Self { depth, offset }.to_linear();
        let last = start + N - 1;
        let range = RangeInclusive { start, last };
        IndexRange::from_linear(range)
    }

    pub const fn from_linear(index: usize) -> Self {
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

    pub const fn to_linear(self) -> usize {
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
            self.0.last - self.0.start + 1
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub const fn empty() -> Self {
        let start = usize::MAX;
        let last = usize::MIN;
        let range = RangeInclusive { start, last };
        Self::from_linear(range)
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
                let start = Index::<N> { depth, offset }.to_linear();
                let last = usize::MAX;
                let range = RangeInclusive { start, last };
                Self::from_linear(range)
            };
        }

        let offset = 0;
        let start = Index::<N> { depth, offset }.to_linear();
        let last = start + N.pow(depth as u32) - 1;
        let range = RangeInclusive { start, last };
        Self::from_linear(range)
    }

    pub const fn cap(self, upper: usize) -> Self {
        let start = self.0.start;
        let last = self.0.last;
        let last = if last < upper { last } else { upper - 1 };
        let range = RangeInclusive { start, last };
        Self::from_linear(range)
    }

    const fn from_linear(range: RangeInclusive<usize>) -> Self {
        const { assert!(N != 0) }

        debug_assert!(
            !(range.start == 0 && range.last != 0),
            "invalid range: a range including the root always has length 1"
        );

        Self(range)
    }

    pub const fn to_linear(&self) -> RangeInclusive<usize> {
        self.0
    }
}

impl<const N: usize> IntoIterator for IndexRange<N> {
    type Item = Index<N>;
    type IntoIter = IndexRangeIter<N>;

    fn into_iter(self) -> Self::IntoIter {
        IndexRangeIter(self.0)
    }
}

#[derive(Debug, Clone)]
pub struct IndexRangeIter<const N: usize>(RangeInclusive<usize>);

impl<const N: usize> Iterator for IndexRangeIter<N> {
    type Item = Index<N>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.0.into_iter().next()?;
        let index = Index::<N>::from_linear(index);
        self.0.start += 1;
        Some(index)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<const N: usize> ExactSizeIterator for IndexRangeIter<N> {
    fn len(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0.last - self.0.start + 1
        }
    }
}

impl<const N: usize> DoubleEndedIterator for IndexRangeIter<N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = self.0.into_iter().next_back()?;
        let index = Index::<N>::from_linear(index);
        self.0.last -= 1;
        Some(index)
    }
}

impl<const N: usize> FusedIterator for IndexRangeIter<N> {}
