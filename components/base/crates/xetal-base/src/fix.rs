//! A diagnostic's fix: the edits that are made together, or none.

use crate::{Diagnostic, Place, Span};

/// A machine-applicable suggestion, as rustc gives one: write
/// `replacement` in place of the source behind `span`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fix {
    pub span: Span,
    pub replacement: String,
}

impl Diagnostic {
    /// With a fix: each `(span, replacement)` written in place of its
    /// span, all made together or none.
    pub fn with_fixes(mut self, edits: impl IntoIterator<Item = (Span, String)>) -> Self {
        let fix = edits
            .into_iter()
            .map(|(span, replacement)| Fix { span, replacement });
        let fixes = &mut self.located.get_or_insert_with(Box::default).fix;
        fixes.extend(fix);
        self
    }

    /// Its fix: none for most.
    pub fn fix(&self) -> &[Fix] {
        self.located.as_ref().map_or(&[], |l| &l.fix)
    }
}

impl Diagnostic {
    /// Where it was written, once located.
    pub fn place(&self) -> Option<&Place> {
        self.located.as_ref()?.place.as_ref()
    }

    /// Located at `place` (or no longer, given none).
    pub fn set_place(&mut self, place: Option<Place>) {
        self.located.get_or_insert_with(Box::default).place = place;
    }
}
