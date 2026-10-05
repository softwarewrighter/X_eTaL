//! Expanding a text: each call replaced by its expansion, which is
//! expanded in turn, to a depth limit.

use xetal_base::{Diagnostic, Span};
use xetal_lex::lex;
use xetal_mapped::Mapped;

use std::cell::Cell;

use crate::calls::{Call, calls};
use crate::hygiene::{hygiene, unwritten};
use crate::user::{Macros, NoMacros, user};

/// How deeply expansions may hold further macro calls.
pub const DEPTH: usize = 32;

/// `text` with no macros to call: a text without macro calls comes
/// back as it is, any call is unknown.
pub fn expand(text: &str) -> Result<Mapped, Diagnostic> {
    expand_with(text, &NoMacros)
}

/// `text` with its macro calls expanded by `macros`, mapped back to
/// `text`; errors are located in `text`. Text that does not lex is left
/// as it is (the parser reports it).
pub fn expand_with(text: &str, macros: &dyn Macros) -> Result<Mapped, Diagnostic> {
    if let Ok(tokens) = lex(text) {
        unwritten(&tokens)?;
    }
    let cx = Cx {
        macros,
        fresh: Cell::new(0),
    };
    expand_at(&Mapped::new(text), &cx, 0)
}

/// The macros, and the count of fresh names given so far (hygiene).
struct Cx<'a> {
    macros: &'a dyn Macros,
    fresh: Cell<usize>,
}

fn expand_at(src: &Mapped, cx: &Cx, depth: usize) -> Result<Mapped, Diagnostic> {
    let Ok(tokens) = lex(src.text()) else {
        return Ok(src.clone());
    };
    let located = |d: Diagnostic| located(src, d);
    let found = calls(&tokens).map_err(located)?;
    if let (Some(first), true) = (found.first(), depth >= DEPTH) {
        let message = format!("macro expansion is nested more than {DEPTH} deep");
        return Err(located(
            Diagnostic::new("macro-depth", message).with_span(first.token),
        ));
    }
    let (mut out, mut at) = (Mapped::default(), 0);
    for call in &found {
        out.push(&src.slice(at..call.span.start));
        let expanded = expansion(src, call, cx).map_err(located)?;
        out.push(&expand_at(&expanded, cx, depth + 1)?);
        at = call.span.end;
    }
    out.push(&src.slice(at..src.text().len()));
    Ok(out)
}

/// The text `call` stands for, its arguments mapped where they were written.
fn expansion(src: &Mapped, call: &Call, cx: &Cx) -> Result<Mapped, Diagnostic> {
    let side = |text: bool, span: Span| match text {
        true => inside(src, span),
        false => Mapped::default(),
    };
    let (left, right) = (
        side(call.texts.0, call.left),
        side(call.texts.1, call.right),
    );
    let (text, binds) = user(call, (&left, &right), cx.macros, src.span(call.span))?;
    Ok(hygiene(&text, &binds, &cx.fresh))
}

/// The inside of the string literal at `span`, escapes replaced.
fn inside(src: &Mapped, span: Span) -> Mapped {
    src.slice(span.start + 1..span.end - 1).unescape()
}

/// `d` with its span located in the text as written.
fn located(src: &Mapped, mut d: Diagnostic) -> Diagnostic {
    d.span = d.span.map(|s| src.span(s));
    d
}
