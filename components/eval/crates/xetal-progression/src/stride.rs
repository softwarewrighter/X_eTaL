//! What a progression stays a progression under: take, drop, reverse,
//! a shift or a scale by an Int, its sum, and where an Int stands in it.

use crate::Progression;

impl Progression {
    /// `n t_ake p`, when no fill is needed (`|n|` at most the count).
    pub fn take(&self, n: i64) -> Option<Self> {
        let k = usize::try_from(n.unsigned_abs()).ok()?;
        if k > self.count() {
            return None;
        }
        if k == 0 {
            return Progression::new(0, 1, 0);
        }
        let start = if n > 0 {
            self.start()
        } else {
            self.at(self.count() - k)?
        };
        Progression::new(start, self.step(), k)
    }

    /// `n d_rop p`.
    pub fn drop(&self, n: i64) -> Option<Self> {
        let k = usize::try_from(n.unsigned_abs()).ok()?.min(self.count());
        let rest = self.count() - k;
        if rest == 0 {
            return Progression::new(0, 1, 0);
        }
        let start = if n >= 0 { self.at(k)? } else { self.start() };
        Progression::new(start, self.step(), rest)
    }

    /// `r_ev p`.
    pub fn reversed(&self) -> Option<Self> {
        match self.count() {
            0 => Some(*self),
            n => Progression::new(self.at(n - 1)?, self.step().checked_neg()?, n),
        }
    }

    /// `p + d` (and `d + p`).
    pub fn shifted(&self, d: i64) -> Option<Self> {
        Progression::new(self.start().checked_add(d)?, self.step(), self.count())
    }

    /// `p * k` (and `k * p`).
    pub fn scaled(&self, k: i64) -> Option<Self> {
        let (start, step) = (self.start().checked_mul(k)?, self.step().checked_mul(k)?);
        Progression::new(start, step, self.count())
    }

    /// `'+ r_/ p`, if it fits an `i64`.
    pub fn sum(&self) -> Option<i64> {
        let n = i128::try_from(self.count()).ok()?;
        let first = i128::from(self.start());
        let wide = n * first + i128::from(self.step()) * (n * (n - 1) / 2);
        i64::try_from(wide).ok()
    }

    /// Where `x` stands (0-origin), if it is an item.
    pub fn position(&self, x: i64) -> Option<usize> {
        let off = i128::from(x) - i128::from(self.start());
        let step = i128::from(self.step());
        let i = match step {
            0 => (off == 0).then_some(0)?,
            _ if off % step != 0 => return None,
            _ => off / step,
        };
        usize::try_from(i).ok().filter(|i| *i < self.count())
    }
}
