//! Raw ASCII -> decorated Unicode. Whitespace between tokens is kept
//! verbatim, which is what keeps `now_@` and `now_ @` apart.

use xetal_base::Diagnostic;
use xetal_lex::{Name, Sub, TokenKind, lex};

use crate::glyphs::{UNDERLINE, subscript_digit, superscript_word};

/// Render lexable raw source in decorated form.
pub fn decorate(src: &str) -> Result<String, Diagnostic> {
    let tokens = lex(src)?;
    let mut out = String::new();
    let mut pos = 0;
    for token in &tokens {
        out.push_str(&src[pos..token.span.start]);
        let raw = &src[token.span.start..token.span.end];
        match &token.kind {
            TokenKind::Name(name) => out.push_str(&name_glyphs(name).unwrap_or_else(|| raw.into())),
            _ => out.push_str(raw),
        }
        pos = token.span.end;
    }
    out.push_str(&src[pos..]);
    Ok(out)
}

/// `None` when the derivation word has no superscript form; the caller
/// then shows the name raw.
fn name_glyphs(name: &Name) -> Option<String> {
    let deriv = match &name.deriv {
        Some(word) => Some(superscript_word(word)?),
        None => None,
    };
    let mut out = String::new();
    if let Some(ns) = &name.ns {
        out.push_str(ns);
        out.push('.');
    }
    let underlined = deriv.is_none()
        && match name.sub {
            Some(Sub::Niladic) => true,
            Some(_) => !name.symbol,
            None => false,
        };
    for c in name.stem.chars() {
        out.push(c);
        if underlined {
            out.push(UNDERLINE);
        }
    }
    out.extend(deriv);
    match &name.sub {
        Some(Sub::Axes(axes)) => out.extend(axes.iter().map(|d| subscript_digit(*d))),
        Some(Sub::Niladic) => out.push('@'),
        Some(Sub::Bare) | None => {}
    }
    Some(out)
}
