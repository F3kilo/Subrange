use std::cmp;

/// Represent integer interval.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Interval {
    pub start: i64,
    pub length: i64,
}

impl Interval {
    pub fn new(start: i64, length: i64) -> Self {
        assert!(length >= 0);
        Self { start, length }
    }

    pub fn len(&self) -> i64 {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn start(&self) -> i64 {
        self.start
    }

    pub fn end(&self) -> i64 {
        self.start + self.length
    }

    pub fn dist(&self) -> i64 {
        (self.length as u64).saturating_sub(1) as i64
    }

    pub fn contains(&self, p: i64) -> bool {
        p >= self.start && p < self.end()
    }

    pub fn intersect(&self, other: &Self) -> bool {
        let connected = self.connect(other);
        connected.length < (self.length + other.length)
    }

    pub fn near(&self, other: &Self) -> bool {
        let connected = self.connect(other);
        connected.length <= (self.length + other.length)
    }

    pub fn connect(&self, other: &Self) -> Self {
        let min_start = cmp::min(self.start, other.start);
        let max_end = cmp::max(self.end(), other.end());
        Interval::new(min_start, max_end - min_start)
    }

    pub fn split(&self, length: i64) -> (Self, Self) {
        let len_fit = length >= 0 && length <= self.length;
        assert!(len_fit, "Split length must be >= 0 and <= original length");
        let left = Self::new(self.start, length);
        let right = Self::new(self.start + length, self.length - length);
        (left, right)
    }

    pub fn join(&self, other: &Self) -> Self {
        assert!(
            self.near(other),
            "Joining intervals must be near to each other"
        );
        self.connect(other)
    }

    pub fn try_join(&self, other: &Self) -> Option<Self> {
        match self.near(other) {
            true => Some(self.connect(other)),
            false => None,
        }
    }

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
