//! An arithmetic progression kept as its start, step and count (as
//! Julia's and R's ranges are): what `r_ange` and `o_ffsets` give, so
//! a range far past `MAX_ITEMS` costs nothing until an operation needs
//! its items. Every operation here is exact or declines (`None`), and
//! the caller then works on the items.

use xetal_array::{ArrayError, size};

/// `start, start + step, ...`, `count` items; every item fits an `i64`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Progression {
    start: i64,
    step: i64,
    count: usize,
}

impl Progression {
    /// The progression, if its last item fits an `i64`.
    pub fn new(start: i64, step: i64, count: usize) -> Option<Self> {
        let p = Progression { start, step, count };
        p.at_wide(count.saturating_sub(1)).map(|_| p)
    }

    pub fn start(&self) -> i64 {
        self.start
    }

    pub fn step(&self) -> i64 {
        self.step
    }

    pub fn count(&self) -> usize {
        self.count
    }

    /// Item `i` (0-origin), if it is one.
    pub fn at(&self, i: usize) -> Option<i64> {
        (i < self.count).then(|| self.at_wide(i)).flatten()
    }

    fn at_wide(&self, i: usize) -> Option<i64> {
        let wide = i128::from(self.start) + i128::from(self.step) * i128::try_from(i).ok()?;
        i64::try_from(wide).ok()
    }

    /// The items, if an array may hold that many.
    pub fn items(&self) -> Result<Vec<i64>, ArrayError> {
        size(&[self.count])?;
        Ok((0..self.count).filter_map(|i| self.at_wide(i)).collect())
    }
}
