//! The scalar arithmetic rules: Bool as 1 / 0 (T1), `/` always Float
//! (T2), exact `=` and tolerant `e_q~` (T3), power (D-4, D-10), overflow
//! and division-by-zero errors; and scalar extension over arrays (T7).

mod lift;
mod num;
mod ops;

pub use lift::{lift1, lift2};
pub use num::{Num, num, truth};
pub use ops::{binary, compare, compare_chars};
