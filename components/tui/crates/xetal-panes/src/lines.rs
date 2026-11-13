//! Styled lines for each pane from the view of the whole text.

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

use xetal_view::{Bracket, Class, RAINBOW, Segment, lines, rainbow_on};

use crate::style;

/// A marked byte range (an error's span), drawn on a red background.
pub(crate) type Mark = Option<(usize, usize)>;

/// Per source line: its segments (raw spans are offsets into the text).
pub(crate) fn split(segments: &[Segment]) -> Vec<Vec<Segment>> {
    lines(segments)
}

fn styled(s: &Segment, mark: Mark) -> Style {
    let hit = mark.is_some_and(|(a, b)| s.raw.start < b.max(a + 1) && a < s.raw.end);
    let base = match (rainbow_on(), s.bracket) {
        (true, Some(Bracket::Depth(d))) => style(s.class).fg(Color::Indexed(RAINBOW[d % 4])),
        (true, Some(Bracket::Unmatched)) => style(Class::Error),
        _ => style(s.class),
    };
    match hit {
        true => base.bg(Color::Red),
        false => base,
    }
}

/// The text as typed, highlighted by the class of each segment.
pub(crate) fn ascii(text: &str, line: &[Segment], mark: Mark) -> Line<'static> {
    let spans: Vec<Span<'static>> = line
        .iter()
        .map(|s| Span::styled(text[s.raw.start..s.raw.end].to_string(), styled(s, mark)))
        .collect();
    Line::from(spans)
}

/// The decorated text, highlighted the same way.
pub(crate) fn rendered(line: &[Segment], mark: Mark) -> Line<'static> {
    let spans: Vec<Span<'static>> = line
        .iter()
        .map(|s| Span::styled(s.text.clone(), styled(s, mark)))
        .collect();
    Line::from(spans)
}
