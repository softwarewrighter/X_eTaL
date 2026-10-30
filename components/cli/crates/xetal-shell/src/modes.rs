//! The shell's modes (the prompt, the REPL, the editor) and what it
//! prints: entering and leaving them, the prompt, the kept lines.

use std::path::Path;

use xetal_edit::Editor;
use xetal_repl::Session;

use crate::shell::{KEEP, Mode, Shell};

impl Shell {
    /// The editor quits: its text is kept unless it is the file's as
    /// saved (the text the host handed over may never have been saved).
    pub(crate) fn close(&mut self) {
        if let Mode::Edit(editor) = std::mem::replace(&mut self.mode, Mode::Prompt) {
            let (name, text) = (editor.name(), editor.text());
            let saved = self.catalog.read(&name);
            let same = saved.as_deref().map(|s| s.strip_suffix('\n').unwrap_or(s)) == Some(&text);
            match same && !editor.is_dirty() {
                true => self.buffers.remove(&name),
                false => self.buffers.insert(name, text),
            };
        }
    }

    /// What the page takes back: the file last opened in the editor,
    /// and its text as the shell has it.
    pub fn leave(&mut self) -> (String, String) {
        if matches!(self.mode, Mode::Edit(_)) {
            self.close();
        }
        let name = self.current.clone();
        let text = self.text_of(&name).unwrap_or_default();
        (name, text)
    }

    /// Start the REPL, or the editor on `name` (a missing file is empty).
    pub(crate) fn repl(&mut self) {
        let session = Session::new("-e", self.seed);
        self.mode = Mode::Repl {
            session: Box::new(session),
            more: false,
        };
    }

    pub(crate) fn edit(&mut self, name: &str) {
        let text = self.text_of(name).unwrap_or_default();
        let text = text.strip_suffix('\n').unwrap_or(&text).to_string();
        let libs = Box::new(xetal_macro::StoreLibraries);
        let mut editor = Editor::with_text(Path::new(name), &text, libs);
        self.current = name.to_string();
        editor.seed(self.seed);
        self.mode = Mode::Edit(Box::new(editor));
    }

    /// The prompt for the line being typed.
    pub(crate) fn prompt(&self) -> &'static str {
        match self.mode {
            Mode::Repl { more: true, .. } => "    ...  ",
            Mode::Repl { .. } => "xetal> ",
            _ => "$ ",
        }
    }

    /// The files `ls` lists: the host's and the unsaved buffers, sorted.
    pub(crate) fn files_list(&self) -> Vec<String> {
        let mut names = self.catalog.list();
        names.extend(self.buffers.keys().cloned());
        names.sort();
        names.dedup();
        names
    }

    /// Print `text` (any number of lines, colored or not).
    pub(crate) fn print(&mut self, text: &str) {
        let text = text.strip_suffix('\n').unwrap_or(text);
        if text.is_empty() {
            return;
        }
        self.lines.extend(text.split('\n').map(String::from));
        let over = self.lines.len().saturating_sub(KEEP);
        self.lines.drain(..over);
    }
}
