//! The Core IR (docs/design.md section 4): the small calculus every
//! surface form desugars into, and its printer.

mod ir;
mod names;
mod notes;
mod show;
mod walk;

pub use ir::{Expr, Item, Kind, Param, Program};
pub use names::{NameSpan, written_at};
pub use notes::SpanNote;
