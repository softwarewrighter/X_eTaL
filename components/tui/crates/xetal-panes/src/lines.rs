//! Styled lines for each pane from the view of the whole text.

use ratatui::text::{Line, Span};
use xetal_view::{Segment, lines};

use crate::style;

/// Per source line: its segments (raw spans are offsets into `text`).
pub(crate) fn split(segments: &[Segment]) -> Vec<Vec<Segment>> {
    lines(segments)
}

/// The text as typed, highlighted by the class of each segment.
pub(crate) fn ascii(text: &str, line: &[Segment]) -> Line<'static> {
    let spans: Vec<Span<'static>> = line
        .iter()
        .map(|s| Span::styled(text[s.raw.start..s.raw.end].to_string(), style(s.class)))
        .collect();
    Line::from(spans)
}

/// The decorated text, highlighted the same way.
pub(crate) fn rendered(line: &[Segment]) -> Line<'static> {
    let spans: Vec<Span<'static>> = line
        .iter()
        .map(|s| Span::styled(s.text.clone(), style(s.class)))
        .collect();
    Line::from(spans)
}
