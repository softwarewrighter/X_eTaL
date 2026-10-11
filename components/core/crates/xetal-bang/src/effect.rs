//! What in a function's body is an effect. Direct: a `!` built-in or a
//! `!` function of the program or a library, called anywhere in the
//! body (an operand runs then). Possible: also a `!` parameter or
//! variable (passed in, or mutable state) and a system function (`[]`;
//! some are effects, and the catalog does not yet say which).

use xetal_ir::{Expr, Kind};

/// The first effect `e` calls directly, by name.
pub(crate) fn direct(e: &Expr) -> Option<&str> {
    match &e.kind {
        Kind::Prim(name) | Kind::Global(name) if name.ends_with('!') => Some(name),
        _ => e.children().into_iter().find_map(direct),
    }
}

/// Whether `e` may have an effect: a direct one, a `!` parameter or
/// variable, a mutation or a system function.
pub(crate) fn possible(e: &Expr) -> bool {
    let here = match &e.kind {
        Kind::Var(name) => name.ends_with('!'),
        Kind::Prim(name) => name.starts_with("[]"),
        Kind::Set { .. } => true,
        _ => false,
    };
    here || direct(e).is_some() || e.children().into_iter().any(possible)
}
