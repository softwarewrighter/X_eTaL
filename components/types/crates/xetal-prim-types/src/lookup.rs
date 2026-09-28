//! A built-in's type by name.

use xetal_base::{Diagnostic, Span};
use xetal_catalog::find;
use xetal_ty::{Type, Unifier};

use crate::sig::read;

/// A fresh instance of the built-in `name`'s type.
pub fn prim_type(name: &str, u: &mut Unifier, span: Span) -> Result<Type, Diagnostic> {
    let err = |code, message: String| Err(Diagnostic::new(code, message).with_span(span));
    match find(name) {
        Some(b) if b.implemented => read(b.sig, u).map_err(|d| d.with_span(span)),
        Some(b) => err(
            "unsupported",
            format!("the built-in {name} arrives with a later saga ({})", b.rule),
        ),
        None => err("unknown-builtin", format!("there is no built-in {name}")),
    }
}
