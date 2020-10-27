use std::cmp::{min, Ordering};
use std::collections::BTreeSet;
use std::ops::Bound;

pub struct Subranges {
    free: IntervalsCollection,
}

impl Subranges {
    pub fn new(range: Interval) -> Self {
        let mut free = IntervalsCollection::new();
        free.insert(&range);
        Self { free }
    }

    /// Take free interval with specified `length` and returns it.
    /// If free interval with specified `length` doesn't exists, return None.
    pub fn take_free_subrange(&mut self, length: i64) -> Option<Interval> {
        assert!(length > 0, "Length must be >= 0");
        self.free.take_exact(length)
    }

    /// Free all filled intervals, that intersects with `subrange`.
    pub fn erase_subrange(&mut self, subrange: Interval) {
        self.free.insert(&subrange)
    }
}

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
        let min_start = min(self.start, other.start);
        let max_end = min(self.end(), other.end());
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

struct IntervalsCollection {
    btree: BTreeSet<IntervalLenOrd>,
}

impl IntervalsCollection {
    pub fn new() -> Self {
        let btree = BTreeSet::new();
        Self { btree }
    }

    pub fn take_enough(&mut self, length: i64) -> Option<Interval> {
        assert!(length >= 0, "Interval length must be >= 0");
        let int_len_ord = IntervalLenOrd(Interval::new(0, length));
        let bounds = (Bound::Included(int_len_ord), Bound::Unbounded);
        self.btree.range(bounds).next().map(|int| int.0)
    }

    pub fn take_exact(&mut self, length: i64) -> Option<Interval> {
        let enough_free_interval = self.take_enough(length);
        enough_free_interval.map(|int| {
            if int.len() > length {
                let (req, extra) = int.split(length);
                self.btree.insert(IntervalLenOrd(extra));
                return req;
            }
            int
        })
    }

    pub fn insert(&mut self, interval: &Interval) {
        let near_intervals = self.near(interval);
        let mut connection = *interval;
        for int in &near_intervals {
            self.btree.remove(int);
            connection = connection.connect(&int.0);
        }
        self.btree.insert(IntervalLenOrd(connection));
    }

    fn near(&self, interval: &Interval) -> Vec<IntervalLenOrd> {
        self.btree
            .iter()
            .filter(|int| interval.near(&int.0))
            .cloned()
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct IntervalLenOrd(Interval);

impl PartialOrd for IntervalLenOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.0.len(), &other.0.len())
    }
}

impl Ord for IntervalLenOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.0.len(), &other.0.len())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
