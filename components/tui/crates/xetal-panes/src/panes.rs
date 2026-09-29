//! The two panes as one widget, each with its own scroll.

use ratatui::buffer::Buffer as Screen;
use ratatui::layout::{Constraint, Layout, Position, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Paragraph, Widget};
use xetal_buffer::Buffer;
use xetal_view::{column, view, width};

use crate::Scroll;
use crate::lines::{ascii, rendered, split};

/// Which pane has the focus (drawn with a highlighted border).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Source,
    Rendered,
}

/// Both panes for one state of the text.
pub struct Panes {
    source: Vec<Line<'static>>,
    rendered: Vec<Line<'static>>,
    /// Cursor row (in the text) and its column in each pane.
    cursor: (usize, usize, usize),
    pub left: Scroll,
    pub right: Scroll,
    pub focus: Option<Focus>,
}

impl Panes {
    /// The panes for `buffer`, unscrolled, with the byte range `mark`
    /// (an error's span) highlighted in both.
    pub fn new(buffer: &Buffer, mark: Option<(usize, usize)>) -> Self {
        let text = buffer.text();
        let lines = split(&view(&text));
        let (row, col) = buffer.cursor();
        let left = width(&buffer.lines()[row].chars().take(col).collect::<String>());
        let right = lines.get(row).map_or(0, |l| column(l, buffer.offset()));
        Panes {
            source: lines.iter().map(|l| ascii(&text, l, mark)).collect(),
            rendered: lines.iter().map(|l| rendered(l, mark)).collect(),
            cursor: (row, left, right),
            left: Scroll::default(),
            right: Scroll::default(),
            focus: None,
        }
    }

    /// Both scrolls moved just enough to show the cursor in `area`.
    pub fn follow(&mut self, area: Rect) {
        let [l, r] = halves(area).map(|p| {
            (
                p.height.saturating_sub(2) as usize,
                p.width.saturating_sub(2) as usize,
            )
        });
        self.left = self.left.follow((self.cursor.0, self.cursor.1), l);
        self.right = self.right.follow((self.cursor.0, self.cursor.2), r);
    }

    /// Where the cursor is drawn in the source and rendered panes.
    pub fn cursors(&self, area: Rect) -> (Position, Position) {
        let [l, r] = halves(area);
        let at = |pane: Rect, s: Scroll, col: usize| {
            let (y, x) = (
                self.cursor.0.saturating_sub(s.row),
                col.saturating_sub(s.col),
            );
            Position::new(pane.x + 1 + x as u16, pane.y + 1 + y as u16)
        };
        (
            at(l, self.left, self.cursor.1),
            at(r, self.right, self.cursor.2),
        )
    }

    /// Rows of text and the widest rendered line (for scroll limits).
    pub fn extent(&self) -> (usize, usize) {
        (
            self.rendered.len(),
            self.rendered.iter().map(Line::width).max().unwrap_or(0),
        )
    }
}

fn halves(area: Rect) -> [Rect; 2] {
    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(area)
}

impl Widget for &Panes {
    fn render(self, area: Rect, screen: &mut Screen) {
        let [l, r] = halves(area);
        let block = |title: &'static str, pane: Focus| {
            let lit = Style::default().fg(if self.focus == Some(pane) {
                Color::Yellow
            } else {
                Color::Reset
            });
            Block::bordered().title(title).border_style(lit)
        };
        let at = |s: Scroll| (s.row as u16, s.col as u16);
        let left = Paragraph::new(self.source.clone()).block(block(" ASCII ", Focus::Source));
        left.scroll(at(self.left)).render(l, screen);
        let right =
            Paragraph::new(self.rendered.clone()).block(block(" Rendered ", Focus::Rendered));
        right.scroll(at(self.right)).render(r, screen);
    }
}
