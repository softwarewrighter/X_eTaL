//! A notebook's cells: before each top-level statement runs, the lines
//! from the last cell shown up to that statement's last line are shown
//! (its comments with it); at the end, any lines left after the last
//! statement. Shared by the notebook run all at once (`notebook_to`)
//! and the one stepped a slice at a time, which waits for typed lines.

use xetal_base::Span;
use xetal_program::Loaded;

use crate::Run;
use crate::engine::ready;

/// The text of `src` up to and including statement `upto` (from 1;
/// all of it with None): what Step runs.
pub(crate) fn reach(src: &str, upto: Option<usize>) -> Result<String, Run> {
    let loaded = ready(src)?;
    let ends = crate::notebook::last_lines(&loaded, src);
    Ok(match upto.and_then(|k| ends.get(k.max(1) - 1)) {
        Some(&last) => src.lines().take(last + 1).collect::<Vec<_>>().join("\n"),
        None => src.to_string(),
    })
}

/// The cells still to show, and where to show them.
pub(crate) struct Cells {
    src: String,
    lines: Vec<String>,
    next: usize,
    cell: Box<dyn FnMut(&str) + Send>,
}

impl Cells {
    pub(crate) fn new(src: &str, cell: Box<dyn FnMut(&str) + Send>) -> Self {
        let lines = src.lines().map(String::from).collect();
        let src = src.to_string();
        Cells {
            src,
            lines,
            next: 0,
            cell,
        }
    }

    /// The statement at `span` of `loaded` is about to run: show its cell.
    pub(crate) fn before(&mut self, loaded: &Loaded, span: Span) {
        let at = loaded.sources.locate(span.end.max(1) - 1);
        let line = self.src[..at.offset.min(self.src.len())]
            .matches('\n')
            .count();
        if at.index == 0 && line >= self.next && !self.lines.is_empty() {
            let last = line.min(self.lines.len() - 1);
            (self.cell)(&self.lines[self.next..=last].join("\n"));
            self.next = line + 1;
        }
    }

    /// The run ended: show the lines after the last statement, if any.
    pub(crate) fn rest(&mut self) {
        let rest = self.lines.get(self.next..).unwrap_or_default().join("\n");
        if !rest.trim().is_empty() {
            (self.cell)(rest.trim_end());
        }
        self.next = self.lines.len();
    }
}
