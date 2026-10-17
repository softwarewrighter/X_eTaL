//! Where a span of the combined text came from: written by a macro (a
//! call in the program's own file) or copied from what the call wrote.

use std::ops::Range;

use xetal_base::Span;
use xetal_sources::{Site, Sources};

/// The call whose macro wrote the text at `span` (in the program's own
/// file, number 0), with the range the call is written over.
pub(crate) fn written_by(sources: &Sources, span: Span) -> Option<(Site, Range<usize>)> {
    let at = sources.locate(span.start);
    if at.index != 0 || at.to <= at.offset + 1 {
        return None;
    }
    let site = sources.site(0, at.offset)?;
    Some((site, at.offset..at.to))
}

/// Whether the text at `span` was copied from the program's own file,
/// from within `call` (the range a call is written over).
pub(crate) fn copied_from(sources: &Sources, span: Span, call: &Range<usize>) -> bool {
    let at = sources.locate(span.start);
    at.index == 0 && at.to == at.offset + 1 && call.contains(&at.offset)
}

/// Whether `name` is a fresh name from `[]G_ENSYM` (`g1:t`, MC30).
pub(crate) fn gensym(name: &str) -> bool {
    name.split_once(':').is_some_and(|(ns, _)| {
        ns.len() > 1 && ns.starts_with('g') && ns[1..].bytes().all(|b| b.is_ascii_digit())
    })
}

/// What `macro-capture` says about `name`, bound by the macro `site`.
pub(crate) fn capture_message(name: &str, site: &str) -> String {
    format!(
        "`{name}` here is bound by {site}, not by your code: the macro writes the name itself (its author can ask for a fresh one with []G_ENSYM, or declare `## binds: {name}` if it is meant for your text)"
    )
}
