//! `missing-example`: a macro whose doc block shows no call.

use xetal_base::Diagnostic;
use xetal_ir::Program;

use crate::macros::Def;

/// The finding for `def` when it is a macro with no `## >>` example.
pub(crate) fn example(program: &Program, def: &Def) -> Option<Diagnostic> {
    let name = def.macro_name?;
    if def.doc.as_ref().is_some_and(|d| !d.examples.is_empty()) {
        return None;
    }
    let message = format!(
        "the macro {name} has no `## >>` example: show a call and what it gives in its doc block, which `xetal doc --test` runs"
    );
    let at = program.name_span(def.value.id).unwrap_or(def.value.span);
    Some(Diagnostic::warning("missing-example", message).with_span(at))
}
