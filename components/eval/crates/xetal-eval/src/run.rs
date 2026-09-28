//! Running a program on a worker thread with a large stack.

use std::collections::HashMap;
use std::io::Write;

use xetal_arith::Rng;
use xetal_base::{Diagnostic, Span};
use xetal_core::Program;

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
    match xetal_core::lower(src) {
        Ok(program) => eval_program(&program, out, None),
        Err(e) => (Vec::new(), Err(e)),
    }
}

/// Evaluate an already lowered (and possibly type-elaborated) program;
/// see [`eval_source`]. `r_oll!` draws from `seed`, or from a fresh
/// unpredictable seed when there is none.
pub fn eval_program(
    program: &Program,
    out: &mut (dyn Write + Send),
    seed: Option<u64>,
) -> (Vec<Diagnostic>, Result<(), Diagnostic>) {
    let warnings = xetal_lint::warnings(program);
    let result = std::thread::scope(|scope| {
        let worker = std::thread::Builder::new()
            .stack_size(STACK_BYTES)
            .spawn_scoped(scope, || {
                let mut machine = crate::machine::Machine {
                    globals: HashMap::new(),
                    out,
                    depth: 0,
                    rng: Rng::seeded(seed.unwrap_or_else(Rng::fresh_seed)),
                };
                machine.run(program)
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
