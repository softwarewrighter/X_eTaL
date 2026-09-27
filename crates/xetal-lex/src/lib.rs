//! Lexer: raw ASCII source to decorated tokens with spans.
//!
//! Stub: the stage is not implemented yet; its error type already
//! converts into `xetal_base::Diagnostic` like every other crate.

use xetal_base::Diagnostic;

/// Pipeline stage name used in diagnostics and by the CLI.
pub const STAGE: &str = "lex";

/// Errors produced by this crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexError {
    /// The stage has not been implemented yet.
    Unsupported,
}

impl From<LexError> for Diagnostic {
    fn from(err: LexError) -> Self {
        match err {
            LexError::Unsupported => Diagnostic::unsupported(STAGE),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsupported_converts_to_a_diagnostic_naming_the_stage() {
        let d: Diagnostic = LexError::Unsupported.into();
        assert_eq!(d.code, "unsupported");
        assert!(d.message.contains("`lex`"), "{}", d.message);
    }
}
