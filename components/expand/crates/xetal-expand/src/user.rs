//! Macros of macro libraries (MC10, MC12): `"left" x:n_ame< "right"`
//! runs the macro library's function on the two texts; the text it
//! gives replaces the call (as statements where the call is a statement
//! of its own, parenthesized inside an expression) and is expanded in
//! turn.

use xetal_base::{Diagnostic, Span};
use xetal_mapped::Mapped;

use crate::calls::Call;

/// What runs the macros of the macro libraries a file imports.
pub trait Macros {
    /// The text macro `name` (`n_ame<`) of the macro library imported
    /// as `ns` gives for the arguments `left` and `right`.
    fn run(&self, ns: &str, name: &str, left: &str, right: &str) -> Result<String, Diagnostic>;
}

/// No macro libraries: every `alias:n_ame<` call is unknown.
pub struct NoMacros;

impl Macros for NoMacros {
    fn run(&self, ns: &str, name: &str, _left: &str, _right: &str) -> Result<String, Diagnostic> {
        let message = format!(
            "there is no macro {ns}:{name}: no macro library is imported as {ns}: (\"{ns}:\" u_se< \"Name\" finds Name.xtlm)"
        );
        Err(Diagnostic::new("unknown-macro", message))
    }
}

/// The expansion of a user macro's `call`, given the text it ran to;
/// all of it maps to the call `whole`. An error with no place of its
/// own is placed at the macro's name.
pub(crate) fn user(
    call: &Call,
    text: Result<String, Diagnostic>,
    whole: Span,
) -> Result<Mapped, Diagnostic> {
    let text = text.map_err(|mut d| {
        d.span = d.span.or(Some(call.token));
        d
    })?;
    let mut out = Mapped::default();
    let range = whole.start..whole.end;
    match call.statement {
        true => out.glue(&text, range),
        false => out.glue(&format!("({text})"), range),
    }
    Ok(out)
}
