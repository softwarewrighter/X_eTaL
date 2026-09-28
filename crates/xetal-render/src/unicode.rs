//! Raw ASCII -> decorated Unicode. Everything between tokens (whitespace
//! and comments) is copied verbatim; only function names and literal
//! exponents change.

use xetal_base::Diagnostic;
use xetal_lex::{FuncName, TokenKind, lex};

use crate::glyphs::{UNDERLINE, subscript_digit, superscript_text};

/// Render lexable raw source in decorated form.
pub fn decorate(src: &str) -> Result<String, Diagnostic> {
    let tokens = lex(src)?;
    let mut out = String::new();
    let mut pos = 0;
    for token in &tokens {
        out.push_str(&src[pos..token.span.start]);
        let raw = &src[token.span.start..token.span.end];
        match &token.kind {
            TokenKind::Func(name) => out.push_str(&func_glyphs(name)),
            TokenKind::Exp(_) => {
                out.push_str(&superscript_text(&raw[1..]).unwrap_or_else(|| raw.into()))
            }
            _ => out.push_str(raw),
        }
        pos = token.span.end;
    }
    out.push_str(&src[pos..]);
    Ok(out)
}

fn func_glyphs(name: &FuncName) -> String {
    let mut out = name
        .ns
        .as_ref()
        .map_or(String::new(), |ns| format!("{ns}:"));
    for (i, c) in name.stem.char_indices() {
        out.push(c);
        if i == name.underline {
            out.push(UNDERLINE);
        }
    }
    out.extend(name.mark);
    out.extend(name.axes.iter().map(|d| subscript_digit(*d)));
    out
}
