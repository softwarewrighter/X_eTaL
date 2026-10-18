//! `macro-capture`: a name a macro writes and binds, neither a gensym
//! nor declared in its `## binds:`, read by the call's own text. The
//! caller wrote that name meaning its own binding and got the macro's.

use xetal_base::{Diagnostic, Span};
use xetal_ir::{Expr, Item, Kind, Param, Program};
use xetal_sources::Sources;

use crate::site::{capture_message, copied_from, gensym, written_by};

/// Every capture in `program`: one finding at each captured read.
pub fn captures(program: &Program, sources: &Sources) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for item in &program.items {
        let (Item::Def { value, .. }
        | Item::Let { value, .. }
        | Item::Set { value, .. }
        | Item::Eval(value)) = item;
        walk(value, &mut |e| out.extend(captured(program, sources, e)));
    }
    out
}

/// Call `f` with `e` and every node below it.
fn walk(e: &Expr, f: &mut dyn FnMut(&Expr)) {
    f(e);
    e.children().into_iter().for_each(|c| walk(c, f));
}

/// The captures by binding node `e`, if a macro wrote its name.
fn captured(program: &Program, sources: &Sources, e: &Expr) -> Vec<Diagnostic> {
    let Some((name, at, scope)) = binding(program, e) else {
        return Vec::new();
    };
    // A name not written anywhere (the shorthand's `_r`) is no capture.
    let Some((site, call)) = at.and_then(|at| written_by(sources, at)) else {
        return Vec::new();
    };
    // A parsing macro binds names of its notation on purpose.
    if site.parsed || gensym(name) || site.binds.iter().any(|b| b == name) {
        return Vec::new();
    }
    let mut reads = Vec::new();
    scope
        .into_iter()
        .for_each(|s| reads_of(s, name, &mut reads));
    let message = capture_message(name, &site.name);
    let captured = reads
        .into_iter()
        .filter(|s| copied_from(sources, *s, &call));
    let found =
        captured.map(|s| Diagnostic::warning("macro-capture", message.clone()).with_span(s));
    found.collect()
}

/// The name binding node `e` binds, where it is written (if recorded)
/// and the nodes it is in scope over.
fn binding<'e>(program: &Program, e: &'e Expr) -> Option<(&'e str, Option<Span>, Vec<&'e Expr>)> {
    match &e.kind {
        Kind::Lam {
            param: Param::Name(p),
            body,
            ..
        } => Some((p, program.param_span(e.id), vec![body])),
        Kind::Let {
            name,
            rec,
            value,
            body,
        } => {
            let scope = [Some(&**body), rec.then_some(&**value)];
            let scope = scope.into_iter().flatten().collect();
            Some((name, program.name_span(value.id), scope))
        }
        _ => None,
    }
}

/// Add to `out` where `e` reads or sets `name`, not counting where an
/// inner binding of the same name hides it.
fn reads_of(e: &Expr, name: &str, out: &mut Vec<Span>) {
    match &e.kind {
        Kind::Var(n) if n == name => out.push(e.span),
        Kind::Lam {
            param: Param::Name(p),
            ..
        } if p == name => {}
        Kind::Let {
            name: n,
            rec,
            value,
            ..
        } if n == name => {
            if !rec {
                reads_of(value, name, out);
            }
        }
        _ => e
            .children()
            .into_iter()
            .for_each(|c| reads_of(c, name, out)),
    }
}
