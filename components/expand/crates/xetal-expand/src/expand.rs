//! Expanding a text: each call replaced by its expansion, which is
//! expanded in turn, to a depth limit.

use xetal_base::{Diagnostic, Span};
use xetal_lex::lex;
use xetal_mapped::Mapped;

use crate::calls::{Call, calls};
use crate::system::{Part, parts};
use crate::user::{Macros, NoMacros, user};

/// How deeply expansions may hold further macro calls.
pub const DEPTH: usize = 32;

/// `text` with its macro calls expanded, mapped back to `text`; errors
/// are located in `text`. Text that does not lex is left as it is (the
/// parser reports it).
pub fn expand(text: &str) -> Result<Mapped, Diagnostic> {
    expand_with(text, &NoMacros)
}

/// [`expand`], with the macros of macro libraries run by `macros`
/// (`alias:n_ame<` calls).
pub fn expand_with(text: &str, macros: &dyn Macros) -> Result<Mapped, Diagnostic> {
    expand_at(&Mapped::new(text), macros, 0)
}

fn expand_at(src: &Mapped, macros: &dyn Macros, depth: usize) -> Result<Mapped, Diagnostic> {
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
        let expanded = expansion(src, call, macros).map_err(located)?;
        out.push(&expand_at(&expanded, macros, depth + 1)?);
        at = call.span.end;
    }
    out.push(&src.slice(at..src.text().len()));
    Ok(out)
}

/// The text `call` stands for, its arguments mapped where they were written.
fn expansion(src: &Mapped, call: &Call, macros: &dyn Macros) -> Result<Mapped, Diagnostic> {
    let (left, right) = (inside(src, call.left), inside(src, call.right));
    let whole = src.span(call.span);
    if let Some((ns, name)) = call.name.split_once(':') {
        let text = macros.run(ns, name, left.text(), right.text());
        return user(call, text, whole);
    }
    let mut out = Mapped::default();
    for part in parts(call, left.text(), right.text())? {
        match part {
            Part::Glue(text) => out.glue(text, whole.start..whole.end),
            Part::Left(range) => out.push(&left.slice(range)),
            Part::Right(range) => out.push(&right.slice(range)),
        }
    }
    Ok(out)
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
