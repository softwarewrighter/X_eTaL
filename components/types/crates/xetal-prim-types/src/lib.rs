//! Types of the built-in functions (T5): number literals and variables
//! are `Num`, conditions `Truthy`, comparisons return a `Truthy` value.

mod generic;
mod prims;

pub use prims::prim_type;
