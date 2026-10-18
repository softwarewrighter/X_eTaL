//! Code strings drawn and read back (ST5, docs/code-strings.md): the
//! inside of `c"..."` and `c#"..."#` is drawn as the code around it
//! would be, one level deep; the `c`, the `#`s and the quotes are kept.
//! The escaped form's escapes are resolved before drawing and `\` and
//! `"` escaped again after, so the drawing is itself a code string of
//! the same text, and `undecorate` reverses it. The pieces say where
//! each drawn run was written, for views that style them (xetal-view).

use xetal_base::{Diagnostic, Span};
use xetal_lex::{Token, TokenKind, lex};
use xetal_strtext::{escape, unescape_mapped, unescaped_quote};

use crate::inverse::undecorate;
use crate::unicode::{gap_text, token_text};

/// One run of a drawn code string: the bytes of the token it was written
/// on, what is drawn, and its token (`None` for a gap: spaces or a
/// comment; a string token for the `c"` and the closing quote).
#[derive(Debug, Clone, PartialEq)]
pub struct CodePiece {
    pub written: Span,
    pub text: String,
    pub kind: Option<TokenKind>,
}

/// The drawing of a code string token written `raw`: its pieces joined.
pub(crate) fn drawn(raw: &str) -> String {
    code_pieces(raw).map_or_else(
        || raw.to_string(),
        |p| p.into_iter().map(|p| p.text).collect(),
    )
}

/// The pieces of a code string token written `raw` (`c"..."` or
/// `c#"..."#`), in order, covering every byte of it once; `None` when it
/// is not one. What does not lex inside (a macro's own notation, a slip
/// while typing) is a piece of its own, drawn as typed, with string kind.
pub fn code_pieces(raw: &str) -> Option<Vec<CodePiece>> {
    let (hashes, text, map) = inside(raw)?;
    let (open, close) = (2 + hashes, 1 + hashes);
    let tokens = tolerant(&text);
    let delimiter = |a: usize, b: usize| CodePiece {
        written: Span::new(a, b),
        text: raw[a..b].to_string(),
        kind: Some(TokenKind::Str(String::new())),
    };
    let shown = |s: String| if hashes == 0 { escape(&s) } else { s };
    let place = |a: usize, b: usize| Span::new(open + map[a], open + map[b]);
    let mut out = vec![delimiter(0, open)];
    let (mut pos, mut line_start) = (0, false);
    for t in &tokens {
        if pos < t.span.start {
            let gap = gap_text(&text[pos..t.span.start], line_start);
            out.push(CodePiece {
                written: place(pos, t.span.start),
                text: shown(gap),
                kind: None,
            });
        }
        let piece = &text[t.span.start..t.span.end];
        // Code strings nest one level: one inside is drawn as typed.
        let drawn = match t.kind {
            TokenKind::Code(_) => piece.to_string(),
            _ => token_text(&t.kind, piece),
        };
        out.push(CodePiece {
            written: place(t.span.start, t.span.end),
            text: shown(drawn),
            kind: Some(t.kind.clone()),
        });
        (pos, line_start) = (t.span.end, t.kind == TokenKind::Newline);
    }
    if pos < text.len() {
        let gap = gap_text(&text[pos..], line_start);
        out.push(CodePiece {
            written: place(pos, text.len()),
            text: shown(gap),
            kind: None,
        });
    }
    out.push(delimiter(raw.len() - close, raw.len()));
    Some(out)
}

/// The tokens of `text`, lexed as far as it lexes: a run that does not
/// lex is one string token (drawn as typed) and lexing goes on after it.
fn tolerant(text: &str) -> Vec<Token> {
    let (mut out, mut at) = (Vec::new(), 0);
    let shifted = |t: Token, by: usize| Token {
        span: Span::new(t.span.start + by, t.span.end + by),
        kind: t.kind,
    };
    while at < text.len() {
        let e = match lex(&text[at..]) {
            Ok(tokens) => {
                out.extend(tokens.into_iter().map(|t| shifted(t, at)));
                break;
            }
            Err(e) => e,
        };
        let mut start = (at + e.span.start).min(text.len());
        while !text.is_char_boundary(start) {
            start -= 1;
        }
        let before = lex(&text[at..start]).unwrap_or_default();
        out.extend(before.into_iter().map(|t| shifted(t, at)));
        let mut end = (at + e.span.end).clamp(start + 1, text.len());
        while !text.is_char_boundary(end) {
            end += 1;
        }
        let kind = TokenKind::Str(String::new());
        out.push(Token {
            kind,
            span: Span::new(start, end),
        });
        at = end;
    }
    out
}

/// A code string's number of `#`, its text (escapes resolved in the
/// escaped form) and, for each byte of the text and one past its end,
/// the offset between the quotes it was written at.
fn inside(raw: &str) -> Option<(usize, String, Vec<usize>)> {
    let hashes = raw
        .strip_prefix('c')?
        .bytes()
        .take_while(|&b| b == b'#')
        .count();
    let written = raw.get(2 + hashes..raw.len().checked_sub(1 + hashes)?)?;
    Some(match hashes {
        0 => {
            let (text, map) = unescape_mapped(written);
            (0, text, map)
        }
        _ => (hashes, written.to_string(), (0..=written.len()).collect()),
    })
}

/// When `rest` starts with a drawn code string: its length in bytes and
/// its source. `None` when it does not start with one.
pub(crate) fn undrawn(rest: &str) -> Option<Result<(usize, String), Diagnostic>> {
    let hashes = rest
        .strip_prefix('c')?
        .bytes()
        .take_while(|&b| b == b'#')
        .count();
    let body = rest
        .get(2 + hashes..)
        .filter(|_| rest.as_bytes().get(1 + hashes) == Some(&b'"'))?;
    let closer = format!("\"{}", "#".repeat(hashes));
    let end = match hashes {
        0 => unescaped_quote(body)?,
        _ => body.find(&closer)?,
    };
    let inner = &body[..end];
    let source = match hashes {
        0 => undecorate(&unescape_mapped(inner).0).map(|s| escape(&s)),
        _ => undecorate(inner),
    };
    let open = &rest[..2 + hashes];
    Some(source.map(|s| {
        (
            2 + hashes + end + closer.len(),
            format!("{open}{s}{closer}"),
        )
    }))
}
