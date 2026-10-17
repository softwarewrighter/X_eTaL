//! The lint over a program: each item in order, top-level bindings
//! seen by the items after them; a definition's body on its own, since
//! definitions are late-bound.

use xetal_base::Diagnostic;
use xetal_ir::{Item, Program};

use crate::env::Env;
use crate::infer::infer;

/// The shape lint's findings for `program`: `shape-mismatch` and
/// `shape-check-proven`. Levels are not applied here (the driver does).
pub fn shapes(program: &Program) -> Vec<Diagnostic> {
    let (mut env, mut out) = (Env::default(), Vec::new());
    for item in &program.items {
        match item {
            Item::Def { value, .. } => {
                infer(value, &mut Env::default(), &mut out);
            }
            Item::Let { name, value, .. } => {
                let fact = infer(value, &mut env, &mut out);
                env.bind(name, fact);
            }
            Item::Set { value, .. } | Item::Eval(value) => {
                infer(value, &mut env, &mut out);
            }
        }
    }
    out
}
