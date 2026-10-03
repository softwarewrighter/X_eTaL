//! The first-order built-ins: which exist and their arity (from the
//! catalog), and calling one on its arguments. The higher-order ones
//! are `xetal-hof`'s; the evaluator (`xetal-step`) chooses.

mod call;

pub use call::{arity, call};
