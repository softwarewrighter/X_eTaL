//! Types: representation, unification with an occurs check, the `Num`
//! and `Truthy` constraints, type schemes, and Algorithm W over Core.

mod builtins;
mod expr;
mod infer;
mod scheme;
mod ty;
mod unify;

pub use infer::infer_program;
pub use ty::{Scheme, Type, TypeVar};
pub use unify::Unifier;

use xetal_base::Diagnostic;

/// Lex, parse, desugar and infer a whole program.
pub fn check_source(src: &str) -> Result<Vec<String>, Diagnostic> {
    infer_program(&xetal_core::lower(src)?)
}

/// Pipeline stage name used in diagnostics and by the CLI.
pub const STAGE: &str = "type";
