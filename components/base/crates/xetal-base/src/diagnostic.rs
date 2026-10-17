//! The `Diagnostic` error value every crate-local error converts into.

use std::fmt;

use crate::shown;
use crate::{Fix, Label, Place, Span};

/// Whether a diagnostic stops the program or only informs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Severity {
    #[default]
    Error,
    Warning,
}

/// A user-facing error or warning: a stable code, a message, an optional
/// primary span, any number of explanatory notes and a fix: the edits
/// that are made together, or none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: String,
    pub message: String,
    pub span: Option<Span>,
    pub notes: Vec<String>,
    /// Its fix, where it was written and the other places it points
    /// at, once located (Saga 27); boxed, as most have none of these,
    /// which keeps `Diagnostic` small.
    pub located: Option<Box<Located>>,
}

/// Where a diagnostic was written, as a reader finds it: its own place
/// (the file, line and column, the line underlined) and its labels'.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Located {
    pub place: Option<Place>,
    /// Other places it points at, each with what it says there.
    pub labels: Vec<Label>,
    /// Empty for most; several for a rename (each place the name is
    /// written).
    pub fix: Vec<Fix>,
}

impl Diagnostic {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code: code.into(),
            message: message.into(),
            span: None,
            notes: Vec::new(),
            located: None,
        }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// A non-fatal diagnostic, printed as `warning[code]`.
    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            ..Self::new(code, message)
        }
    }

    /// The diagnostic every not-yet-implemented pipeline stage reports.
    pub fn unsupported(stage: &str) -> Self {
        Self::new("unsupported", format!("stage `{stage}` is not implemented"))
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level = match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        write!(f, "{level}[{}]: {}", self.code, self.message)?;
        if let Some(place) = self.place() {
            return shown::rest(self, place, f);
        }
        if let Some(span) = self.span {
            write!(f, " at {}..{}", span.start, span.end)?;
        }
        for note in &self.notes {
            match note.strip_prefix("help: ") {
                Some(help) => write!(f, "\n  help: {help}")?,
                None => write!(f, "\n  note: {note}")?,
            }
        }
        Ok(())
    }
}

impl std::error::Error for Diagnostic {}
