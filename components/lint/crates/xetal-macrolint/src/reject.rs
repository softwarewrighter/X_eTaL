//! `reject-undocumented`, `reject-unplaced` and `reject-bad-place`: the
//! literal codes a macro rejects its call with (`"code place" []R_EJECT
//! message`, MC20), as its users meet them.

use xetal_base::Diagnostic;
use xetal_doccom::Doc;
use xetal_ir::{Expr, Kind};

use crate::macros::Def;

/// The reject-code findings of definition `def`: the places in any
/// definition, the documentation only in a macro's.
pub(crate) fn rejects(def: &Def) -> Vec<Diagnostic> {
    let mut codes = Vec::new();
    literal_codes(def.value, &mut codes);
    let mut out = Vec::new();
    let mut named: Vec<&str> = Vec::new();
    for (written, at) in &codes {
        let (code, place) = written
            .split_once(' ')
            .map_or((&written[..], None), |(c, p)| (c, Some(p)));
        out.extend(placed(code, place, at));
        let Some(name) = def.macro_name else {
            continue;
        };
        if !named.contains(&code) && !mentions(def.doc.as_ref(), code) {
            named.push(code);
            let message = format!(
                "{name} rejects its call with `{code}`, which its doc block does not mention: say when it happens (its users meet it as error[{code}])"
            );
            out.push(Diagnostic::warning("reject-undocumented", message).with_span(at.span));
        }
    }
    out
}

/// The finding for a code written with `place`, if it lands on the
/// macro's name without saying so.
fn placed(code: &str, place: Option<&str>, at: &Expr) -> Option<Diagnostic> {
    let (lint, message) = match place {
        None => (
            "reject-unplaced",
            format!(
                "`{code}` names no place, so the error lands on the macro's name: write \"{code} left\" or \"{code} right\" for the argument at fault, or \"{code} call\""
            ),
        ),
        Some("left" | "right" | "call") => return None,
        Some(other) => (
            "reject-bad-place",
            format!(
                "the place `{other}` is not left, right or call, so the error lands on the macro's name"
            ),
        ),
    };
    Some(Diagnostic::warning(lint, message).with_span(at.span))
}

/// Every `"code place" []R_EJECT message` in `e` whose code is a
/// literal, with that literal's node.
fn literal_codes<'e>(e: &'e Expr, out: &mut Vec<(&'e str, &'e Expr)>) {
    if let Kind::App2 { f, left, .. } = &e.kind
        && matches!(&f.kind, Kind::Prim(p) if p == "[]R_EJECT")
        && let Kind::Str(code) = &left.kind
    {
        out.push((code, left));
    }
    e.children().into_iter().for_each(|c| literal_codes(c, out));
}

/// Whether `doc` names `code` as a word, in its prose or its examples.
fn mentions(doc: Option<&Doc>, code: &str) -> bool {
    let Some(doc) = doc else {
        return false;
    };
    let word = |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_';
    let texts =
        std::iter::once(&doc.text).chain(doc.examples.iter().flat_map(|x| [&x.code, &x.output]));
    texts
        .flat_map(|t| t.split(|c: char| !word(c)))
        .any(|w| w == code)
}
