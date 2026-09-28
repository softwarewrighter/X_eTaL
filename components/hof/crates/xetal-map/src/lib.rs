//! Item-by-item higher-order built-ins (B6): `e_ach` and `t_able`.
//! Operands run through [`xetal_value::Caller`]; until nested arrays
//! exist (A7), every call must give a single value.

mod each;
mod items;
mod table;

pub use each::{each, zip};
pub use table::table;
