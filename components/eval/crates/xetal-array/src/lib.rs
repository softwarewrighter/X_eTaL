//! Dense row-major arrays and their layout (knows nothing about syntax).
//! A scalar is not an `Array`: rank-0 values stay scalars in the
//! evaluator, and every `Array` has rank 1 or more.

mod array;
mod error;
mod layout;

pub use array::{Array, zip};
pub use error::{ArrayError, STAGE};
pub use layout::layout;
