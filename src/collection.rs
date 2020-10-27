use crate::interval::Interval;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::ops::Bound;

#[derive(Debug)]
pub struct IntervalsCollection {
    btree: BTreeSet<IntervalLenOrd>,
}

impl IntervalsCollection {
    pub fn take_enough(&mut self, length: i64) -> Option<Interval> {
        assert!(length >= 0, "Interval length must be >= 0");
        let int_len_ord = IntervalLenOrd(Interval::new(0, length));
        let bounds = (Bound::Included(int_len_ord), Bound::Unbounded);
        let range = self.btree.range(bounds);
        let found = range.copied().next();
        found.map(|i| {
            self.btree.remove(&i);
            i.0
        })
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

    pub fn insert(&mut self, interval: Interval) {
        let near_intervals = self.near(&interval);
        let mut connection = interval;
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

    pub fn iter(&self) -> impl Iterator<Item = &Interval> {
        self.btree.iter().map(|i| &i.0)
    }
}

impl Default for IntervalsCollection {
    fn default() -> Self {
        let btree = BTreeSet::new();
        Self { btree }
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
    use crate::collection::IntervalsCollection;
    use crate::interval::Interval;

    fn test_data() -> IntervalsCollection {
        let mut coll = IntervalsCollection::new();
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
    fn take_exact_none() {
        let mut coll = test_data();
        coll.insert(Interval::new(15, 5));
        coll.insert(Interval::new(25, 3));
        assert!(coll.take_exact(20).is_none());
    }
}
