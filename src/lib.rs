pub struct Subranges {
    range: Interval,
}

impl Subranges {
    pub fn new(range: Interval) -> Self {
        Self { range }
    }

    /// Fill part of range with `length` subrange and return it.
    /// If free interval with specified `length` doesn't exists, return None.
    pub fn take_free_subrange(&mut self, length: u64) -> Option<Interval> {
        todo!()
    }

    /// Free all filled intervals, that intersects with `subrange`.
    pub fn erase_subrange(&mut self, subrange: Interval) {
        todo!()
    }
}

/// Represent integer interval [a, b).
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

    pub fn start(&self) -> i64 {
        self.start
    }

    pub fn end(&self) -> i64 {
        self.start + self.length
    }

    pub fn contains(&self, p: i64) -> bool {
        p >= self.start && p < self.end()
    }

    pub fn intersect(&self, other: &Self) -> bool {
        let sum_len = self.length + other.length;
        sum_len < self.max_dist(other)
    }

    fn max_dist(&self, other: &Self) -> i64 {
        let points = [self.start, self.end(), other.start, other.end()];
        let (min, max) = (*points.iter().min().unwrap(), *points.iter().max().unwrap());
        max - min
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
