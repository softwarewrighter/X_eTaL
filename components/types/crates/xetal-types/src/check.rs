//! The checker's entry points: check a source or a lowered program, or
//! report the types the typed lints need.

use xetal_base::{Diagnostic, Span};
use xetal_core::Program;
use xetal_ty::Type;

use crate::infer::{infer_program, inferred};

/// Lex, parse, desugar and infer a whole program; one line per item.
pub fn check_source(src: &str) -> Result<Vec<String>, Diagnostic> {
    check_program(&mut xetal_core::lower(src)?)
}

/// Infer `program`, then elaborate it (`xetal-elab`): polymorphic
/// number code takes the number type it is used at, so evaluation
/// agrees with the types (T6).
pub fn check_program(program: &mut Program) -> Result<Vec<String>, Diagnostic> {
    let (lines, dicts) = infer_program(program)?;
    xetal_elab::elaborate(program, &dicts);
    Ok(lines)
}

/// Each guard's condition (`cond ? then`) in `program`, by its span,
/// with the type inference gives it: a variable where it stays
/// polymorphic (a `Truthy` that defaults to Bool). For the typed lints;
/// a program that does not type-check is an error.
pub fn guard_types(program: &Program) -> Result<Vec<(Span, Type)>, Diagnostic> {
    let inf = inferred(program)?;
    let resolve = |(span, ty): &(Span, Type)| (*span, inf.u.resolve(ty));
    Ok(inf.rec.guards.iter().map(resolve).collect())
}
