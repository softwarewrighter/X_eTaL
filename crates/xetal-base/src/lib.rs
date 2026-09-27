//! Shared foundations for every xetal crate: the language display name,
//! source spans, Core node ids and the `Diagnostic` error value that all
//! crate-local error types convert into.

mod diagnostic;
mod span;

pub use diagnostic::Diagnostic;
pub use span::Span;

/// Display name of the language. The only place the name is spelled in
/// code; everything else refers to this constant.
pub const LANG_NAME: &str = "X_eTaL";

/// Identity of a Core node, used to link traces back to source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub u32);
