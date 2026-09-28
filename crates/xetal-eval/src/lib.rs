//! Strict evaluator over Core only (docs/design.md section 7): scalars,
//! closures, currying, guards, call-by-need `~` parameters, late-bound
//! definitions, mutable `!` variables, and scalar built-ins.

mod apply;
mod arith;
mod machine;
mod prim;
mod value;
mod warn;

use std::collections::HashMap;
use std::io::Write;

use xetal_base::{Diagnostic, Span};

pub use value::Value;

/// Pipeline stage name used in diagnostics and by the CLI.
pub const STAGE: &str = "eval";

/// Stack reserved for evaluation (virtual memory; pages are used only as
/// recursion deepens).
const STACK_BYTES: usize = 1 << 30;

/// Evaluate a program, writing each top-level expression's value (and
/// any `p_rint!` output) to `out`. Returns the warnings, and the first
/// error if evaluation stopped.
///
/// Evaluation runs on its own thread with a large stack, so deep (but
/// finite) recursion works; runaway recursion is a `stack-overflow`
/// error, never a crash.
pub fn eval_source(
    src: &str,
    out: &mut (dyn Write + Send),
) -> (Vec<Diagnostic>, Result<(), Diagnostic>) {
    let program = match xetal_core::lower(src) {
        Ok(p) => p,
        Err(e) => return (Vec::new(), Err(e)),
    };
    let warnings = warn::warnings(&program);
    let result = std::thread::scope(|scope| {
        let worker = std::thread::Builder::new()
            .stack_size(STACK_BYTES)
            .spawn_scoped(scope, || {
                let mut machine = machine::Machine {
                    globals: HashMap::new(),
                    out,
                    depth: 0,
                };
                machine.run(&program)
            });
        match worker {
            Ok(handle) => handle
                .join()
                .unwrap_or_else(|_| Err(Diagnostic::new("internal", "the evaluator failed"))),
            Err(e) => Err(Diagnostic::new(
                "internal",
                format!("cannot start the evaluator: {e}"),
            )),
        }
    });
    (warnings, result)
}

pub(crate) fn err(code: &str, span: Span, message: impl Into<String>) -> Diagnostic {
    Diagnostic::new(code, message).with_span(span)
}
