//! The steppable evaluator (D50): Core evaluated by an explicit machine
//! whose state is data (the control in hand and a stack of pending
//! work), so a run can be taken in slices of any size and stop between
//! any two transitions, as web-sw-tos steps its emulated CPU.

mod apply;
mod caller;
mod eval;
mod machine;
mod resume;
mod trap;

pub use machine::{Machine, Status};
