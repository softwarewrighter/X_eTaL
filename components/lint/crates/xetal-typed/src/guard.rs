//! `int-condition`: a guard whose condition is inferred to be an Int.
//! Conditions take any `Truthy` (T5), so `n ? ...` type-checks, but an
//! Int condition must be 1 or 0 at run time (`error[not-a-bool]`): a
//! count where a truth belongs usually means a comparison was left out.

use xetal_base::Diagnostic;
use xetal_ir::Program;
use xetal_types::{Type, guard_types};

/// The typed lints' findings for `program`; none when it does not
/// type-check (the type error is reported where the program is run).
pub fn typed(program: &Program) -> Vec<Diagnostic> {
    let Ok(guards) = guard_types(program) else {
        return Vec::new();
    };
    let message = "this guard's condition is an Int, which fails at run time unless it is 1 or 0: \
                   write the comparison meant, as `0 < n` or `0 != n`";
    guards
        .into_iter()
        .filter(|(_, ty)| *ty == Type::Int)
        .map(|(span, _)| Diagnostic::warning("int-condition", message).with_span(span))
        .collect()
}
