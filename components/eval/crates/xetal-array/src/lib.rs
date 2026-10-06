//! Dense row-major arrays and their layout (knows nothing about syntax).
//! A kernel may produce a rank-0 array (one item); the evaluator turns it
//! into a scalar, so every array value has rank 1 or more.

mod array;
mod error;
mod kind;
mod layout;
mod ops;

pub use array::Array;
pub use error::{ArrayError, MAX_ITEMS, size};
pub use kind::Kind;
pub use layout::layout;
pub use ops::zip;
