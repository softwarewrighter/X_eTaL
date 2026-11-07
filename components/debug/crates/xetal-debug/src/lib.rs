//! The debugger's core (Saga 43, docs/debugger-plan.md): a session over
//! the steppable evaluator (D50) that stops at statements and, when
//! stepping in, at each application; keeps breakpoints by file and
//! line; steps over, in and out, continues; and gives the call stack.
//! Every front end is a client of it: the Debug Adapter Protocol
//! (`xetal dap`), the terminal debugger and the live demo's page. It
//! drives the machine one transition at a time, so a run without a
//! debugger is not slowed by it.

mod here;
mod session;
mod stack;
mod steps;
mod walk;

pub use here::{Kind, Place};
pub use session::{Debugger, Reason, Stop};
pub use stack::Frame;
