//! `string-kind`: a plain or raw string on a side its macro takes as
//! code, or a code string on a side it takes as text, with the fix
//! that adds or drops the `c` and keeps the value.

use xetal_base::{Diagnostic, Span};

use crate::args::Arg;

/// The `string-kind` findings among `args` (of `text`).
pub(crate) fn string_kind(text: &str, args: &[Arg]) -> Vec<Diagnostic> {
    args.iter()
        .filter(|a| a.takes && a.takes_code != a.code)
        .map(|a| found(text, a))
        .collect()
}

/// The finding for `a`, whose kind is not the kind its side takes.
fn found(text: &str, a: &Arg) -> Diagnostic {
    let side = if a.right { "right" } else { "left" };
    let message = match a.takes_code {
        true => format!(
            "{} takes code on its {side}: write it as a code string, c\"...\", so it is drawn and checked as X_eTaL",
            a.name
        ),
        false => format!(
            "{} takes text on its {side}, not X_eTaL: write it as a plain string",
            a.name
        ),
    };
    let written = &text[a.span.start..a.span.end];
    Diagnostic::warning("string-kind", message)
        .with_span(a.span)
        .with_fixes(respelled(written, a.span))
}

/// The edits that turn the literal `written` at `span` into the other
/// kind, the same value: `"..."` and `c"..."` share their escapes;
/// `r"..."`, `r#"..."#` and `c#"..."#` are verbatim.
fn respelled(written: &str, span: Span) -> Vec<(Span, String)> {
    let at = |from: usize, to: usize| Span::new(span.start + from, span.start + to);
    let hashes = |rest: &str| rest.len() - rest.trim_start_matches('#').len();
    if written.starts_with('"') {
        return vec![(at(0, 0), "c".into())];
    }
    if written.starts_with("c#") {
        return vec![(at(0, 1), "r".into())];
    }
    if written.starts_with("c\"") {
        return vec![(at(0, 1), String::new())];
    }
    // A raw string: c and at least one #, at both ends.
    let h = hashes(&written[1..]);
    let n = h.max(1);
    let end = written.len();
    vec![
        (at(0, 1 + h), format!("c{}", "#".repeat(n))),
        (at(end - h, end), "#".repeat(n)),
    ]
}
