//! The libraries a file is expanded with, remembering what each macro
//! call gave: the program's own runner (`xetal-program`'s `Running`),
//! watched.

use std::cell::RefCell;
use std::collections::HashMap;

use xetal_base::Diagnostic;
use xetal_macro::{Found, Libraries, MacroRun, Pair};
use xetal_program::Running;
use xetal_sources::Sources;

/// A call as its key: whether it is a statement, and the call written
/// with its sides quoted (`"c" i_f< "a; b"`, `@ l_ine< @`).
pub(crate) type Key = (bool, String);

/// Libraries that run macros and remember each call's text.
pub(crate) struct Recorder<'a> {
    pub inner: Running<'a>,
    pub seen: RefCell<HashMap<Key, String>>,
}

impl Libraries for Recorder<'_> {
    fn find(&self, spec: &str, from: &str) -> Option<Found> {
        self.inner.find(spec, from)
    }

    fn find_both(&self, spec: &str, from: &str) -> Pair {
        self.inner.find_both(spec, from)
    }

    fn run_macro(&self, library: &Sources, call: &MacroRun) -> Result<String, Diagnostic> {
        let text = self.inner.run_macro(library, call)?;
        let hidden = format!(" {} ", call.hidden);
        let written = call
            .line
            .replacen(&hidden, &format!(" {} ", call.written), 1);
        let key = (call.statement, written);
        self.seen.borrow_mut().insert(key, text.clone());
        Ok(text)
    }
}

/// `text` as a string literal, as the macro phase quotes a side.
pub(crate) fn quoted(text: &str) -> String {
    let escaped = text
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\t', "\\t");
    format!("\"{escaped}\"")
}
