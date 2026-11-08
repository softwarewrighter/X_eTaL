//! Values as a debugger shows them (Saga 43, step debug-variables): a
//! one-line [`summary`] with the type, the shape and the first items,
//! and the value's [`children`] a page at a time (an array's major
//! cells, a tuple's parts, a box's content), so a large array opens
//! row by row instead of all at once; and whether a value is
//! [`truthy`], for a breakpoint's condition.

mod children;
mod env;
mod summary;

pub use children::{children, count};
pub use env::{bindings, own};
pub use summary::{summary, truthy, type_name};
