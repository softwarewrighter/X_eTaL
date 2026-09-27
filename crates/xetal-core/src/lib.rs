//! Core IR and desugaring of the surface AST.
//!
//! Stub: the stage is not implemented yet; its error type already
//! converts into `xetal_base::Diagnostic` like every other crate.

use xetal_base::Diagnostic;

/// Pipeline stage name used in diagnostics and by the CLI.
pub const STAGE: &str = "core";

/// Errors produced by this crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DesugarError {
    /// The stage has not been implemented yet.
    Unsupported,
}

impl From<DesugarError> for Diagnostic {
    fn from(err: DesugarError) -> Self {
        match err {
            DesugarError::Unsupported => Diagnostic::unsupported(STAGE),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsupported_converts_to_a_diagnostic_naming_the_stage() {
        let d: Diagnostic = DesugarError::Unsupported.into();
        assert_eq!(d.code, "unsupported");
        assert!(d.message.contains("`core`"), "{}", d.message);
    }
}
