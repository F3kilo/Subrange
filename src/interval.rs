use std::cmp;

/// Represent integer interval.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Interval {
    start: i64,
    length: i64,
}

impl Interval {
    /// Create new interval [`start`; start + length).
    pub fn new(start: i64, length: i64) -> Self {
        assert!(length >= 0);
        Self { start, length }
    }

    /// Length of interval. Count of integers in `self`.
    pub fn len(&self) -> i64 {
        self.length
    }

    /// True if self.len() == 0.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// First intager in `self`.
    pub fn start(&self) -> i64 {
        self.start
    }

    /// Integer after last integer in `self`.
    pub fn end(&self) -> i64 {
        self.start + self.length
    }

    /// Return `true` if `self` contains `p`. Else `false`.
    pub fn contains(&self, p: i64) -> bool {
        p >= self.start && p < self.end()
    }

    /// Test if `other` has got common integers with `self`.
    pub fn intersect(&self, other: &Self) -> bool {
        let connected = self.connect(other);
        connected.length < (self.length + other.length)
    }

    /// Return `true` if no integers between `self` and `other` exists.
    pub fn near(&self, other: &Self) -> bool {
        let connected = self.connect(other);
        connected.length <= (self.length + other.length)
    }

    /// Return interval from min `start` to max `end`.
    pub fn connect(&self, other: &Self) -> Self {
        let min_start = cmp::min(self.start, other.start);
        let max_end = cmp::max(self.end(), other.end());
        Interval::new(min_start, max_end - min_start)
    }

    /// Split `self` into two intervals.
    /// First - `[self.start; self.start + length)`, second - `[self.start + length; self.end)`.
    pub fn split(&self, length: i64) -> (Self, Self) {
        let len_fit = length >= 0 && length <= self.length;
        assert!(len_fit, "Split length must be >= 0 and <= original length");
        let left = Self::new(self.start, length);
        let right = Self::new(self.start + length, self.length - length);
        (left, right)
    }

    /// If intervals is near, return their union. Else `None`.
    pub fn try_join(&self, other: &Self) -> Option<Self> {
        match self.near(other) {
            true => Some(self.connect(other)),
            false => None,
        }
    }

    /// Return iterator over integers in `self`.
    pub fn iter(&self) -> IntervalIterator {
        IntervalIterator {
            interval: &self,
            counter: 0,
        }
    }
}

pub struct IntervalIterator<'a> {
    interval: &'a Interval,
    counter: i64,
}

impl<'a> Iterator for IntervalIterator<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.counter < self.interval.len() {
            true => Some(self.counter),
            false => None,
        };
        self.counter += 1;
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::interval::Interval;

    #[test]
    fn connect() {
        let i1 = Interval::new(0, 10);
        let i2 = Interval::new(15, 10);
        let conn = i1.connect(&i2);
        assert_eq!(conn.start(), i1.start());
        assert_eq!(conn.end(), i2.end());

        let conn_refl = i2.connect(&i1);
        assert_eq!(conn, conn_refl);
    }

    #[test]
    fn split() {
        let i = Interval::new(0, 10);
        let sp = 3;
        let (s1, s2) = i.split(sp);
        assert_eq!(s1.start(), i.start());
        assert_eq!(s1.len(), sp);
        assert_eq!(s2.len(), i.len() - s1.len());
        assert_eq!(s2.start(), s1.end())
    }

    #[test]
    fn join() {
        let i1 = Interval::new(0, 10);
        let i2 = Interval::new(5, 10);
        let i3 = Interval::new(20, 10);
        let join = i1.try_join(&i2).unwrap();
        assert_eq!(join.start(), i1.start());
        assert_eq!(join.end(), i2.end());

        let join_refl = i2.try_join(&i1).unwrap();
        assert_eq!(join, join_refl);

        assert!(i1.try_join(&i3).is_none());
        assert!(i3.try_join(&i1).is_none());
    }
}
