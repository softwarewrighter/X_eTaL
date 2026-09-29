//! Diagnostics reported where they were written.

use xetal_base::{Diagnostic, Severity};

use crate::Sources;

impl Sources {
    /// `d` as printed: unchanged for a one-file program; otherwise its
    /// place is given as `FILE:LINE:COLUMN` in the file it came from.
    pub fn describe(&self, d: &Diagnostic) -> String {
        let (Some(span), true) = (d.span, self.file_count() > 1) else {
            return d.to_string();
        };
        let at = self.locate(span.start);
        let level = match d.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        let mut out = format!(
            "{level}[{}]: {} at {}:{}:{}",
            d.code, d.message, at.file, at.line, at.col
        );
        for note in &d.notes {
            out.push_str(&format!("\n  note: {note}"));
        }
        out
    }
}
