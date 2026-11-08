//! The debugger's core (Saga 43, docs/debugger-plan.md): a session over
//! the steppable evaluator (D50) that stops at statements and, when
//! stepping in, at each application; keeps breakpoints by file and
//! line (with conditions and hit counts); steps over, in and out,
//! continues; gives the call stack, each scope's variables a page at
//! a time, and watches evaluated where it stopped.
//! Every front end is a client of it: the Debug Adapter Protocol
//! (`xetal dap`), the terminal debugger and the live demo's page. It
//! drives the machine one transition at a time, so a run without a
//! debugger is not slowed by it.

mod here;
mod session;
mod stack;
mod steps;
mod walk;
mod watch;

pub use here::{Kind, Place};
pub use session::{Breakpoint, Debugger, Reason, Stop};
pub use stack::{Frame, Scope, Var};
