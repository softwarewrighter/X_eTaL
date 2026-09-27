//! Strict evaluator over Core.
//!
//! Stub: the stage is not implemented yet; its error type already
//! converts into `xetal_base::Diagnostic` like every other crate.

use xetal_base::Diagnostic;

/// Pipeline stage name used in diagnostics and by the CLI.
pub const STAGE: &str = "eval";

/// Errors produced by this crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalError {
    /// The stage has not been implemented yet.
    Unsupported,
}

impl From<EvalError> for Diagnostic {
    fn from(err: EvalError) -> Self {
        match err {
            EvalError::Unsupported => Diagnostic::unsupported(STAGE),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsupported_converts_to_a_diagnostic_naming_the_stage() {
        let d: Diagnostic = EvalError::Unsupported.into();
        assert_eq!(d.code, "unsupported");
        assert!(d.message.contains("`eval`"), "{}", d.message);
    }
}
