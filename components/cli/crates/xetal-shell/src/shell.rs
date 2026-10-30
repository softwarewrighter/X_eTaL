//! The shell's state and how keys change it: a prompt, the REPL, or the
//! editor; what is printed is kept as lines (with their colors).

use std::collections::BTreeMap;

use xetal_edit::Editor;
use xetal_keys::Key;
use xetal_line::{Line, Outcome};
use xetal_repl::{Reply, Session};

/// The host's files: the text of a path, and the paths to list.
pub trait Catalog {
    fn read(&self, path: &str) -> Option<String>;
    fn list(&self) -> Vec<String>;
}

/// What a key did to the page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
    Continue,
    /// `exit`, or Ctrl-D at an empty prompt: back to the host's page.
    Exit,
}

pub(crate) enum Mode {
    Prompt,
    Repl { session: Box<Session>, more: bool },
    Edit(Box<Editor>),
}

/// How many printed lines are kept.
pub(crate) const KEEP: usize = 2000;

pub struct Shell {
    pub(crate) lines: Vec<String>,
    pub(crate) line: Line,
    pub(crate) mode: Mode,
    /// Text edited and not saved, by path: read before the host's files.
    pub(crate) buffers: BTreeMap<String, String>,
    pub(crate) catalog: Box<dyn Catalog>,
    pub(crate) seed: u64,
    /// The file last opened in the editor: the page's file.
    pub(crate) current: String,
}

impl Shell {
    /// A shell over `catalog`, the file `name` holding `text` (the host's
    /// unsaved text) open in the editor, as `xetal edit NAME` opens it.
    pub fn new(name: &str, text: &str, catalog: Box<dyn Catalog>) -> Shell {
        let mut shell = Shell {
            lines: Vec::new(),
            line: Line::default(),
            mode: Mode::Prompt,
            buffers: BTreeMap::from([(name.to_string(), text.to_string())]),
            catalog,
            seed: 0,
            current: name.to_string(),
        };
        shell.submit(&format!("xetal edit {}", crate::words::quote(name)));
        shell
    }

    /// The seed the next run rolls from (a browser has no clock to seed
    /// from: the host gives one before each key).
    pub fn seed(&mut self, seed: u64) {
        self.seed = seed;
        if let Mode::Edit(editor) = &mut self.mode {
            editor.seed(seed);
        }
    }

    /// Handle one key.
    pub fn handle(&mut self, key: Key) -> Flow {
        if let Mode::Edit(editor) = &mut self.mode {
            if editor.handle(key) == xetal_edit::Flow::Quit {
                self.close();
            }
            return Flow::Continue;
        }
        match self.line.handle(key) {
            Outcome::Editing => Flow::Continue,
            Outcome::Submit(text) => self.submit(&text),
            Outcome::End => self.end(),
        }
    }

    /// Open `name` in the editor, as `xetal edit NAME` does (an open
    /// editor's text is kept first, saved or not).
    pub fn open(&mut self, name: &str) {
        if matches!(self.mode, Mode::Edit(_)) {
            self.close();
        }
        self.submit(&format!("xetal edit {}", crate::words::quote(name)));
    }

    /// The text of `name` as the shell has it: an open editor's, a
    /// buffer's, else the host's file.
    pub fn text_of(&self, name: &str) -> Option<String> {
        if let Mode::Edit(editor) = &self.mode
            && editor.name() == name
        {
            return Some(editor.text());
        }
        (self.buffers.get(name).cloned()).or_else(|| self.catalog.read(name))
    }

    /// Run a line typed at the prompt (or at the REPL's).
    fn submit(&mut self, text: &str) -> Flow {
        let prompt = self.prompt().to_string();
        let shown = match self.mode {
            Mode::Repl { .. } => crate::screen::typed(text),
            _ => text.to_string(),
        };
        self.print(&format!("{prompt}{shown}"));
        let Mode::Repl { session, more } = &mut self.mode else {
            return crate::exec::command(self, text);
        };
        let reply = session.feed(text);
        *more = reply == Reply::More;
        if let Reply::Done { out, err } = reply {
            self.print(&out);
            self.print(&crate::ansi::red(&err));
        }
        Flow::Continue
    }

    /// Ctrl-D at an empty line: the REPL ends, or the shell does.
    fn end(&mut self) -> Flow {
        match self.mode {
            Mode::Repl { .. } => {
                self.mode = Mode::Prompt;
                Flow::Continue
            }
            _ => Flow::Exit,
        }
    }
}
