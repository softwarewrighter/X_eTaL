//! Decorated Unicode -> raw ASCII. ASCII passes through unchanged, so
//! anything shown raw (e.g. `x^0.5`) inverts trivially.

use xetal_base::{Diagnostic, Span};

use crate::glyphs::{UNDERLINE, from_subscript, from_superscript};

#[derive(Clone, Copy, PartialEq, Eq)]
enum After {
    Other,
    Sub,
    Super,
}

/// Convert decorated text back to raw ASCII source.
pub fn undecorate(text: &str) -> Result<String, Diagnostic> {
    let mut out = String::new();
    let mut after = After::Other;
    let mut chars = text.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        if chars.peek().is_some_and(|&(_, n)| n == UNDERLINE) {
            chars.next();
            if !c.is_ascii_alphabetic() {
                return Err(bad_underline(Span::new(
                    i,
                    i + c.len_utf8() + UNDERLINE.len_utf8(),
                )));
            }
            out.push(c);
            out.push('_');
            after = After::Other;
        } else if let Some(d) = from_subscript(c) {
            if after != After::Sub {
                out.push('_');
            }
            out.push(d);
            after = After::Sub;
        } else if let Some(d) = from_superscript(c) {
            if after != After::Super {
                out.push('^');
            }
            out.push(d);
            after = After::Super;
        } else {
            out.push(plain(c, i)?);
            after = After::Other;
        }
    }
    Ok(out)
}

fn plain(c: char, i: usize) -> Result<char, Diagnostic> {
    if c == UNDERLINE {
        return Err(bad_underline(Span::new(i, i + UNDERLINE.len_utf8())));
    }
    if !c.is_ascii() {
        return Err(
            Diagnostic::new("not-decorated", "not ASCII and not a decoration glyph")
                .with_span(Span::new(i, i + c.len_utf8())),
        );
    }
    Ok(c)
}

fn bad_underline(span: Span) -> Diagnostic {
    Diagnostic::new("bad-underline", "an underline must be under a letter").with_span(span)
}
