//! Higher-order built-ins as kernels (D50): a kernel asks the evaluator
//! to call a function and is given the result, one call at a time, so
//! the evaluator can stop between any two calls (in a page, while it
//! waits for input). The combinators build kernels in the shape of the
//! loops they replace, keeping the order of every call.

mod combine;
mod kernel;

pub use combine::{all, apply, fold, then};
pub use kernel::{Kernel, Next, done, drive, fail};
