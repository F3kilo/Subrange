mod collection;
pub mod interval;

use crate::collection::IntervalsCollection;
use crate::interval::Interval;

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
