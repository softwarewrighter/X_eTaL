//! Running a macro (MC10): the macro phase hands over the macro
//! library's program with the call appended; it is lowered, checked
//! (the call must give text) and run, and what the call prints is the
//! macro's text. The macro phase cannot depend on the evaluator, so
//! every loader here wraps its libraries in [`Running`].

use xetal_base::Diagnostic;
use xetal_macro::{Found, Libraries, Pair};
use xetal_sources::Sources;

use crate::load::located;

/// `libs`, able to run macros.
pub(crate) struct Running<'a>(pub(crate) &'a dyn Libraries);

impl Libraries for Running<'_> {
    fn find(&self, spec: &str, from: &str) -> Option<Found> {
        self.0.find(spec, from)
    }

    fn find_both(&self, spec: &str, from: &str) -> Pair {
        self.0.find_both(spec, from)
    }

    fn run_macro(&self, program: &Sources) -> Result<String, Diagnostic> {
        run(program)
    }
}

/// The text the last statement of `sources` gives.
fn run(sources: &Sources) -> Result<String, Diagnostic> {
    let at = |d| located(sources, d);
    let mut program = xetal_core::lower(sources.combined()).map_err(at)?;
    let types = xetal_types::check_program(&mut program).map_err(at)?;
    if let Some(other) = types.last().filter(|t| *t != "Char") {
        let message = format!("it gives {other}, not text (a macro is (String, String) -> String)");
        return Err(Diagnostic::new("macro-not-text", message));
    }
    let mut out = Vec::new();
    let (_, result) = xetal_eval::eval_program(&program, &mut out, Some(0));
    result.map_err(at)?;
    let text = String::from_utf8_lossy(&out);
    Ok(text.strip_suffix('\n').unwrap_or(&text).to_string())
}
