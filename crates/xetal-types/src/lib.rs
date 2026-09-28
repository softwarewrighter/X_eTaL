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
use xetal_core::Program;

/// Lex, parse, desugar and infer a whole program; one line per item.
pub fn check_source(src: &str) -> Result<Vec<String>, Diagnostic> {
    check_program(&mut xetal_core::lower(src)?)
}

/// Infer `program`, then elaborate it: integer literals whose type is
/// Float become Float literals, so evaluation agrees with the types.
pub fn check_program(program: &mut Program) -> Result<Vec<String>, Diagnostic> {
    let (lines, floats) = infer_program(program)?;
    program.float_literals(&floats);
    Ok(lines)
}

/// Pipeline stage name used in diagnostics and by the CLI.
pub const STAGE: &str = "type";
