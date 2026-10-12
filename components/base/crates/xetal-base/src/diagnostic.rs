//! The `Diagnostic` error value every crate-local error converts into.

use std::fmt;

use crate::Span;

/// Whether a diagnostic stops the program or only informs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Severity {
    #[default]
    Error,
    Warning,
}

/// A machine-applicable suggestion, as rustc gives one: write
/// `replacement` in place of the source behind `span`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fix {
    pub span: Span,
    pub replacement: String,
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
    /// Empty for most; several for a rename (each place the name is
    /// written). A boxed slice keeps `Diagnostic` small.
    pub fix: Box<[Fix]>,
}

impl Diagnostic {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code: code.into(),
            message: message.into(),
            span: None,
            notes: Vec::new(),
            fix: Box::default(),
        }
    }

    /// With a fix: each `(span, replacement)` written in place of its
    /// span, all made together or none.
    pub fn with_fixes(mut self, edits: impl IntoIterator<Item = (Span, String)>) -> Self {
        let fix = edits
            .into_iter()
            .map(|(span, replacement)| Fix { span, replacement });
        self.fix = self.fix.iter().cloned().chain(fix).collect();
        self
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
