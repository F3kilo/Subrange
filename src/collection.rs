use crate::interval::Interval;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::iter;
use std::ops::Bound;

/// Collection of free intervals.
/// You can take parts of the free intervals and add new free intervals.
#[derive(Debug, Default)]
pub struct FreeIntervals {
    btree: BTreeSet<IntervalLenOrd>,
}

impl FreeIntervals {
    /// Initialize collection with free interval.
    pub fn new(free_interval: Interval) -> Self {
        let btree = iter::once(IntervalLenOrd(free_interval)).collect();
        Self { btree }
    }

    /// Take the minimal interval larger then `length`.
    /// If collection doesn't contain such free interval, `None` will be returned.
    pub fn take_enough(&mut self, length: u64) -> Option<Interval> {
        let int_len_ord = IntervalLenOrd(Interval::new(0, length));
        let bounds = (Bound::Included(int_len_ord), Bound::Unbounded);
        let range = self.btree.range(bounds);
        let found = range.copied().next();
        found.map(|i| {
            self.btree.remove(&i);
            i.0
        })
    }

    /// Take the minimal interval larger then `length`.
    /// Add paddintg to the interval start, to align it.
    /// The align padding will be added to collection as a new free interval.
    /// If collection doesn't contain such free interval, `None` will be returned.
    pub fn take_enough_aligned(&mut self, length: u64, align: u64) -> Option<Interval> {
        let int_len_ord = IntervalLenOrd(Interval::new(0, length));
        let bounds = (Bound::Included(int_len_ord), Bound::Unbounded);
        let mut range = self.btree.range(bounds);
        let enough_int = range
            .find(|i| {
                let pad = Self::align_pad(&i.0, align);
                i.0.len() >= length + pad
            })
            .copied();
        if let Some(i) = enough_int {
            self.btree.remove(&i);
            return Some(i.0);
        }
        None
    }

    /// Take the minimal interval larger then `length` and split it into `[length, extra]` parts.
    /// Add `extra` part as new free interval.
    /// If collection doesn't contain such free interval, `None` will be returned.
    pub fn take_exact(&mut self, length: u64) -> Option<Interval> {
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

    /// Take the minimal interval larger then `length` and split it into `[length, extra]` parts.
    /// Add `extra` part as new free interval.
    /// Add paddintg to the interval start, to align it.
    /// The align padding will be added to collection as a new free interval.
    /// If collection doesn't contain such free interval, `None` will be returned.
    pub fn take_exact_aligned(&mut self, length: u64, align: u64) -> Option<Interval> {
        let enough_free_interval = self.take_enough_aligned(length, align);
        enough_free_interval.map(|int| {
            let align_pad = Self::align_pad(&int, align);
            if align_pad > 0 {
                let pad_int = Interval::new(int.start(), align_pad);
                self.btree.insert(IntervalLenOrd(pad_int));
            }

            let int = Interval::new(int.start() + align_pad, int.len() - align_pad);
            if int.len() > length {
                let (req, extra) = int.split(length);
                self.btree.insert(IntervalLenOrd(extra));
                return req;
            }
            int
        })
    }

    /// Insert free `interval` to collection.
    /// Connects it with any near free interval in the collection.
    pub fn insert(&mut self, interval: Interval) {
        let near_intervals = self.near(&interval);
        let mut connection = interval;
        for int in &near_intervals {
            self.btree.remove(int);
            connection = connection.connect(&int.0);
        }

        self.btree.insert(IntervalLenOrd(connection));
    }

    /// Restore interval storage to initial state.
    pub fn clear(&mut self) {}

    /// Returns iterator over free intervals.
    pub fn iter(&self) -> impl Iterator<Item = &Interval> {
        self.btree.iter().map(|i| &i.0)
    }

    /// Find all intervals near to `interval`.
    fn near(&self, interval: &Interval) -> Vec<IntervalLenOrd> {
        self.btree
            .iter()
            .filter(|int| interval.near(&int.0))
            .cloned()
            .collect()
    }

    fn align_pad(int: &Interval, align: u64) -> u64 {
        let rem = int.start() % align;
        if rem == 0 {
            return 0;
        }
        align - rem
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct IntervalLenOrd(Interval);

impl PartialOrd for IntervalLenOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let len_cmp = self.0.len().partial_cmp(&other.0.len())?;
        if len_cmp == Ordering::Equal {
            return self.0.start().partial_cmp(&other.0.start());
        }

        Some(len_cmp)
    }
}

impl Ord for IntervalLenOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        let len_cmp = self.0.len().cmp(&other.0.len());
        if len_cmp == Ordering::Equal {
            return self.0.start().cmp(&other.0.start());
        }

        len_cmp
    }
}

#[cfg(test)]
mod tests {
    use crate::collection::FreeIntervals;
    use crate::interval::Interval;

    fn test_data() -> FreeIntervals {
        let mut coll = FreeIntervals::default();
        let free_interval = Interval::new(0, 10);
        coll.insert(free_interval);
        coll
    }

    #[test]
    fn take_enough() {
        let mut coll = test_data();
        let seven_len = coll.take_enough(7).unwrap();
        assert!(seven_len.len() > 7);
        assert!(coll.take_enough(1).is_none());
    }

    #[test]
    fn insert_non_intersect() {
        let mut coll = test_data();
        coll.insert(Interval::new(15, 5));
        let mut it = coll.iter();
        assert_eq!(it.next().unwrap().len(), 5);
        assert_eq!(it.next().unwrap().len(), 10);
        assert!(it.next().is_none());
    }

    #[test]
    fn insert_intersect() {
        let mut coll = test_data();
        coll.insert(Interval::new(15, 5));
        coll.insert(Interval::new(8, 10));
        let mut it = coll.iter();
        assert_eq!(it.next().unwrap().len(), 20);
        assert!(it.next().is_none());
    }

    #[test]
    fn take_enough_none() {
        let mut coll = test_data();
        coll.insert(Interval::new(15, 5));
        assert!(coll.take_enough(15).is_none());
    }

    #[test]
    fn take_enough_twice() {
        let mut coll = test_data();
        coll.insert(Interval::new(15, 5));
        coll.insert(Interval::new(25, 3));
        assert_eq!(coll.take_enough(3).unwrap().len(), 3);
        assert_eq!(coll.take_enough(5).unwrap().len(), 5);
        assert_eq!(coll.take_enough(10).unwrap().len(), 10);
        assert!(coll.take_enough(1).is_none());
    }

    #[test]
    fn take_exact() {
        let mut coll = test_data();
        coll.insert(Interval::new(15, 5));
        coll.insert(Interval::new(25, 3));
        assert_eq!(coll.take_exact(2).unwrap().len(), 2);
        assert_eq!(coll.take_exact(2).unwrap().len(), 2);
        let mut it = coll.iter();
        assert_eq!(it.next().unwrap().len(), 1);
        assert_eq!(it.next().unwrap().len(), 3);
        assert_eq!(it.next().unwrap().len(), 10);
        assert!(it.next().is_none());
    }

    #[test]
    fn take_exact_align() {
        let mut coll = FreeIntervals::default();
        let free_interval = Interval::new(0, 30);
        coll.insert(free_interval);

        let len = 15;
        let align = 2;
        let int = coll.take_exact_aligned(len, align).unwrap();
        assert_eq!(int.start() % align, 0);
        assert_eq!(int.len(), len);

        let len = 4;
        let align = 4;
        let int = coll.take_exact_aligned(len, align).unwrap();
        assert_eq!(int.start() % align, 0);
        assert_eq!(int.len(), len);

        let mut iter = coll.iter();
        assert_eq!(*iter.next().unwrap(), Interval::new(15, 1));
        assert_eq!(*iter.next().unwrap(), Interval::new(20, 10));
        assert!(iter.next().is_none());
    }

    #[test]
    fn take_exact_none() {
        let mut coll = test_data();
        coll.insert(Interval::new(15, 5));
        coll.insert(Interval::new(25, 3));
        assert!(coll.take_exact(20).is_none());
    }

    #[test]
    fn insert_different_intervals_with_same_size() {
        let mut collection = FreeIntervals::new((0..9).into());

        let taken1 = collection.take_exact(3).unwrap();
        let taken2 = collection.take_exact(3).unwrap();

        {
            let mut iter = collection.iter();
            assert_eq!(iter.next().unwrap().len(), 3);
            assert!(iter.next().is_none());
        }

        collection.insert(taken1);
        collection.insert(taken2);
        collection.take_exact(9).unwrap();
    }
}
