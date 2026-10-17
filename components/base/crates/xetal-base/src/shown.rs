//! A diagnostic located in its file and shown as rustc shows one
//! (Saga 27, D162): the message, then `--> FILE:LINE:COLUMN`, the line
//! with the spot underlined, each label's line with its words, and the
//! notes and hints.

use std::fmt;

use crate::{Diagnostic, Label, Place, Span};

impl Diagnostic {
    /// With a second place it points at, saying `text` there.
    pub fn with_label(mut self, span: Span, text: impl Into<String>) -> Self {
        let text = text.into();
        self.labels_mut().push(Label {
            span,
            text,
            place: None,
        });
        self
    }

    /// With a hint: a note shown as `help:`.
    pub fn with_help(self, help: impl AsRef<str>) -> Self {
        let note = format!("help: {}", help.as_ref());
        self.with_note(note)
    }

    /// Located by `at`, which gives the place of a span: its own span
    /// (unless already located) and each label's.
    pub fn place_with(mut self, at: impl Fn(Span) -> Place) -> Self {
        if let (None, Some(span)) = (self.place(), self.span) {
            self.set_place(Some(at(span)));
        }
        for label in self.labels_mut().iter_mut().filter(|l| l.place.is_none()) {
            label.place = Some(at(label.span));
        }
        self
    }

    /// Located in `text`, the one file a program was read from, which
    /// is reported as `file` (a path, or `-e`).
    pub fn placed_in(self, file: &str, text: &str) -> Self {
        self.place_with(|span| Place::of(file, text, span))
    }

    /// The other places it points at.
    pub fn labels(&self) -> &[Label] {
        self.located.as_ref().map_or(&[], |l| &l.labels)
    }

    /// The other places it points at, to change.
    pub fn labels_mut(&mut self) -> &mut Vec<Label> {
        &mut self.located.get_or_insert_with(Box::default).labels
    }
}

/// The rest of a located diagnostic, after its first line.
pub(crate) fn rest(d: &Diagnostic, place: &Place, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let labels = d
        .labels()
        .iter()
        .filter_map(|l| Some((l.place.as_ref()?, &l.text)));
    let w = labels
        .clone()
        .map(|(p, _)| p.digits())
        .fold(place.digits(), usize::max);
    write!(f, "\n{:w$}--> {place}", "")?;
    place.snippet(f, w, '^', "")?;
    for (at, text) in labels {
        if at.file != place.file {
            write!(f, "\n{:w$}::: {at}", "")?;
        }
        at.snippet(f, w, '-', text)?;
    }
    for note in &d.notes {
        match note.strip_prefix("help: ") {
            Some(help) => write!(f, "\n{:w$} = help: {help}", "")?,
            None => write!(f, "\n{:w$} = note: {note}", "")?,
        }
    }
    Ok(())
}
