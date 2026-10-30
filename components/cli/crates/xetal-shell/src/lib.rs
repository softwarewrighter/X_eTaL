//! A shell for the browser's terminal page: at its prompt, `xetal` and
//! its subcommands run through the same dispatch as the binary
//! (`xetal-command`), `xetal repl` is the REPL on the same line editor
//! (`xetal-line`), and `xetal edit FILE` is the editor full screen
//! (`xetal-edit`), all keys in and a ratatui frame out, so a host draws
//! it into any backend. A few words of its own: `help`, `ls`, `cat`,
//! `clear`, `exit`. Files are the host's (a [`Catalog`]), with the text
//! edited and not saved kept in the shell's buffers. Also the editor's
//! screen after given keys, as text (`xetal edit --screen`), so the
//! terminal's screen and the page's can be compared.

mod ansi;
mod exec;
mod modes;
mod screen;
mod shell;
mod words;

pub use ansi::line;
pub use screen::{screen, text};
pub use shell::{Catalog, Flow, Shell};
pub use words::words;
