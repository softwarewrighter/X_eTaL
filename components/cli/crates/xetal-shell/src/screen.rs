//! Drawing the shell (the scrollback and the line being typed, or the
//! editor full screen), and a screen as text: the editor's after given
//! keys (`xetal edit --screen`), or any buffer's rows.

use ratatui::Frame;
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::buffer::Buffer;
use ratatui::layout::Position;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use xetal_edit::{Editor, Flow};
use xetal_keys::{Code, Key};

use crate::shell::{Mode, Shell};

impl Shell {
    /// Draw the shell into `frame`: the editor while one is open, else
    /// the last lines that fit and the prompt with the line being typed
    /// (decorated), the cursor in it.
    pub fn draw(&self, frame: &mut Frame) {
        if let Mode::Edit(editor) = &self.mode {
            return editor.draw(frame);
        }
        let area = frame.area();
        let (typed, col) = match self.mode {
            Mode::Prompt => self.line.plain(self.prompt()),
            _ => self.line.draw(self.prompt()),
        };
        let mut lines: Vec<Line> = self.lines.iter().map(|l| crate::line(l)).collect();
        lines.push(crate::line(&typed));
        let skip = lines.len().saturating_sub(usize::from(area.height));
        let row = (lines.len() - skip).saturating_sub(1) as u16;
        let shown: Vec<Line> = lines.into_iter().skip(skip).collect();
        frame.render_widget(Paragraph::new(shown), area);
        let x = (col as u16).min(area.width.saturating_sub(1));
        frame.set_cursor_position(Position::new(area.x + x, area.y + row));
    }
}

/// The rows of `buffer` as text, each row's cells joined, trailing
/// blanks dropped.
pub fn text(buffer: &Buffer) -> Vec<String> {
    let area = buffer.area;
    (0..area.height)
        .map(|y| {
            let row: String = (0..area.width)
                .map(|x| buffer[(area.x + x, area.y + y)].symbol())
                .collect();
            row.trim_end().to_string()
        })
        .collect()
}

/// The editor's screen, `cols` by `rows`, after `keys` (the keys after
/// a quit are not pressed), as text: what `xetal edit` shows, and what
/// the browser's terminal page shows after the same keys.
pub fn screen(mut editor: Editor, keys: &[Key], (cols, rows): (u16, u16)) -> Vec<String> {
    for key in keys {
        if editor.handle(*key) == Flow::Quit {
            break;
        }
    }
    let Ok(mut terminal) = Terminal::new(TestBackend::new(cols, rows));
    let _ = terminal.draw(|f| editor.draw(f));
    text(terminal.backend().buffer())
}

/// A line typed at the REPL, as its prompt showed it: decorated and
/// colored.
pub(crate) fn typed(text: &str) -> String {
    let mut line = xetal_line::Line::default();
    for c in text.chars() {
        line.handle(Key::plain(Code::Char(c)));
    }
    line.draw("").0
}
