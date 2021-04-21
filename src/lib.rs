pub mod collection;
pub mod interval;

use crate::collection::IntervalsCollection;
use crate::interval::Interval;

/// Provides non-intersecting integer subranges of initial range.
pub struct Subranges {
    free: IntervalsCollection,
    len: u64,
}

impl Subranges {

    /// Creates `Self` with specified free range.
    pub fn new(range: Interval) -> Self {
        let mut free = IntervalsCollection::default();
        free.insert(range);
        Self { free, len: range.len() }
    }

    /// Take free interval with specified `length` and returns it.
    /// If free interval with specified `length` doesn't exists, return None.
    pub fn take_free_subrange(&mut self, length: u64) -> Option<Interval> {
        assert!(length > 0, "Length must be > 0");
        self.free.take_exact(length)
    }

    /// Take free interval with specified `length` and returns it.
    /// If free interval with specified `length` doesn't exists, return None.
    pub fn take_free_align_subrange(&mut self, length: u64, align: u64) -> Option<Interval> {
        assert!(length > 0, "Length must be > 0");
        self.free.take_exact_aligned(length, align)
    }

    /// Free all filled intervals, that intersects with `subrange`.
    pub fn erase_subrange(&mut self, subrange: Interval) {
        self.free.insert(subrange)
    }

    /// Length of full range.
    pub fn len(&self) -> u64 {
        self.len
    }

    /// True if `self.len()` is zero.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}