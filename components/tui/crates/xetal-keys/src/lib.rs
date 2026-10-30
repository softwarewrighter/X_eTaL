//! A key as any host sees it, the keymap (nano-like) as a table,
//! applying editing commands, and keys written as text (`<C-r>`).

mod apply;
mod map;
mod names;

pub use apply::{PAGE, apply};
pub use map::{Action, Code, Command, KEYMAP, Key, command};
pub use names::keys;
