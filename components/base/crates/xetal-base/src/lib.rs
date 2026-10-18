//! Shared foundations for every xetal crate: the language display name,
//! source spans, Core node ids and the `Diagnostic` error value that all
//! crate-local error types convert into.

mod diagnostic;
mod fix;
mod lang;
mod place;
mod shown;
mod span;

pub use diagnostic::{Diagnostic, Located, Severity};
pub use fix::Fix;
pub use lang::LANG_NAME;
pub use place::CALLED_FROM;
pub use place::{Label, Place};
pub use span::{NodeId, Span};
