//! Expanding a text: each call replaced by its expansion, which is
//! expanded in turn, to a depth limit.

use xetal_base::{Diagnostic, Span};
use xetal_lex::lex;
use xetal_mapped::Mapped;

use std::cell::Cell;

use crate::calls::{Call, calls};
use crate::code::{inside, lexed, lexes, parsed};
use crate::hygiene::{anaphora, unwritten};
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
        // A macro's arguments are checked with its contract (B2).
        let sides: Vec<Span> = calls(&tokens)
            .unwrap_or_default()
            .iter()
            .flat_map(|c| [c.codes.0.then_some(c.left), c.codes.1.then_some(c.right)])
            .flatten()
            .collect();
        lexed(text, &tokens, &sides)?;
    }
    let cx = Cx {
        macros,
        fresh: Cell::new(0),
    };
    expand_at(&Mapped::new(text), &cx, 0)
}

/// The macros, and the count of fresh names given so far (`[]G_ENSYM`).
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
        let whole = expand_at(&expanded, cx, depth + 1)?;
        // Laid out as a program where a line of it is long (D159); the
        // breaks map to the call, as any text the macro added does.
        let at_call = src.span(call.span);
        out.push(&xetal_reflow::laid_out(&whole, at_call.start..at_call.end));
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
    // A code string where the macro takes code parses, elsewhere it
    // lexes (src's places).
    let code = |right: bool, takes_code: bool| {
        let span = if right { call.right } else { call.left };
        match takes_code {
            true => parsed(src.text(), span),
            false => lexes(src.text(), span),
        }
    };
    let (text, binds) = user(
        call,
        (&left, &right),
        cx.macros,
        (src.span(call.span), &cx.fresh, &code),
    )?;
    anaphora(call, (&left, &right), &binds)?;
    Ok(text)
}

/// `d` with its span located in the text as written.
fn located(src: &Mapped, mut d: Diagnostic) -> Diagnostic {
    d.span = d.span.map(|s| src.span(s));
    d
}
