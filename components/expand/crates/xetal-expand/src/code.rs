//! Code strings checked with the file (ST5, step 112): the inside of
//! every code string is lexed, and of one passed where its macro takes
//! code (`## takes: @ code`) parsed, before any macro runs; an error is
//! placed inside the string, where it was written.

use xetal_base::{Diagnostic, Span};
use xetal_lex::{Token, TokenKind, lex};
use xetal_mapped::Mapped;

/// The inside of the string literal at `span`: escapes replaced, or a
/// raw string's text as written (`r"..."`, `r#"..."#`, ST4).
pub(crate) fn inside(src: &Mapped, span: Span) -> Mapped {
    let written = &src.text()[span.start..span.end];
    // A raw string, or a code string's verbatim form (ST4, ST5): the text
    // between the quotes as it is. A code string's escaped form and a
    // plain string: the escapes resolved.
    let verbatim = written
        .strip_prefix('r')
        .or_else(|| written.strip_prefix("c#").map(|_| &written[1..]));
    match verbatim {
        Some(raw) => {
            let hashes = raw.len() - raw.trim_start_matches('#').len();
            src.slice(span.start + 2 + hashes..span.end - 1 - hashes)
        }
        None => {
            let open = 1 + usize::from(written.starts_with('c'));
            src.slice(span.start + open..span.end - 1).unescape()
        }
    }
}

/// Each code string among `tokens` (of `text`) lexes: the first that
/// does not is its lexer's error, placed in `text`.
pub(crate) fn lexed(text: &str, tokens: &[Token]) -> Result<(), Diagnostic> {
    let whole = Mapped::new(text);
    for t in tokens
        .iter()
        .filter(|t| matches!(t.kind, TokenKind::Code(_)))
    {
        let code = inside(&whole, t.span);
        lex(code.text()).map_err(|d| placed(&code, d.into()))?;
    }
    Ok(())
}

/// The code string at `span` of `src` parses (as a lambda's body,
/// where its macro puts it, so a guard and `_r` may stand): its
/// parser's error, placed in `src`'s text.
pub(crate) fn parsed(src: &str, span: Span) -> Result<(), Diagnostic> {
    let code = inside(&Mapped::new(src), span);
    xetal_syntax::parse_body(code.text())
        .map(drop)
        .map_err(|d| placed(&code, d))
}

/// `d`, found in `code`, placed where `code` was written.
fn placed(code: &Mapped, mut d: Diagnostic) -> Diagnostic {
    d.span = d.span.map(|s| code.span(s));
    d
}
