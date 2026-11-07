//! The call stack at a stop: each call in progress, innermost first,
//! named by the text of its call and placed where it was written.

use xetal_base::Span;
use xetal_sources::Sources;
use xetal_step::{Kont, View};

use crate::here::{Place, place};

/// One call in progress: what was called (the function's name as the
/// call writes it), and where the machine is in it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub name: String,
    pub place: Place,
}

/// The frames, innermost first: the stop itself, then each call site
/// out to the top level.
pub(crate) fn frames(view: &View, sources: &Sources, at: Option<Span>) -> Vec<Frame> {
    let calls: Vec<Span> = view
        .stack
        .iter()
        .filter_map(|k| match k {
            Kont::Called { span } => Some(*span),
            _ => None,
        })
        .collect();
    let mut out = Vec::new();
    let mut here = at;
    for call in calls.iter().rev() {
        if let Some(span) = here {
            out.push(Frame {
                name: text(sources, *call),
                place: place(sources, span),
            });
        }
        here = Some(*call);
    }
    if let Some(span) = here {
        out.push(Frame {
            name: "(top level)".into(),
            place: place(sources, span),
        });
    }
    out
}

/// What a call called: the first word of its text (`u:f_`), at most
/// 40 characters.
fn text(sources: &Sources, span: Span) -> String {
    let all = sources.combined();
    let raw = all.get(span.start..span.end.min(all.len())).unwrap_or("");
    let word = raw.split_whitespace().next().unwrap_or("(a call)");
    match word.chars().count() > 40 {
        true => format!("{}...", word.chars().take(37).collect::<String>()),
        false => word.to_string(),
    }
}
