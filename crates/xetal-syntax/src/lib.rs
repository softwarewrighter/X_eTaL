//! Parser: tokens to the surface AST, reporting ambiguous expressions.
//!
//! Stub: the stage is not implemented yet; its error type already
//! converts into `xetal_base::Diagnostic` like every other crate.

use xetal_base::Diagnostic;

/// Pipeline stage name used in diagnostics and by the CLI.
pub const STAGE: &str = "parse";

/// Errors produced by this crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The stage has not been implemented yet.
    Unsupported,
}

impl From<ParseError> for Diagnostic {
    fn from(err: ParseError) -> Self {
        match err {
            ParseError::Unsupported => Diagnostic::unsupported(STAGE),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsupported_converts_to_a_diagnostic_naming_the_stage() {
        let d: Diagnostic = ParseError::Unsupported.into();
        assert_eq!(d.code, "unsupported");
        assert!(d.message.contains("`parse`"), "{}", d.message);
    }
}
