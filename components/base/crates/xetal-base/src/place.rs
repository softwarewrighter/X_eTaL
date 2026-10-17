//! Where a diagnostic was written, as a reader finds it (Saga 27):
//! the file, the line and column, and the line itself with the spot
//! underlined, as rustc shows one.

use std::fmt;

use crate::Span;

/// A place in a source file: line and column from 1 (the column in
/// characters), the line's text and how many characters are marked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Place {
    pub file: String,
    pub line: usize,
    pub col: usize,
    pub text: String,
    pub width: usize,
}

/// The label an error unwinding through a call gets at the call (ask
/// E5); shown only for the call a program made into a library.
pub const CALLED_FROM: &str = "called from the program here";

/// A second place a diagnostic points at, with what it says there
/// (where a type came from, D163): its span in the same text as the
/// diagnostic's own, and its place once located.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    pub span: Span,
    pub text: String,
    pub place: Option<Place>,
}

impl Place {
    /// The place of `span` in `text`, a file reported as `file`. A span
    /// past the end marks the last character; one over several lines
    /// is marked to the end of its first line.
    pub fn of(file: &str, text: &str, span: Span) -> Place {
        let start = floor(text, span.start);
        let line_start = text[..start].rfind('\n').map_or(0, |n| n + 1);
        let line_end = text[start..].find('\n').map_or(text.len(), |n| start + n);
        let end = floor(text, span.end.clamp(start, line_end));
        Place {
            file: file.to_string(),
            line: text[..start].matches('\n').count() + 1,
            col: text[line_start..start].chars().count() + 1,
            text: text[line_start..line_end].replace('\t', " "),
            width: text[start..end].chars().count().max(1),
        }
    }

    /// The digits of this place's line number (the gutter's width).
    pub fn digits(&self) -> usize {
        self.line.to_string().len()
    }

    /// Its line, under an empty gutter line.
    pub(crate) fn line_of(&self, f: &mut fmt::Formatter<'_>, w: usize) -> fmt::Result {
        let text = self.text.trim_end();
        let sep = if text.is_empty() { "" } else { " " };
        write!(f, "\n{:w$} |\n{:>w$} |{sep}{text}", "", self.line)
    }

    /// Its mark under its line (`^` for the error, `-` for a label),
    /// followed by `say` when there is something to say.
    pub(crate) fn mark(
        &self,
        f: &mut fmt::Formatter<'_>,
        w: usize,
        mark: char,
        say: &str,
    ) -> fmt::Result {
        let marks = mark.to_string().repeat(self.width);
        let pad = " ".repeat(self.col - 1);
        write!(f, "\n{:w$} | {pad}{marks}", "")?;
        match say.is_empty() {
            true => Ok(()),
            false => write!(f, " {say}"),
        }
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.col)
    }
}

/// The largest character boundary of `text` at or before `at`.
fn floor(text: &str, at: usize) -> usize {
    (0..=at.min(text.len()))
        .rev()
        .find(|&i| text.is_char_boundary(i))
        .unwrap_or(0)
}
