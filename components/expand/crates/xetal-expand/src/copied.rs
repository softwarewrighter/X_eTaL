//! Where the text a macro gives was written: text copied from one of
//! the call's arguments maps back into that argument (so an error in
//! code written inside a string is reported there, and hygiene can tell
//! the call's text from the macro's own, MC30); the rest, the macro's
//! own text, maps to the whole call. An argument found whole is copied
//! there; the pieces of an argument not found whole (one the macro
//! took apart) are runs of at least [`RUN`] bytes.

use std::ops::Range;

use xetal_base::Span;
use xetal_mapped::Mapped;

/// Shortest run of a taken-apart argument taken as copied from it.
const RUN: usize = 3;

/// A copied range of the text: which argument, from where in it.
type Seg = (Range<usize>, usize, usize);

/// `text` mapped: copies of `args` to where they were written, the rest
/// to the call `whole`.
pub(crate) fn copied(text: &str, args: &[&Mapped], whole: Span) -> Mapped {
    let mut segs = whole_args(text, args);
    let found: Vec<bool> = (0..args.len())
        .map(|k| segs.iter().any(|s| s.1 == k))
        .collect();
    let mut gaps = Vec::new();
    let mut at = 0;
    for (range, _, _) in &segs {
        gaps.push(at..range.start);
        at = range.end;
    }
    gaps.push(at..text.len());
    for gap in gaps {
        segs.extend(runs(text, gap, args, &found));
    }
    segs.sort_by_key(|s| s.0.start);
    let (mut out, mut glue) = (Mapped::default(), 0);
    for (range, k, start) in segs {
        if glue < range.start {
            out.glue(&text[glue..range.start], whole.start..whole.end);
        }
        out.push(&args[k].slice(start..start + range.len()));
        glue = range.end;
    }
    if glue < text.len() {
        out.glue(&text[glue..], whole.start..whole.end);
    }
    out
}

/// Every occurrence of a whole argument (the longest arguments first,
/// occurrences not overlapping), in order.
fn whole_args(text: &str, args: &[&Mapped]) -> Vec<Seg> {
    let mut order: Vec<usize> = (0..args.len()).collect();
    order.sort_by_key(|&k| std::cmp::Reverse(args[k].text().len()));
    let mut segs: Vec<Seg> = Vec::new();
    for k in order {
        let arg = args[k].text();
        if arg.is_empty() {
            continue;
        }
        for (at, _) in text.match_indices(arg) {
            let range = at..at + arg.len();
            if !segs
                .iter()
                .any(|s| s.0.start < range.end && range.start < s.0.end)
            {
                segs.push((range, k, 0));
            }
        }
    }
    segs.sort_by_key(|s| s.0.start);
    segs
}

/// The runs of `gap` copied from arguments not found whole.
fn runs(text: &str, gap: Range<usize>, args: &[&Mapped], found: &[bool]) -> Vec<Seg> {
    let (mut out, mut at) = (Vec::new(), gap.start);
    while at < gap.end {
        match longest(&text[at..gap.end], args, found) {
            Some((k, start, len)) => {
                out.push((at..at + len, k, start));
                at += len;
            }
            None => at += text[at..].chars().next().map_or(1, char::len_utf8),
        }
    }
    out
}

/// The longest prefix of `text`, at least [`RUN`] bytes, found in an
/// argument not found whole: which argument, where, how long.
fn longest(text: &str, args: &[&Mapped], found: &[bool]) -> Option<(usize, usize, usize)> {
    let mut best: Option<(usize, usize, usize)> = None;
    for (k, arg) in args.iter().enumerate().filter(|(k, _)| !found[*k]) {
        let source = arg.text();
        for (start, _) in source.char_indices() {
            let len = common(text, &source[start..]);
            if len >= RUN && best.is_none_or(|(_, _, b)| len > b) {
                best = Some((k, start, len));
            }
        }
    }
    best
}

/// The length of the common prefix of `a` and `b`, at a character
/// boundary of both.
fn common(a: &str, b: &str) -> usize {
    let n = a.bytes().zip(b.bytes()).take_while(|(x, y)| x == y).count();
    (0..=n)
        .rev()
        .find(|&i| a.is_char_boundary(i) && b.is_char_boundary(i))
        .unwrap_or(0)
}
