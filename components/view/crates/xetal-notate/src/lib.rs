//! A notation call overlaid on a view (MC32, docs/code-strings.md
//! "Notations"): the program phase recorded, per call whose macro
//! draws a notation side, the drawing (LaTeX) and the string literals
//! it stands for. A view that draws math (HTML, KaTeX) replaces those
//! strings by one math box after the macro's name and the binding, which
//! stay as code; a view that cannot (the terminal) keeps the typed
//! text and marks it as notation. Spans are kept, so the raw-to-drawn
//! map still holds.

use xetal_base::Span;
use xetal_view::{Class, Segment};

/// A notation call: the string literal on each side, when that side is
/// text, and its drawing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notation {
    pub left: Option<Span>,
    pub right: Option<Span>,
    pub latex: String,
}

/// `segments` with each notation's strings drawn: the last of its
/// strings becomes one segment of class `Notation` holding the LaTeX
/// (the math box), the other, if any, an empty segment of that class,
/// so the box follows the macro's name.
pub fn drawn(mut segments: Vec<Segment>, notations: &[Notation]) -> Vec<Segment> {
    for n in notations {
        let (box_at, blank) = match (n.left, n.right) {
            (Some(l), Some(r)) => (r, Some(l)),
            (None, Some(r)) => (r, None),
            (Some(l), None) => (l, None),
            (None, None) => continue,
        };
        for s in &mut segments {
            if s.raw == box_at {
                (s.text, s.class) = (n.latex.clone(), Class::Notation);
            } else if Some(s.raw) == blank {
                (s.text, s.class) = (String::new(), Class::Notation);
            }
        }
    }
    segments
}

/// `segments` with each notation's strings marked as notation, their
/// typed text kept: for views without a math renderer.
pub fn marked(mut segments: Vec<Segment>, notations: &[Notation]) -> Vec<Segment> {
    for n in notations {
        for s in &mut segments {
            if Some(s.raw) == n.left || Some(s.raw) == n.right {
                s.class = Class::Notation;
            }
        }
    }
    segments
}
