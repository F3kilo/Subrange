use crate::interval::Interval;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::ops::Bound;

pub(crate) struct IntervalsCollection {
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
