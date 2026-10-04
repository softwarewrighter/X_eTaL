//! Macro libraries (MC10, MC11): an import finds a library and a macro
//! library together; a macro library is loaded on its own (its own
//! imports first, its `m:` exports in its own hidden namespace) and
//! kept, so a call can run it: the call is appended to it and the whole
//! is run by [`Libraries::run_macro`].

use std::collections::HashMap;
use std::rc::Rc;

use xetal_base::Diagnostic;
use xetal_expand::{Macros, NoMacros};
use xetal_lookup::{Found, Libraries, Pair};
use xetal_names::Import;
use xetal_sources::Sources;

use crate::MacroError;
use crate::expand::Loader;

/// A loaded macro library: the program its file makes (its libraries
/// first), its hidden namespace and its macros.
#[derive(Debug)]
pub struct MacroLib {
    pub sources: Sources,
    pub own: String,
    pub exports: Vec<String>,
}

/// Per alias letters: the macro library named.
pub(crate) type MacroAliases = HashMap<String, Rc<MacroLib>>;

/// The macros a file can call: its macro libraries, run by `libs`.
pub(crate) struct Table<'a> {
    pub macros: &'a MacroAliases,
    pub libs: &'a dyn Libraries,
}

impl Macros for Table<'_> {
    fn run(&self, ns: &str, name: &str, left: &str, right: &str) -> Result<String, Diagnostic> {
        let Some(lib) = self.macros.get(ns) else {
            return NoMacros.run(ns, name, left, right);
        };
        if !lib.exports.iter().any(|e| e == name) {
            let message = format!(
                "{ns}:{name} is not defined by that macro library; it defines {}",
                lib.exports.join(", ")
            );
            return Err(Diagnostic::new("not-exported", message));
        }
        let call = format!("{} {}:{name} {}", quoted(left), lib.own, quoted(right));
        let mut program = lib.sources.clone();
        let index = program.add(&format!("the call of {ns}:{name}"), &call);
        program.copy(index, 0..call.len());
        self.libs.run_macro(&program).map_err(|d| {
            let message = format!("the macro {ns}:{name} failed: {}", d.message);
            Diagnostic::new(&d.code, message)
        })
    }
}

impl Loader<'_> {
    /// The macro library `found`, loaded once.
    pub(crate) fn macro_library(&mut self, found: &Found) -> Result<Rc<MacroLib>, Box<MacroError>> {
        if let Some(lib) = self.macros.get(&found.key) {
            return Ok(lib.clone());
        }
        let mut inner = Loader {
            libs: self.libs,
            sources: Sources::default(),
            loaded: HashMap::new(),
            macros: HashMap::new(),
            chain: self.chain.clone(),
            library: true,
            expansion: None,
        };
        inner.load(found, false)?;
        let (own, exports) = inner.loaded.remove(&found.key).unwrap_or_default();
        let lib = Rc::new(MacroLib {
            sources: inner.sources,
            own,
            exports,
        });
        self.macros.insert(found.key.clone(), lib.clone());
        Ok(lib)
    }

    /// The library and macro library an import names, if either exists
    /// and neither is being loaded.
    pub(crate) fn resolve(&self, import: &Import, file: &Found) -> Result<Pair, Diagnostic> {
        let pair = self.libs.find_both(&import.spec, &file.name);
        if pair.0.is_none() && pair.1.is_none() {
            let place = match file.name.as_str() {
                "-e" => "in the current directory".to_string(),
                name => format!("beside {name}"),
            };
            let spec = &import.spec;
            let message = match spec.contains('/')
                || spec.ends_with(".xtl")
                || spec.ends_with(".xtlm")
            {
                true => format!("no library {spec:?} (looked {place})"),
                false => format!(
                    "no library {spec:?} (looked for {spec}.xtl and {spec}.xtlm {place}, in userlibs/, in XETAL_PATH and among the standard libraries)"
                ),
            };
            return Err(Diagnostic::new("library-not-found", message).with_span(import.span));
        }
        for lib in pair.0.iter().chain(&pair.1) {
            if let Some(at) = self.chain.iter().position(|(k, _)| *k == lib.key) {
                let names: Vec<&str> = self.chain[at..].iter().map(|(_, n)| n.as_str()).collect();
                let message = format!("import cycle: {} -> {}", names.join(" -> "), lib.name);
                return Err(Diagnostic::new("import-cycle", message).with_span(import.span));
            }
        }
        Ok(pair)
    }
}

/// `text` as a string literal.
fn quoted(text: &str) -> String {
    let escaped = text
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\t', "\\t");
    format!("\"{escaped}\"")
}
