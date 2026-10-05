//! The site with its search written into a directory.

use std::path::Path;

use xetal_base::Diagnostic;
use xetal_doc::DocFile;

use crate::index::{entries, script};

/// The search, as a script (`search.js`).
const SEARCH: &str = include_str!("search.js");

/// `xetal doc --out DIR`: the programs and libraries `inputs` (each
/// a name it is reported by and its text) documented into `dir` as one
/// site with its search; the paths written, one a line.
pub fn document(inputs: &[(String, String)], dir: &Path) -> Result<String, Diagnostic> {
    let files = xetal_doc::models(inputs)?;
    let mut written = xetal_docsite::write(dir, &files)?;
    written.extend(search(dir, &files)?);
    Ok(written.join("\n"))
}

/// The search index of `files` and the search script, written.
fn search(dir: &Path, files: &[DocFile]) -> Result<Vec<String>, Diagnostic> {
    let failed =
        |e: std::io::Error| Diagnostic::new("io", format!("cannot write {}: {e}", dir.display()));
    let parts = [
        ("search-index.js", script(&entries(files))),
        ("search.js", SEARCH.to_string()),
    ];
    let mut written = Vec::new();
    for (path, content) in parts {
        std::fs::write(dir.join(path), content).map_err(failed)?;
        written.push(dir.join(path).display().to_string());
    }
    Ok(written)
}
