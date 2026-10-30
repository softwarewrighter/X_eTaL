//! The line being typed, its history, and how keys change them; Tab
//! completes (tab.rs).

use xetal_buffer::Buffer;
use xetal_complete::Completion;
use xetal_keys::{Code, Command, Key, apply, command};
use xetal_view::{ansi, column, view, width};

/// What a key did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Editing,
    /// Enter: the line, as typed (ASCII).
    Submit(String),
    /// Ctrl-D on an empty line.
    End,
}

#[derive(Debug, Default)]
pub struct Line {
    pub(crate) buffer: Buffer,
    history: Vec<String>,
    /// Position in `history` while recalling; `draft` holds the line
    /// being typed before recall started.
    recall: Option<usize>,
    draft: String,
    /// The completion Tab is cycling through, and which candidate shows.
    pub(crate) cycle: Option<(Completion, usize)>,
}

impl Line {
    /// The line as typed.
    pub fn text(&self) -> String {
        self.buffer.text()
    }

    /// Handle one key (a terminal's event, or a [`Key`] the host made).
    pub fn handle(&mut self, key: impl Into<Key>) -> Outcome {
        let key = key.into();
        let ctrl = key.ctrl;
        if key.code == Code::Tab {
            self.complete();
            return Outcome::Editing;
        }
        self.cycle = None;
        match (key.code, ctrl) {
            (Code::Char('d'), true) if self.text().is_empty() => return Outcome::End,
            (Code::Char('c'), true) => self.replace(""),
            (Code::Enter, _) => return self.submit(),
            (Code::Up, _) => self.step(-1),
            (Code::Down, _) => self.step(1),
            _ => {
                if let Some(cmd) =
                    command(key).filter(|c| !matches!(c, Command::Newline | Command::App(_)))
                {
                    apply(&mut self.buffer, cmd);
                }
            }
        }
        Outcome::Editing
    }

    /// The prompt and the decorated line (ANSI colored), and the
    /// cursor's column.
    pub fn draw(&self, prompt: &str) -> (String, usize) {
        let text = self.text();
        let segments = view(&text);
        let col = width(prompt) + column(&segments, self.buffer.offset());
        (format!("{prompt}{}", ansi(&segments)), col)
    }

    /// The prompt and the line as typed, undecorated (a shell's
    /// command line), and the cursor's column.
    pub fn plain(&self, prompt: &str) -> (String, usize) {
        let text = self.text();
        let before = text[..self.buffer.offset().min(text.len())].chars().count();
        (format!("{prompt}{text}"), width(prompt) + before)
    }

    fn submit(&mut self) -> Outcome {
        let text = self.text();
        if !text.trim().is_empty() && self.history.last() != Some(&text) {
            self.history.push(text.clone());
        }
        (self.recall, self.draft) = (None, String::new());
        self.replace("");
        Outcome::Submit(text)
    }

    /// Recall an older (-1) or newer (+1) history entry.
    fn step(&mut self, by: isize) {
        let len = self.history.len() as isize;
        let at = self.recall.map_or(len, |i| i as isize) + by;
        if at < 0 || at > len {
            return;
        }
        if self.recall.is_none() {
            self.draft = self.text();
        }
        let (recall, text) = match at == len {
            true => (None, self.draft.clone()),
            false => (Some(at as usize), self.history[at as usize].clone()),
        };
        self.recall = recall;
        self.replace(&text);
    }

    fn replace(&mut self, text: &str) {
        self.buffer = Buffer::new(text);
        self.buffer.end();
    }
}
