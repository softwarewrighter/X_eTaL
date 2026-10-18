//! Source text -> styled segments, tolerating text that does not lex.

use xetal_base::Span;
use xetal_lex::{TokenKind, lex};
use xetal_render::{code_pieces, token_text};

use crate::Class;
use crate::class::{
    classify, imports_drawn, powers_take_function_class, quotes_take_function_class,
};
use crate::comment::{align, comment};

/// A run of source shown as `text` in style `class`, from bytes `raw`.
#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub raw: Span,
    pub text: String,
    pub class: Class,
}

/// The view of `src`: segments in order, covering every byte once.
pub fn view(src: &str) -> Vec<Segment> {
    let mut out = segments(src);
    align(&mut out, src);
    out
}

/// The segments before comments are aligned.
fn segments(src: &str) -> Vec<Segment> {
    let mut out = Vec::new();
    let mut at = 0;
    while at < src.len() {
        let rest = &src[at..];
        match lex(rest) {
            Ok(_) => {
                valid(rest, at, &mut out);
                break;
            }
            Err(d) => {
                // A code string still being typed (unterminated to the
                // end) is drawn as code, not as an error (ST5).
                let open = Some(d.span)
                    .filter(|s| s.end >= rest.len() && rest[s.start..].starts_with('c'))
                    .and_then(|s| code(&rest[s.start..], at + s.start, true).map(|c| (s.start, c)));
                if let Some((start, inside)) = open {
                    out.extend(segments(&rest[..start]).into_iter().map(|s| shift(s, at)));
                    out.extend(inside);
                    break;
                }
                at = around_error(rest, at, Some(d.span), &mut out);
            }
        }
    }
    out
}

/// Segments for text before an error, the error itself (at least one
/// character) and the offset to continue from.
fn around_error(rest: &str, base: usize, span: Option<Span>, out: &mut Vec<Segment>) -> usize {
    let mut start = span.map_or(0, |s| s.start).min(rest.len());
    while !rest.is_char_boundary(start) {
        start -= 1;
    }
    if start == rest.len() {
        start = rest.char_indices().last().map_or(0, |(i, _)| i);
    }
    out.extend(segments(&rest[..start]).into_iter().map(|s| shift(s, base)));
    let first = rest[start..].chars().next().map_or(1, char::len_utf8);
    let mut end = span.map_or(0, |s| s.end).clamp(start + first, rest.len());
    while !rest.is_char_boundary(end) {
        end += 1;
    }
    let text = rest[start..end].to_string();
    out.push(Segment {
        raw: Span::new(base + start, base + end),
        text,
        class: Class::Error,
    });
    base + end
}

fn shift(mut s: Segment, by: usize) -> Segment {
    s.raw = Span::new(s.raw.start + by, s.raw.end + by);
    s
}

/// Segments for text that lexes: tokens and the gaps between them.
fn valid(src: &str, base: usize, out: &mut Vec<Segment>) {
    let Ok(tokens) = lex(src) else { return };
    let first = out.len();
    let mut pos = 0;
    for t in &tokens {
        gap(src, pos, t.span.start, base, out);
        let raw = &src[t.span.start..t.span.end];
        if let TokenKind::Code(_) = t.kind
            && let Some(inside) = code(raw, base + t.span.start, false)
        {
            out.extend(inside);
            pos = t.span.end;
            continue;
        }
        let (text, class) = (token_text(&t.kind, raw), classify(&t.kind));
        out.push(Segment {
            raw: Span::new(base + t.span.start, base + t.span.end),
            text,
            class,
        });
        pos = t.span.end;
    }
    gap(src, pos, src.len(), base, out);
    powers_take_function_class(&mut out[first..]);
    quotes_take_function_class(&mut out[first..]);
    imports_drawn(&mut out[first..]);
}

/// Whitespace, then a comment if the gap holds one.
fn gap(src: &str, from: usize, to: usize, base: usize, out: &mut Vec<Segment>) {
    let text = &src[from..to];
    let hash = text.find('#').unwrap_or(text.len());
    let end = text[hash..].find('\n').map_or(text.len(), |n| hash + n);
    let space = |a: usize, b: usize, out: &mut Vec<Segment>| {
        if a < b {
            let raw = Span::new(base + from + a, base + from + b);
            out.push(Segment {
                raw,
                text: text[a..b].to_string(),
                class: Class::Space,
            });
        }
    };
    space(0, hash, out);
    if hash < end {
        let line_start = src[..from].is_empty() || src[..from].ends_with('\n');
        comment(&text[hash..end], base + from + hash, line_start, out);
    }
    space(end, text.len(), out);
}

/// A code string's segments (ST5): the pieces xetal-render draws, each
/// classed as the code it is and placed on its bytes; `open` for one
/// still being typed, whose closing quote is supplied to lex it and not
/// shown. `None` when `raw` is not a code string whose inside lexes.
fn code(raw: &str, base: usize, open: bool) -> Option<Vec<Segment>> {
    let hashes = raw.get(1..)?.bytes().take_while(|&b| b == b'#').count();
    let closed = format!("{raw}\"{}", "#".repeat(hashes));
    let mut pieces = code_pieces(if open { &closed } else { raw })?;
    if open {
        pieces.pop();
    }
    let segment = |p: xetal_render::CodePiece| Segment {
        raw: Span::new(base + p.written.start, base + p.written.end),
        text: p.text,
        class: p.kind.as_ref().map_or(Class::Space, classify),
    };
    Some(pieces.into_iter().map(segment).collect())
}
