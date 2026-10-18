//! The shape of a call against what the macro takes, before it runs.

use xetal_base::Diagnostic;

use crate::Contract;

/// `bad-macro-call` when `texts` (which sides of the call are text) is
/// not the shape `contract` declares, placed at the first side that
/// differs; none declared, nothing.
pub fn shape(contract: &Contract, written: &str, texts: (bool, bool)) -> Option<Diagnostic> {
    let takes = contract.takes.filter(|takes| *takes != texts)?;
    let kind = |text: bool, code: bool| match (text, code) {
        (true, true) => "code",
        (true, false) => "text",
        _ => "@",
    };
    let message = format!(
        "{written} takes {} on its left and {} on its right",
        kind(takes.0, contract.code.0),
        kind(takes.1, contract.code.1)
    );
    let place = if takes.0 != texts.0 { "left" } else { "right" };
    Some(Diagnostic::new("bad-macro-call", message).with_note(format!("macro-place: {place}")))
}
