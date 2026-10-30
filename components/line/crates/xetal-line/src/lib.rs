//! The REPL's line editor: you type ASCII and see the line decorated
//! and highlighted as you type (the view model), with the editor's keys
//! (`xetal-keys`), a history, and Tab completion (`xetal-complete`). [`Line`] is a state machine tested
//! without a terminal (the browser's terminal drives it too); with the
//! `terminal` feature, `read_line` drives it on a terminal.

mod line;
mod tab;
#[cfg(feature = "terminal")]
mod term;
#[cfg(feature = "terminal")]
mod tty;

pub use line::{Line, Outcome};
#[cfg(feature = "terminal")]
pub use term::read_line;
#[cfg(feature = "terminal")]
pub use tty::Terminal;
