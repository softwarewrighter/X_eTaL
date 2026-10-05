//! A file expanded with its calls remembered.

use std::cell::RefCell;
use std::collections::HashMap;

use crate::Expansion;
use crate::record::Recorder;
use crate::tree::tree;

/// The expansions of the calls in file `text` (reported as `name`, its
/// libraries found beside it).
pub fn expansions(name: &str, text: &str) -> Result<Vec<Expansion>, xetal_base::Diagnostic> {
    let libs = xetal_macro::FsLibraries::from_env();
    let recorder = Recorder {
        inner: xetal_program::Running(&libs),
        seen: RefCell::new(HashMap::new()),
    };
    xetal_macro::expansion(name, text, &recorder).map_err(|e| e.diagnostic)?;
    Ok(tree(text, &recorder.seen.borrow(), 0))
}
