//! Runs run again. A REPL session runs its accepted source again for
//! each new line, and a debugger goes back in a run by running it again
//! from a checkpoint (Saga 43); either way a picture is shown once, a
//! line is read once and a file is written once. Process-wide, as the
//! store is (xetal-store re-exports this crate).

mod pictures;
mod travel;

pub use pictures::{admit, forget_lines, line, muted, replay, shown};
pub use travel::{Mark, admit_write, mark, rewind, written};
