//! Loading a program and its libraries into one combined text.

use std::collections::HashMap;

use xetal_base::Diagnostic;
use xetal_sources::Sources;

use crate::MacroError;
use crate::imports::{Import, imports};

/// A library found by [`Libraries::find`]: a key naming it uniquely
/// (its resolved path), the name it is reported by, and its text.
#[derive(Debug, Clone)]
pub struct Found {
    pub key: String,
    pub name: String,
    pub text: String,
}

/// Where libraries come from.
pub trait Libraries {
    /// The library `spec` (a name or a path) imported by file `from`.
    fn find(&self, spec: &str, from: &str) -> Option<Found>;
}

struct Loader<'l> {
    libs: &'l dyn Libraries,
    sources: Sources,
    loaded: HashMap<String, usize>,
    /// Names of the files being loaded, outermost first (for cycles).
    chain: Vec<(String, String)>,
}

/// The program `text` (reported as `name`) with its libraries, each
/// loaded once and placed before the files that import it.
pub fn expand(name: &str, text: &str, libs: &dyn Libraries) -> Result<Sources, Box<MacroError>> {
    let mut loader = Loader {
        libs,
        sources: Sources::default(),
        loaded: HashMap::new(),
        chain: Vec::new(),
    };
    let main = Found {
        key: format!("\u{0}{name}"),
        name: name.into(),
        text: text.into(),
    };
    loader.load(&main, true)?;
    Ok(loader.sources)
}

impl Loader<'_> {
    fn load(&mut self, file: &Found, main: bool) -> Result<(), Box<MacroError>> {
        let error = |diagnostic: Diagnostic| {
            Box::new(MacroError {
                diagnostic,
                file: file.name.clone(),
                text: file.text.clone(),
                main,
            })
        };
        let index = self.sources.add(&file.name, &file.text);
        self.loaded.insert(file.key.clone(), index);
        self.chain.push((file.key.clone(), file.name.clone()));
        let found = imports(&file.text).map_err(error)?;
        let mut aliases: HashMap<String, String> = HashMap::new();
        for import in &found {
            let lib = self.resolve(import, file).map_err(error)?;
            if let Some(other) = aliases.insert(import.alias.clone(), lib.key.clone()) {
                let d = if other == lib.key {
                    "library-reimported"
                } else {
                    "alias-reused"
                };
                return Err(error(twice(d, import)));
            }
            if aliases.iter().filter(|(_, k)| **k == lib.key).count() > 1 {
                return Err(error(twice("library-reimported", import)));
            }
            if !self.loaded.contains_key(&lib.key) {
                self.load(&lib, false)?;
            }
        }
        self.chain.pop();
        self.emit(index, &file.text, &found);
        Ok(())
    }

    /// The library an import names, if it exists and is not being loaded.
    fn resolve(&self, import: &Import, file: &Found) -> Result<Found, Diagnostic> {
        let span = import.span;
        let lib = self.libs.find(&import.spec, &file.name).ok_or_else(|| {
            Diagnostic::new("library-not-found", format!("no library {:?} (looked beside {}, in XETAL_PATH and among the standard libraries)", import.spec, file.name)).with_span(span)
        })?;
        if let Some(at) = self.chain.iter().position(|(k, _)| *k == lib.key) {
            let names: Vec<&str> = self.chain[at..].iter().map(|(_, n)| n.as_str()).collect();
            let message = format!("import cycle: {} -> {}", names.join(" -> "), lib.name);
            return Err(Diagnostic::new("import-cycle", message).with_span(span));
        }
        Ok(lib)
    }

    /// Append the file's text without its import statements.
    fn emit(&mut self, index: usize, text: &str, found: &[Import]) {
        let mut at = 0;
        for import in found {
            self.sources.copy(index, at..import.span.start);
            at = import.span.end;
        }
        self.sources.copy(index, at..text.len());
    }
}

fn twice(code: &str, import: &Import) -> Diagnostic {
    let message = match code {
        "alias-reused" => format!(
            "{} already names another library in this file",
            import.alias
        ),
        _ => format!(
            "{:?} is already imported in this file under another alias",
            import.spec
        ),
    };
    Diagnostic::new(code, message).with_span(import.span)
}
