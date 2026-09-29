//! The two panes as one widget.

use ratatui::buffer::Buffer as Screen;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph, Widget};
use xetal_buffer::Buffer;
use xetal_view::{column, view, width};

use crate::lines::{ascii, rendered, split};

/// Both panes for one state of the text, scrolled to `scroll`.
pub struct Panes {
    source: Vec<Line<'static>>,
    rendered: Vec<Line<'static>>,
    /// Cursor row (in the text) and its column in each pane.
    cursor: (usize, usize, usize),
    scroll: usize,
}

impl Panes {
    /// The panes for `buffer`, showing rows from `scroll` on.
    pub fn new(buffer: &Buffer, scroll: usize) -> Self {
        let text = buffer.text();
        let lines = split(&view(&text));
        let (row, col) = buffer.cursor();
        let line = &buffer.lines()[row];
        let left = width(&line.chars().take(col).collect::<String>());
        let right = lines.get(row).map_or(0, |l| column(l, buffer.offset()));
        Panes {
            source: lines.iter().map(|l| ascii(&text, l)).collect(),
            rendered: lines.iter().map(|l| rendered(l)).collect(),
            cursor: (row, left, right),
            scroll,
        }
    }

    /// The first row to show so the cursor row stays within `height`.
    pub fn scroll_for(buffer: &Buffer, scroll: usize, height: usize) -> usize {
        let row = buffer.cursor().0;
        let height = height.max(1);
        scroll.min(row).max((row + 1).saturating_sub(height))
    }

    /// Where the cursor is drawn in the source and rendered panes.
    pub fn cursors(&self, area: Rect) -> (Position, Position) {
        let [l, r] = halves(area);
        let y = (self.cursor.0 - self.scroll.min(self.cursor.0)) as u16 + 1;
        let at = |pane: Rect, col: usize| Position::new(pane.x + 1 + col as u16, pane.y + y);
        (at(l, self.cursor.1), at(r, self.cursor.2))
    }
}

fn halves(area: Rect) -> [Rect; 2] {
    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(area)
}

impl Widget for &Panes {
    fn render(self, area: Rect, screen: &mut Screen) {
        let [l, r] = halves(area);
        let at = (self.scroll as u16, 0);
        Paragraph::new(self.source.clone())
            .block(Block::bordered().title(" ASCII "))
            .scroll(at)
            .render(l, screen);
        Paragraph::new(self.rendered.clone())
            .block(Block::bordered().title(" Rendered "))
            .scroll(at)
            .render(r, screen);
    }
}
