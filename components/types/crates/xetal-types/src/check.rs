//! The checker's entry points: check a source or a lowered program, or
//! report the types the typed lints need.

use xetal_base::{Diagnostic, Span};
use xetal_core::{Expr, Kind, Program};
use xetal_ty::Type;
use xetal_why::{Bound, said, signature};

use crate::infer::{Infer, infer_program, inferred};

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

impl Infer {
    /// A type mismatch at application `app` of `f` explained (D163):
    /// what each operand's type is and where it came from, and the
    /// built-in's type. Only the application's own mismatch, once.
    pub(crate) fn explained(
        &self,
        d: Diagnostic,
        app: &Expr,
        f: &Expr,
        args: &[(&Expr, &Type)],
    ) -> Diagnostic {
        if d.code != "type-mismatch" || d.span != Some(app.span) || !d.labels().is_empty() {
            return d;
        }
        let bound = |name: &str| {
            let (_, _, binder) = self.env.iter().rev().find(|(n, ..)| n == name)?;
            match binder {
                Some(id) => self
                    .rec
                    .bound
                    .get(id)
                    .map(|(s, g)| Bound::Value(*s, g.clone())),
                None => Some(Bound::Param),
            }
        };
        let labels = args
            .iter()
            .flat_map(|(e, t)| said(e, &self.u.resolve(t), &bound));
        let d = labels.fold(d, |d, (span, text)| d.with_label(span, text));
        match &f.kind {
            Kind::Prim(name) => match signature(name) {
                Some(sig) => d.with_note(format!("`{name}` has type {sig}")),
                None => d,
            },
            _ => d,
        }
    }
}
