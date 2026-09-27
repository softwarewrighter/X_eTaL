//! Printers: raw <-> decorated Unicode, canonical and expanded forms.
//!
//! Stub: the stage is not implemented yet; its error type already
//! converts into `xetal_base::Diagnostic` like every other crate.

use xetal_base::Diagnostic;

/// Pipeline stage name used in diagnostics and by the CLI.
pub const STAGE: &str = "render";

/// Errors produced by this crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderError {
    /// The stage has not been implemented yet.
    Unsupported,
}

impl From<RenderError> for Diagnostic {
    fn from(err: RenderError) -> Self {
        match err {
            RenderError::Unsupported => Diagnostic::unsupported(STAGE),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsupported_converts_to_a_diagnostic_naming_the_stage() {
        let d: Diagnostic = RenderError::Unsupported.into();
        assert_eq!(d.code, "unsupported");
        assert!(d.message.contains("`render`"), "{}", d.message);
    }
}
