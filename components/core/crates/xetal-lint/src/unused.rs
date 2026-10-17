//! `unused-binding`: a lambda parameter or local `:=` that nothing in
//! its scope uses. Exempt: `_` (a value kept for its effect), the
//! shorthand's `_r` and `_l`, and the hidden names a pattern binds.
//! Top-level bindings are not checked (a reader or importer may want them).

use xetal_base::Diagnostic;
use xetal_ir::{Expr, Item, Kind, Param, Program};

use crate::walk::binders;

/// The unused-binding findings of every lambda and local in `program`.
pub(crate) fn unused(program: &Program) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for item in &program.items {
        let (Item::Def { value, .. }
        | Item::Let { value, .. }
        | Item::Set { value, .. }
        | Item::Eval(value)) = item;
        binders(value, &mut |name, e| out.extend(check(program, name, e)));
    }
    out
}

/// The finding for `name`, bound by node `e`, if nothing uses it: at
/// the name where it is written (D152), and for a `:=` a fix to `_`.
fn check(program: &Program, name: &str, e: &Expr) -> Option<Diagnostic> {
    if name.starts_with(['_', '%']) {
        return None;
    }
    let (used, message, at) = match &e.kind {
        Kind::Lam { body, .. } => (
            uses(body, name),
            format!("parameter `{name}` is never used"),
            program.param_span(e.id),
        ),
        Kind::Let {
            rec, value, body, ..
        } => (
            uses(body, name) || (*rec && uses(value, name)),
            format!(
                "`{name}` is bound but never used; write `_ := ...` to keep a value for its effect"
            ),
            program.name_span(value.id),
        ),
        _ => return None,
    };
    if used {
        return None;
    }
    let found = Diagnostic::warning("unused-binding", message).with_span(at.unwrap_or(e.span));
    match (at, &e.kind) {
        (Some(at), Kind::Let { .. }) => Some(found.with_fixes([(at, "_".into())])),
        _ => Some(found),
    }
}

/// Whether `e` refers to `name` (a variable read or set), not counting
/// where an inner binding of the same name hides it.
pub fn uses(e: &Expr, name: &str) -> bool {
    match &e.kind {
        Kind::Var(n) => n == name,
        Kind::Set { name: n, .. } if n == name => true,
        Kind::Lam {
            param: Param::Name(p),
            ..
        } if p == name => false,
        Kind::Let {
            name: n,
            rec,
            value,
            ..
        } if n == name => !rec && uses(value, name),
        _ => e.children().into_iter().any(|c| uses(c, name)),
    }
}
