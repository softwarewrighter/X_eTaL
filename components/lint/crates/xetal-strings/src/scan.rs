//! Running every string lint over a text, and placing an offset in it
//! by line and column as `xetal lint` reports places.

use xetal_base::Diagnostic;

use crate::directive::string_holds_directive;
use crate::literal::literals;
use crate::spelling::{escapes_could_be_raw, needless_raw_hashes, needless_raw_string};

/// The string lints' findings in `text`, a program's file as written:
/// spans (and fixes) are byte offsets in `text`; in text order, then by
/// code. A text that does not lex has none (loading it reports why).
pub fn strings(text: &str) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for lit in literals(text) {
        out.extend(needless_raw_hashes(&lit));
        out.extend(needless_raw_string(&lit));
        out.extend(escapes_could_be_raw(&lit));
        out.extend(string_holds_directive(&lit));
    }
    out.sort_by(|a, b| (a.span.map(|s| s.start), &a.code).cmp(&(b.span.map(|s| s.start), &b.code)));
    out
}

/// The line and column of byte `offset` in `text`, both from 1, the
/// column in characters (as `xetal_sources` locates): an offset inside
/// a character is that character's, one past the text is its end.
pub fn line_col(text: &str, offset: usize) -> (usize, usize) {
    let cut = (0..=offset.min(text.len()))
        .rev()
        .find(|&i| text.is_char_boundary(i))
        .unwrap_or(0);
    let before = text.get(..cut).unwrap_or_default();
    let line_start = before.rfind('\n').map_or(0, |n| n + 1);
    let line = before.get(line_start..).unwrap_or_default();
    (before.matches('\n').count() + 1, line.chars().count() + 1)
}
