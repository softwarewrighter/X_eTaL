//! The `Diagnostic` error value every crate-local error converts into.

use std::fmt;

use crate::Span;

/// A user-facing error: a stable code, a message, an optional primary
/// span and any number of explanatory notes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub code: String,
    pub message: String,
    pub span: Option<Span>,
    pub notes: Vec<String>,
}

impl Diagnostic {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            span: None,
            notes: Vec::new(),
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

    /// The diagnostic every not-yet-implemented pipeline stage reports.
    pub fn unsupported(stage: &str) -> Self {
        Self::new("unsupported", format!("stage `{stage}` is not implemented"))
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error[{}]: {}", self.code, self.message)?;
        if let Some(span) = self.span {
            write!(f, " at {}..{}", span.start, span.end)?;
        }
        for note in &self.notes {
            write!(f, "\n  note: {note}")?;
        }
        Ok(())
    }
}

impl std::error::Error for Diagnostic {}
