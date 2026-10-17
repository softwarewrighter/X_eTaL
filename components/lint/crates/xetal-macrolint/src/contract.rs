//! `takes-mismatch`, `ignored-argument` and `binds-unwritten`: a
//! macro's contract (MC31) that its own body contradicts, found before
//! any call meets it.

use xetal_base::Diagnostic;
use xetal_contract::Uses;
use xetal_ir::{Expr, Kind, Param, Program};
use xetal_lint::uses;

use crate::macros::Def;

/// A macro's two parameters, left then right: each lambda with its
/// name (none for `@`).
type Sides<'p> = [(&'p Expr, Option<&'p str>); 2];

/// The contract findings of definition `def`, when it is a macro.
pub(crate) fn contract(program: &Program, def: &Def) -> Vec<Diagnostic> {
    let (Some(name), Some(c)) = (def.macro_name, def.contract.as_ref()) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for (i, (lam, param)) in sides(def.value).into_iter().flatten().enumerate() {
        let side = ["left", "right"][i];
        let at = program.param_span(lam.id).unwrap_or(lam.span);
        let text = c.takes.map(|t| [t.0, t.1][i]);
        let found = match mismatch(name, side, text, param) {
            Some(message) => Some(("takes-mismatch", message)),
            None => ignored(name, side, c.uses, (lam, param)),
        };
        out.extend(found.map(|(lint, m)| Diagnostic::warning(lint, m).with_span(at)));
    }
    out.extend(c.binds.iter().filter(|b| !writes(def.value, b)).map(|b| {
        let message = format!(
            "{name} declares ## binds: {b}, but no text it writes has `{b}`: write the binding, or drop it from the line"
        );
        let at = program.name_span(def.value.id).unwrap_or(def.value.span);
        Diagnostic::warning("binds-unwritten", message).with_span(at)
    }));
    out
}

/// The finding for a copying macro's named side (`lam` binds `param`)
/// that its body never reads.
fn ignored(
    name: &str,
    side: &str,
    how: Uses,
    (lam, param): (&Expr, Option<&str>),
) -> Option<(&'static str, String)> {
    let (Uses::Copied, Some(p), Kind::Lam { body, .. }) = (how, param, &lam.kind) else {
        return None;
    };
    let message = format!(
        "{name} copies its text (## uses: copied) but never reads `{p}`, so every call with text on its {side} fails with unused-argument: take @ there (## takes:), or say ## uses: parsed"
    );
    (!uses(body, p)).then_some(("ignored-argument", message))
}

/// What is wrong when `## takes:` says a side is `text` (or `@`) and
/// its parameter is `param` (a name, or `@` for none).
fn mismatch(name: &str, side: &str, text: Option<bool>, param: Option<&str>) -> Option<String> {
    match (text?, param) {
        (true, None) => Some(format!(
            "## takes: says {name} takes text on its {side}, but its parameter there is @, which receives no text: name it, or write @ in the takes: line"
        )),
        (false, Some(p)) => Some(format!(
            "## takes: says {name} takes @ on its {side}, but its parameter there is named `{p}`: write @ for it, or text in the takes: line"
        )),
        _ => None,
    }
}

/// The two parameters of a macro written `{ l r -> ... }`.
fn sides(value: &Expr) -> Option<Sides<'_>> {
    let (left, inner) = param(value)?;
    let (right, _) = param(inner)?;
    Some([(value, left), (inner, right)])
}

/// The parameter of lambda `e` (none for `@`) and its body.
fn param(e: &Expr) -> Option<(Option<&str>, &Expr)> {
    let Kind::Lam { param, body, .. } = &e.kind else {
        return None;
    };
    let name = match param {
        Param::Name(n) => Some(n.as_str()),
        _ => None,
    };
    Some((name, body))
}

/// Whether a string literal anywhere in `e` has `name` as a word.
fn writes(e: &Expr, name: &str) -> bool {
    let word = |c: char| c.is_ascii_alphanumeric() || c == '_' || c == '!';
    match &e.kind {
        Kind::Str(s) => s.split(|c: char| !word(c)).any(|w| w == name),
        _ => e.children().into_iter().any(|c| writes(c, name)),
    }
}
