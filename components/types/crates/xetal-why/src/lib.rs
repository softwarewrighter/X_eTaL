//! Where a type came from (Saga 27 step 2, D163): what a type mismatch
//! says about each side, as labels at the places a reader can look at
//! (a binding, a literal, a built-in's result, a lambda's parameter),
//! and the hints for the common mistakes (D164).
//! Pure: the checker says how each name is bound and what each operand's
//! type resolved to.

mod gives;
mod hints;
mod said;

pub use gives::{gives, signature};
pub use hints::hinted;
pub use said::{Bound, said};
