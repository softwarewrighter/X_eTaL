//! Shared foundations for every xetal crate: the language display name,
//! source spans, Core node ids and the `Diagnostic` error value that all
//! crate-local error types convert into.

use std::fmt;

/// Display name of the language. The only place the name is spelled in
/// code; everything else refers to this constant.
pub const LANG_NAME: &str = "X_eTaL";

/// Half-open byte range `start..end` into the source text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Smallest span covering both `self` and `other`.
    pub fn join(self, other: Span) -> Span {
        Span::new(self.start.min(other.start), self.end.max(other.end))
    }
}

/// Identity of a Core node, used to link traces back to source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(pub u32);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lang_name_is_the_display_name() {
        assert_eq!(LANG_NAME, "X_eTaL");
    }

    #[test]
    fn span_join_covers_both() {
        let joined = Span::new(4, 6).join(Span::new(1, 3));
        assert_eq!(joined, Span::new(1, 6));
        assert_eq!(joined.len(), 5);
        assert!(Span::new(3, 3).is_empty());
    }

    #[test]
    fn diagnostic_display_includes_code_span_and_notes() {
        let d = Diagnostic::new("E1", "bad token")
            .with_span(Span::new(2, 4))
            .with_note("try spaces");
        assert_eq!(
            d.to_string(),
            "error[E1]: bad token at 2..4\n  note: try spaces"
        );
    }

    #[test]
    fn unsupported_names_the_stage() {
        let d = Diagnostic::unsupported("lex");
        assert_eq!(d.code, "unsupported");
        assert_eq!(
            d.to_string(),
            "error[unsupported]: stage `lex` is not implemented"
        );
    }
}
