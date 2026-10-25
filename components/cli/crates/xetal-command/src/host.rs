//! What a command needs from where it runs: files by path, the two
//! output streams, and a seed for a run that names none.

use std::io::Write;

use xetal_base::Diagnostic;

/// Files by path, as the host keeps them: the disk, or a browser's store.
pub trait Files {
    /// The text of the file at `path`, or why it cannot be read.
    fn read(&self, path: &str) -> Result<String, String>;
}

/// The process's own files: the disk.
pub struct Local;

impl Files for Local {
    fn read(&self, path: &str) -> Result<String, String> {
        std::fs::read_to_string(path).map_err(|e| e.to_string())
    }
}

/// The host a command runs against.
pub struct Host<'h> {
    pub files: &'h dyn Files,
    pub out: &'h mut (dyn Write + Send),
    pub err: &'h mut (dyn Write + Send),
    /// The seed a run rolls from when neither `--seed` nor `XETAL_SEED`
    /// gives one; none takes a fresh one from the clock (not in a
    /// browser, which has no clock to seed from: it gives one).
    pub seed: Option<u64>,
}

impl Host<'_> {
    /// The source given inline with `-e`, or read from the file.
    pub fn input(&self, expr: Option<&str>, file: Option<&str>) -> Result<String, Diagnostic> {
        match (expr, file) {
            (Some(expr), _) => Ok(expr.to_string()),
            (None, Some(path)) => self
                .files
                .read(path)
                .map_err(|e| Diagnostic::new("io", format!("cannot read {path}: {e}"))),
            (None, None) => Err(Diagnostic::new(
                "no-input",
                "give source with -e EXPR or a FILE path",
            )),
        }
    }

    /// Write a line to the error stream.
    pub(crate) fn warn(&mut self, line: impl std::fmt::Display) {
        let _ = writeln!(self.err, "{line}");
    }
}
