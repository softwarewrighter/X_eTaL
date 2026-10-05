//! A library file checked on its own.

use xetal_base::Diagnostic;
use xetal_lex::{TokenKind, lex};
use xetal_macro::{FsLibraries, Libraries, expand_library};

use crate::load::{Loaded, lowered};
use crate::run::Running;

/// Whether `text` is a library: it names the `l:` namespace, or
/// defines a macro (`m:n_ame< := ...`, a macro library, MC10), and is
/// not a program (a file starting with `#!` is one, S5, whatever it
/// names: its `l:` names are then MC8 row 9).
pub fn is_library(text: &str) -> bool {
    if text.starts_with("#!") {
        return false;
    }
    let is_l = |ns: &Option<String>| ns.as_deref() == Some("l");
    macros_of(text).is_some()
        || lex(&code(text)).is_ok_and(|tokens| {
            tokens.iter().any(|t| match &t.kind {
                TokenKind::Func(f) => is_l(&f.ns),
                TokenKind::Var(v) => is_l(&v.ns),
                _ => false,
            })
        })
}

/// The name a text with no file name is checked under, by what it
/// defines (the live demo's editor): `System.xtlm` for the system
/// macros (`s:` definitions), `main.xtlm` for a macro library (`m:`
/// definitions), else `main.xtl`. The loader takes a file's rules
/// from its name.
pub fn name_for(text: &str) -> &'static str {
    match macros_of(text) {
        Some(true) => "System.xtlm",
        Some(false) => "main.xtlm",
        None => "main.xtl",
    }
}

/// Whether `text` defines macros, and if so whether they are the
/// system's (`s:n_ame< :=`, or a `::` signature line) rather than a
/// macro library's (`m:n_ame< :=`).
fn macros_of(text: &str) -> Option<bool> {
    let tokens = lex(&code(text)).ok()?;
    tokens.iter().enumerate().find_map(|(i, t)| match &t.kind {
        TokenKind::Func(f) if f.is_macro() && matches!(f.ns.as_deref(), Some("m" | "s")) => {
            let assigned = matches!(tokens.get(i + 1).map(|t| &t.kind), Some(TokenKind::Assign));
            assigned.then(|| f.ns.as_deref() == Some("s"))
        }
        _ => None,
    })
}

/// `text` without its signature lines (`s:u_se< :: ...`, System.xtlm),
/// which do not lex; they are not what makes a file a library.
fn code(text: &str) -> String {
    text.split_inclusive('\n')
        .map(|l| if l.contains("::") { "\n" } else { l })
        .collect()
}

/// The library `text` (reported as `name`) loaded on its own, as it
/// is when imported, its own libraries found beside it.
pub fn load_library(name: &str, text: &str) -> Result<Loaded, Diagnostic> {
    load_library_with(name, text, &FsLibraries::from_env())
}

/// [`load_library`], its own libraries found by `libs`.
pub fn load_library_with(
    name: &str,
    text: &str,
    libs: &dyn Libraries,
) -> Result<Loaded, Diagnostic> {
    lowered(expand_library(name, text, &Running(libs)))
}
