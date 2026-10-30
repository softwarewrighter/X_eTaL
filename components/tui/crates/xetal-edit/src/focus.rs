//! Which pane has the focus, scrolling a pane by hand, the text the
//! host reads back, and the seed it gives. In the ASCII
//! pane the motion keys move the cursor and both panes follow it; in
//! the Rendered and Output panes they scroll that pane.

use xetal_keys::{Command, PAGE};
use xetal_panes::{Panes, Scroll};

use crate::Editor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    Source,
    Rendered,
    Output,
}

const ORDER: [Pane; 3] = [Pane::Source, Pane::Rendered, Pane::Output];

/// How far a motion key scrolls, as (rows, columns).
pub(crate) fn motion(cmd: Command) -> Option<(isize, isize)> {
    let page = PAGE as isize;
    Some(match cmd {
        Command::Up => (-1, 0),
        Command::Down => (1, 0),
        Command::Left => (0, -1),
        Command::Right => (0, 1),
        Command::PageUp => (-page, 0),
        Command::PageDown => (page, 0),
        Command::Home => (0, isize::MIN),
        Command::End => (0, isize::MAX),
        _ => return None,
    })
}

impl Editor {
    /// The text as it is now (the host takes it back on quit).
    pub fn text(&self) -> String {
        self.buffer.text()
    }

    /// The file's path as it was given (the name a host shows).
    pub fn name(&self) -> String {
        self.path.display().to_string()
    }

    /// Whether the text has changed since it was opened or saved.
    pub fn is_dirty(&self) -> bool {
        self.buffer.is_dirty()
    }

    /// The seed the next run rolls from (a browser host gives one: the
    /// page has no clock to seed from).
    pub fn seed(&mut self, seed: u64) {
        self.seed = Some(seed);
    }

    /// Move the focus `step` panes along (Tab: 1, Shift-Tab: -1).
    pub(crate) fn cycle(&mut self, step: isize) {
        let at = ORDER.iter().position(|p| *p == self.focus).unwrap_or(0) as isize;
        self.focus = ORDER[(at + step).rem_euclid(ORDER.len() as isize) as usize];
    }

    /// Scroll the focused Rendered or Output pane.
    pub(crate) fn scroll_focused(&mut self, delta: (isize, isize)) {
        let cell = match self.focus {
            Pane::Rendered => &self.right,
            _ => &self.out,
        };
        let extent = match self.focus {
            Pane::Rendered => Panes::new(&self.buffer, None, &self.report.notations).extent(),
            _ => (
                self.report.lines.len(),
                self.report
                    .lines
                    .iter()
                    .map(|l| l.chars().count())
                    .max()
                    .unwrap_or(0),
            ),
        };
        cell.set(Scroll::by(cell.get(), delta, extent));
    }
}
