//! Completion in the ASCII pane: after a character of a name is typed,
//! the candidates for the word at the cursor in a list under it (Tab
//! accepts the chosen one, Up and Down choose, Escape closes; any other
//! key closes it and goes on). Libraries are the standard ones and the
//! `.xtl` files beside the file being edited.

use ratatui::crossterm::event::{KeyCode, KeyEvent};
use xetal_complete::{Completion, Context, complete};

use crate::Editor;
use crate::check::{Bottom, check};

/// The list open under the cursor, and which candidate is chosen.
#[derive(Debug, Clone)]
pub(crate) struct Suggest {
    pub completion: Completion,
    pub selected: usize,
}

/// The library files beside `dir`, as the store names them.
fn beside(dir: &std::path::Path) -> Vec<String> {
    std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok().map(|e| e.file_name().to_string_lossy().into_owned()))
                .filter(|n| n.ends_with(".xtl") || n.ends_with(".xtlm"))
                .collect()
        })
        .unwrap_or_default()
}

impl Editor {
    /// Open the list for the word at the cursor, or close it when
    /// nothing is offered.
    pub(crate) fn suggest(&mut self) {
        let dir = self
            .path
            .parent()
            .map_or_else(|| ".".into(), |d| d.to_path_buf());
        let files = beside(&dir);
        let library = |name: &str| {
            let own = std::fs::read_to_string(dir.join(format!("{name}.xtl"))).ok();
            own.or_else(|| xetal_libs::standard(name).map(String::from))
        };
        let ctx = Context {
            files: &files,
            library: &library,
        };
        let found = complete(&self.buffer.text(), self.buffer.offset(), &ctx);
        self.suggest = found.map(|completion| Suggest {
            completion,
            selected: 0,
        });
    }

    /// A key while the list is open: Tab accepts, Up and Down choose,
    /// Escape closes (all taken); any other key closes it and is not.
    pub(crate) fn choose(&mut self, key: KeyEvent) -> bool {
        let Some(s) = self.suggest.as_mut() else {
            return false;
        };
        let n = s.completion.candidates.len();
        match key.code {
            KeyCode::Tab => self.accept(),
            KeyCode::Down => s.selected = (s.selected + 1) % n,
            KeyCode::Up => s.selected = (s.selected + n - 1) % n,
            KeyCode::Esc => self.suggest = None,
            _ => {
                self.suggest = None;
                return false;
            }
        }
        true
    }

    /// Write the chosen candidate over the word, and check the text.
    fn accept(&mut self) {
        let Some(s) = self.suggest.take() else {
            return;
        };
        let text = self.buffer.text();
        let typed = text[s.completion.start..s.completion.end].chars().count();
        for _ in 0..typed {
            self.buffer.backspace();
        }
        for c in s.completion.candidates[s.selected].insert.chars() {
            self.buffer.insert(c);
        }
        let origin = self.path.display().to_string();
        (self.report, self.bottom) = (check(&self.buffer.text(), &origin), Bottom::Types);
    }
}
