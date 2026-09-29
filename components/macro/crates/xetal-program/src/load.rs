//! Loading a program.

use xetal_base::Diagnostic;
use xetal_core::Program;
use xetal_macro::{FsLibraries, expand};
use xetal_sources::Sources;

/// A program with its libraries, in Core, and its source map.
#[derive(Debug)]
pub struct Loaded {
    pub sources: Sources,
    pub program: Program,
}

/// Load the program `text`, reported as `name` (a file path, or `-e`
/// for text given on the command line; libraries are looked for
/// beside it).
pub fn load(name: &str, text: &str) -> Result<Loaded, Diagnostic> {
    let sources = expand(name, text, &FsLibraries::from_env()).map_err(|e| {
        let mut d = e.diagnostic.clone();
        if !e.main {
            (d.message, d.span) = (tail(&e.describe(), &d.code), None);
        }
        d
    })?;
    let program = xetal_core::lower(sources.combined()).map_err(|d| located(&sources, d))?;
    Ok(Loaded { sources, program })
}

/// `d` located in the file it came from: unchanged for one file;
/// otherwise its place is given in the message as `FILE:LINE:COLUMN`.
pub fn located(sources: &Sources, d: Diagnostic) -> Diagnostic {
    if sources.file_count() < 2 || d.span.is_none() {
        return d;
    }
    let mut out = d.clone();
    (out.message, out.span, out.notes) = (tail(&sources.describe(&d), &d.code), None, Vec::new());
    out
}

/// The text of a described diagnostic after `level[code]: `.
fn tail(described: &str, code: &str) -> String {
    let marker = format!("[{code}]: ");
    described.find(&marker).map_or(described.to_string(), |i| {
        described[i + marker.len()..].to_string()
    })
}
