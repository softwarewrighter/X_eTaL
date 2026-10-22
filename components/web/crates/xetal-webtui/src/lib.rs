//! `xetal edit` in the browser: the same editor as the terminal's
//! (`xetal-edit`: keys in, a ratatui frame out) drawn into ratatui's
//! in-memory backend, whose cells the page shows as styled spans; the
//! page's key events become the editor's keys (`xetal-keys`). The
//! editor saves through the browser's store and finds libraries there.
//! Ctrl-Q hands the text back to the host.

mod cells;
mod keys;
mod view;

pub use cells::{Run, rows};
pub use keys::key_of;
pub use view::{Props, Tui};
