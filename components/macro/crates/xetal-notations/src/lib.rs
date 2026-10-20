//! The notation calls of a program's file (MC32): each call whose macro
//! draws a notation side, with the strings a view replaces by the
//! drawing. The drawing itself was recorded on the call's site when
//! the program was loaded (the `draws:` macro run by the macro phase).

use xetal_base::Span;
use xetal_lex::lex;
use xetal_sources::Sources;

/// A drawn notation call in one of the program's files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notation {
    /// The whole call, as written.
    pub call: Span,
    /// The string literal on each side, when that side is text.
    pub left: Option<Span>,
    pub right: Option<Span>,
    /// The drawing (LaTeX for Equations).
    pub latex: String,
}

/// The notation calls written in file `file` of `sources` (0 is the
/// program's own file) whose drawing was recorded; in order.
pub fn notations(file: usize, sources: &Sources) -> Vec<Notation> {
    let Ok(tokens) = lex(sources.written_text(file)) else {
        return Vec::new();
    };
    let Ok(calls) = xetal_expand::calls(&tokens) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for call in calls {
        let at = call.span.start..call.span.end;
        let mine = |s: &&xetal_sources::Site| s.file == file && s.span == at && s.name == call.name;
        let Some(latex) = sources
            .sites()
            .iter()
            .find(mine)
            .and_then(|s| s.drawn.clone())
        else {
            continue;
        };
        out.push(Notation {
            call: call.span,
            left: call.texts.0.then_some(call.left),
            right: call.texts.1.then_some(call.right),
            latex,
        });
    }
    out
}
