//! xetal in the browser's terminal: a shell (`xetal-shell`) where
//! xetal's commands run as at a terminal, `xetal repl` is the REPL and
//! `xetal edit FILE` the same editor as the terminal's, all keys in and
//! a ratatui frame out, drawn into ratatui's in-memory backend, whose
//! cells the page shows as styled spans; the page's key events become
//! the shell's keys (`xetal-keys`). Files are the browser's store, the
//! demos and the standard libraries; `exit` hands the file back to the
//! host. A run rolls from a seed the page draws, since the page has no
//! clock to seed from.

mod cells;
mod keys;
mod view;

pub use cells::{Run, rows};
pub use keys::key_of;
pub use view::{Files, Props, Tui};
