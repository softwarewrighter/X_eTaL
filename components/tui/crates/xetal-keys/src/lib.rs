//! A key as any host sees it, the keymap (nano-like) as a table, and
//! applying editing commands.

mod apply;
mod map;

pub use apply::{PAGE, apply};
pub use map::{Action, Code, Command, KEYMAP, Key, command};
