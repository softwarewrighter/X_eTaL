//! Decorated Unicode -> raw ASCII. ASCII passes through unchanged, so
//! names shown raw (no superscript glyph) invert trivially.

use xetal_base::{Diagnostic, Span};

use crate::glyphs::{UNDERLINE, from_subscript, from_superscript};

#[derive(Clone, Copy, PartialEq, Eq)]
enum After {
    Plain,
    Underline,
    Sub,
    Super,
}

struct Inverse {
    out: String,
    after: After,
}

/// Convert decorated text back to raw ASCII source.
pub fn undecorate(text: &str) -> Result<String, Diagnostic> {
    let mut inv = Inverse {
        out: String::new(),
        after: After::Plain,
    };
    let mut chars = text.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        if chars.peek().is_some_and(|&(_, n)| n == UNDERLINE) {
            let (j, _) = chars.next().unwrap_or((i, UNDERLINE));
            inv.underlined(c, Span::new(i, j + UNDERLINE.len_utf8()))?;
        } else {
            inv.plain(c, Span::new(i, i + c.len_utf8()))?;
        }
    }
    if inv.after == After::Underline {
        inv.out.push('_');
    }
    Ok(inv.out)
}

fn stem_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || "+-*/=<>|".contains(c)
}

/// Stems are all letters/digits or a single symbol, so an underline
/// run ends after a symbol or where the character class changes.
fn ends_stem(prev: char, next: char) -> bool {
    !prev.is_ascii_alphanumeric() || !next.is_ascii_alphanumeric()
}

impl Inverse {
    fn underlined(&mut self, c: char, span: Span) -> Result<(), Diagnostic> {
        if !stem_char(c) {
            return Err(Diagnostic::new(
                "bad-underline",
                "an underline must be under a name or symbol character",
            )
            .with_span(span));
        }
        if self.after == After::Underline
            && self.out.chars().last().is_some_and(|p| ends_stem(p, c))
        {
            self.out.push('_');
        }
        self.out.push(c);
        self.after = After::Underline;
        Ok(())
    }

    fn plain(&mut self, c: char, span: Span) -> Result<(), Diagnostic> {
        if let Some(d) = from_subscript(c) {
            if self.after != After::Sub {
                self.out.push('_');
            }
            self.out.push(d);
            self.after = After::Sub;
        } else if let Some(l) = from_superscript(c) {
            if self.after != After::Super {
                self.out.push('^');
            }
            self.out.push(l);
            self.after = After::Super;
        } else {
            self.other(c, span)?;
        }
        Ok(())
    }

    fn other(&mut self, c: char, span: Span) -> Result<(), Diagnostic> {
        let niladic = c == '@' && matches!(self.after, After::Underline | After::Super);
        if self.after == After::Underline || niladic {
            self.out.push('_');
        }
        if c == UNDERLINE {
            return Err(
                Diagnostic::new("bad-underline", "an underline must follow a character")
                    .with_span(span),
            );
        }
        if !c.is_ascii() {
            return Err(
                Diagnostic::new("not-decorated", "not ASCII and not a decoration glyph")
                    .with_span(span),
            );
        }
        self.out.push(c);
        self.after = After::Plain;
        Ok(())
    }
}
