//! Running a macro (MC10, MC12, MC18): the system macros (called
//! unprefixed) and those of macro libraries (`alias:n_ame<`) alike are
//! run by a [`Macros`] table; the text a macro gives replaces its call
//! (as statements where the call is a statement of its own,
//! parenthesized inside an expression) and is expanded in turn.

use std::cell::Cell;

use xetal_base::{Diagnostic, Span};
use xetal_contract::{Contract, shape, unused};
use xetal_mapped::Mapped;

use crate::calls::Call;
use crate::copied::copied;

/// One call, as a macro sees it.
#[derive(Debug, Clone)]
pub struct MacroCall<'a> {
    /// The alias it is called under (none for a system macro).
    pub ns: Option<&'a str>,
    /// Its name with the mark: `i_f<`, `n_ame<`.
    pub name: &'a str,
    /// The text on each side (none for `@`, MC22).
    pub left: Option<&'a str>,
    pub right: Option<&'a str>,
    /// The call stands as a statement of its own.
    pub statement: bool,
    /// Where the call was written (a byte offset in its file; inside
    /// another macro's expansion, that macro's call), and where it ends.
    pub at: usize,
    pub end: usize,
    /// The fresh names given so far in this file (`g1:`, `g2:`, ...): a
    /// macro asking for one (`[]G_ENSYM`, MC30) takes the next number.
    pub fresh: &'a Cell<usize>,
}

/// What runs the macros a file can call.
pub trait Macros {
    /// The text `call` gives. An error with a note `macro-place: left`
    /// or `right` is reported at that argument, else at the macro.
    fn run(&self, call: &MacroCall) -> Result<String, Diagnostic>;

    /// The contract of the macro of `call` (MC31): the names it binds
    /// for the call's text (`## binds:`, anaphora), the shape of call
    /// it takes (`## takes:`), whether it copies or parses its arguments
    /// (`## uses:`); none declared, the default.
    fn contract(&self, _call: &MacroCall) -> Contract {
        Contract::default()
    }
}

/// No macros at all: every call is unknown.
pub struct NoMacros;

impl Macros for NoMacros {
    fn run(&self, call: &MacroCall) -> Result<String, Diagnostic> {
        let written = match call.ns {
            Some(ns) => format!("{ns}:{}", call.name),
            None => call.name.to_string(),
        };
        Err(Diagnostic::new(
            "unknown-macro",
            format!("there is no macro {written}"),
        ))
    }
}

/// Parses the code string on a side of the call (`true`, the right).
pub(crate) type Parse<'a> = &'a dyn Fn(bool) -> Result<(), Diagnostic>;

/// The expansion of `call`, run by `macros`; its arguments are `left`
/// and `right` (mapped where they were written, empty for `@`).
pub(crate) fn user(
    call: &Call,
    (left, right): (&Mapped, &Mapped),
    macros: &dyn Macros,
    (whole, fresh, code): (Span, &Cell<usize>, Parse<'_>),
) -> Result<(Mapped, Vec<String>), Diagnostic> {
    let (ns, name) = match call.name.split_once(':') {
        Some((ns, name)) => (Some(ns), name),
        None => (None, call.name.as_str()),
    };
    let request = MacroCall {
        ns,
        name,
        left: call.texts.0.then_some(left.text()),
        right: call.texts.1.then_some(right.text()),
        statement: call.statement,
        at: whole.start,
        end: whole.end,
        fresh,
    };
    let contract = macros.contract(&request);
    let texts = (request.left.is_some(), request.right.is_some());
    let written = written(&request);
    if let Some(d) = shape(&contract, &written, texts) {
        return Err(placed(d, call));
    }
    // Code where the macro takes code is parsed before it runs (ST5).
    if contract.code.0 && call.codes.0 {
        code(false)?;
    }
    if contract.code.1 && call.codes.1 {
        code(true)?;
    }
    let text = macros.run(&request).map_err(|d| placed(d, call))?;
    let args = [request.left, request.right];
    if let Some(d) = unused(&contract, &written, &args, &text) {
        return Err(placed(d, call));
    }
    let text = match call.statement {
        true => text,
        false => format!("({text})"),
    };
    Ok((copied(&text, &[left, right], whole), contract.binds))
}

/// The macro as the call writes it: `x:n_ame<`, or `i_f<` for a system macro.
fn written(call: &MacroCall) -> String {
    match call.ns {
        Some(ns) => format!("{ns}:{}", call.name),
        None => call.name.to_string(),
    }
}

/// `d` at the place it names (its `macro-place` note), else at the macro.
fn placed(mut d: Diagnostic, call: &Call) -> Diagnostic {
    let place = d.notes.iter().position(|n| n.starts_with("macro-place: "));
    let span = match place.map(|i| d.notes.remove(i)).as_deref() {
        Some("macro-place: left") => call.left,
        Some("macro-place: right") => call.right,
        _ => call.token,
    };
    d.span = d.span.or(Some(span));
    d
}
