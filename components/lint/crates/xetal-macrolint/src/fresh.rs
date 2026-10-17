//! `gensym-unused` and `gensym-not-name`: the fresh names a macro asks
//! for with `[]G_ENSYM` (MC30).

use xetal_base::Diagnostic;
use xetal_ir::{Expr, Kind, Program};
use xetal_lint::uses;

/// The fresh-name findings anywhere in `e`.
pub(crate) fn fresh(program: &Program, e: &Expr) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    walk(program, e, &mut out);
    out
}

fn walk(program: &Program, e: &Expr, out: &mut Vec<Diagnostic>) {
    match &e.kind {
        Kind::Let {
            name, value, body, ..
        } if gensym(value).is_some() => out.extend(unused(program, name, value, body)),
        _ => {}
    }
    if let Some(Expr {
        kind: Kind::Str(asked),
        span,
        ..
    }) = gensym(e)
        && !plain(asked.trim())
    {
        let message = format!(
            "{asked:?} is not a plain name, so []G_ENSYM fails at every call (error[bad-gensym]): write a name such as t or s_um"
        );
        out.push(Diagnostic::warning("gensym-not-name", message).with_span(*span));
    }
    e.children().into_iter().for_each(|c| walk(program, c, out));
}

/// The finding for a fresh name bound to `name` (`_` when the statement
/// throws it away) that `body` never reads.
fn unused(program: &Program, name: &str, value: &Expr, body: &Expr) -> Option<Diagnostic> {
    let message = match name {
        "_" => "a fresh name asked for and thrown away: bind it (t := []G_ENSYM \"t\") and write it into the text the macro gives".to_string(),
        _ if uses(body, name) => return None,
        _ => format!(
            "the fresh name `{name}` is never used: write it into the text the macro gives, or drop the []G_ENSYM"
        ),
    };
    let at = match name {
        "_" => None,
        _ => program.name_span(value.id),
    };
    Some(Diagnostic::warning("gensym-unused", message).with_span(at.unwrap_or(value.span)))
}

/// The argument of `e` when `e` is a call `[]G_ENSYM x`.
fn gensym(e: &Expr) -> Option<&Expr> {
    match &e.kind {
        Kind::App(f, x) if matches!(&f.kind, Kind::Prim(p) if p == "[]G_ENSYM") => Some(x),
        _ => None,
    }
}

/// Whether `name` is a plain name `[]G_ENSYM` takes: a letter, then
/// letters, digits and at most one underline, second (as xetal-system's
/// hook checks it).
fn plain(name: &str) -> bool {
    let b = name.as_bytes();
    let underlines = b.iter().filter(|&&c| c == b'_').count();
    b.first().is_some_and(u8::is_ascii_alphabetic)
        && b.iter().all(|c| c.is_ascii_alphanumeric() || *c == b'_')
        && (underlines == 0 || (underlines == 1 && b.get(1) == Some(&b'_')))
}
