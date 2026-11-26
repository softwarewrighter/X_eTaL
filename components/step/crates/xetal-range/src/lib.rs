//! Built-ins on a range that keep it a progression (step 202, D211):
//! `r_ange` and `o_ffsets` make one, and the operations here give a
//! progression or a scalar without its items. `None` declines: the
//! caller then makes every range an array (`xetal_value::dense_all`)
//! and calls the built-in as usual, so results never differ.

mod arith;
mod dense;
mod make;
mod search;
mod shape;

pub use dense::{dense, dense_args, dense_one, guard};

use xetal_value::Value;

/// A built-in applied to all its arguments, when a range lets it skip
/// the items; `None` otherwise.
pub fn call<'a>(name: &str, args: &[Value<'a>]) -> Option<Value<'a>> {
    make::call(name, args)
        .or_else(|| shape::call(name, args))
        .or_else(|| arith::call(name, args))
        .or_else(|| search::call(name, args))
}
