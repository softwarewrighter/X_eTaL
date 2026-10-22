//! The editor's state and how keys change it.

use std::cell::Cell;
use std::path::{Path, PathBuf};

use xetal_buffer::Buffer;
use xetal_keys::{Action, Command, Key, apply, command};
use xetal_macro::{FsLibraries, Libraries};
use xetal_panes::Scroll;

use crate::check::{Bottom, Report, check, run};
use crate::focus::{Pane, motion};
use crate::suggest::Suggest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
    Continue,
    Quit,
}

pub struct Editor {
    pub(crate) path: PathBuf,
    /// Where the file's libraries come from: the disk beside it, or
    /// the host's store (the browser).
    pub(crate) libs: Box<dyn Libraries>,
    pub(crate) buffer: Buffer,
    pub(crate) focus: Pane,
    /// Scrolls of the ASCII, Rendered and Output panes (the first two
    /// follow the cursor while the ASCII pane has the focus).
    pub(crate) left: Cell<Scroll>,
    pub(crate) right: Cell<Scroll>,
    pub(crate) out: Cell<Scroll>,
    pub(crate) report: Report,
    /// What the bottom pane shows: the types, a run's output or the
    /// lint report.
    pub(crate) bottom: Bottom,
    /// The text with the lint report's fixes made (Ctrl-L again).
    pub(crate) fix: Option<String>,
    /// The completion list open under the cursor.
    pub(crate) suggest: Option<Suggest>,
    /// The seed a run rolls from (the host's; fresh when none).
    pub(crate) seed: Option<u64>,
    pub(crate) status: String,
    /// Only the focused pane is shown, full screen (Ctrl-T).
    pub(crate) zoom: bool,
    pub(crate) confirm_quit: bool,
}

impl Editor {
    /// Edit `path`; a missing file starts empty and is created on save.
    pub fn open(path: &Path) -> std::io::Result<Self> {
        let text = match std::fs::read_to_string(path) {
            Ok(t) => t.strip_suffix('\n').map_or(t.clone(), String::from),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(e) => return Err(e),
        };
        Ok(Editor::with_text(
            path,
            &text,
            Box::new(FsLibraries::from_env()),
        ))
    }

    /// Edit `text` as the file `path`, its libraries found by `libs`
    /// (the browser's editor: the text and the files are the store's).
    pub fn with_text(path: &Path, text: &str, libs: Box<dyn Libraries>) -> Self {
        let fresh = || Cell::new(Scroll::default());
        let report = check(text, &path.display().to_string(), libs.as_ref());
        Editor {
            path: path.into(),
            libs,
            buffer: Buffer::new(text),
            focus: Pane::Source,
            left: fresh(),
            right: fresh(),
            out: fresh(),
            report,
            bottom: Bottom::Types,
            fix: None,
            suggest: None,
            seed: None,
            status: "^S save  ^R run  ^L lint  ^Q quit  Tab pane  ^T zoom".into(),
            zoom: false,
            confirm_quit: false,
        }
    }

    /// Handle one key (a terminal's event, or a [`Key`] the host made).
    pub fn handle(&mut self, key: impl Into<Key>) -> Flow {
        let key = key.into();
        if self.choose(key) {
            return Flow::Continue;
        }
        let Some(cmd) = command(key) else {
            return Flow::Continue;
        };
        match (cmd, motion(cmd)) {
            (Command::App(Action::NextPane), _) => self.cycle(1),
            (Command::App(Action::PrevPane), _) => self.cycle(-1),
            (Command::App(Action::Lint), _) => self.lint(),
            (_, Some(delta)) if self.focus != Pane::Source => self.scroll_focused(delta),
            _ => return self.edit(cmd),
        }
        Flow::Continue
    }

    /// Apply a command to the text (from any pane: typing returns the
    /// focus to the ASCII pane); re-check when the text changed.
    fn edit(&mut self, cmd: Command) -> Flow {
        if !matches!(cmd, Command::App(_)) {
            self.focus = Pane::Source;
        }
        let before = self.buffer.text();
        let flow = match apply(&mut self.buffer, cmd) {
            Some(action) => self.act(action),
            None => Flow::Continue,
        };
        if self.buffer.text() != before {
            (self.report, self.bottom, self.fix) = (
                check(&self.buffer.text(), &self.origin(), self.libs.as_ref()),
                Bottom::Types,
                None,
            );
            self.confirm_quit = false;
            if matches!(cmd, Command::Insert(c) if c.is_ascii_alphanumeric() || "_:[".contains(c)) {
                self.suggest();
            }
        }
        flow
    }

    fn act(&mut self, action: Action) -> Flow {
        match action {
            Action::Save => self.save(),
            Action::Run => {
                let ran = run(&self.buffer.text(), &self.origin(), self.libs.as_ref(), self.seed);
                (self.report, self.bottom) = (ran, Bottom::Output)
            }
            Action::Quit if self.buffer.is_dirty() && !self.confirm_quit => {
                self.confirm_quit = true;
                self.status = "unsaved changes: ^Q again to quit, ^S to save".into();
            }
            Action::Quit => return Flow::Quit,
            Action::Zoom => self.zoom = !self.zoom,
            Action::Lint | Action::NextPane | Action::PrevPane => {}
        }
        Flow::Continue
    }

    /// Where the file's libraries are found from: its own path.
    pub(crate) fn origin(&self) -> String {
        self.path.display().to_string()
    }

    /// Save through the store in use: the disk, or the browser's storage.
    fn save(&mut self) {
        self.status = match xetal_store::write(&self.origin(), &(self.buffer.text() + "\n")) {
            Ok(()) => {
                self.buffer.mark_saved();
                format!("saved {}", self.path.display())
            }
            Err(e) => format!("cannot save {}: {e}", self.path.display()),
        };
        self.confirm_quit = false;
    }
}
