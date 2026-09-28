//! Runtime values: scalars, arrays, closures and built-ins, the
//! persistent environment, and printed results.

mod display;
mod value;

pub use value::{Closure, Env, Frame, Prim, Slot, Value, extend, lookup};
