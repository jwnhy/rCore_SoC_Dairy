#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Range<T: From<usize> + Into<usize> + Copy> {
    pub start: T,
    pub end: T,
}

impl <T: From<usize> + Into<usize> + Copy, U: Into<T>> From<core::ops::Range<U>> for Range<T> {
    fn from(range: core::ops::Range<U>) -> Self {
        Self {
            start: range.start.into(),
            end: range.end.into(),
        }
    }
}

impl<T: From<usize> + Into<usize> + Copy> Range<T> {
    pub fn overlap_with(&self, other: &Range<T>) -> bool {
        self.start.into() < other.end.into() && self.end.into() > other.start.into()
    }

    pub fn iter(&self) -> impl Iterator<Item = T> {
        (self.start.into()..self.end.into()).map(T::from)
    }

    pub fn len(&self) -> usize {
        self.end.into() - self.start.into()
    }

    pub fn into<U: From<usize> + Into<usize> + Copy + From<T>>(self) -> Range<U> {
        Range::<U> {
            start: U::from(self.start),
            end: U::from(self.end),
        }
    }

    pub fn get(&self, index: usize) -> T {
        assert!(index < self.len());
        T::from(self.start.into() + index)
    }

    pub fn contains(&self, value: T) -> bool {
        self.start.into() <= value.into() && value.into() < self.end.into()
    }
}