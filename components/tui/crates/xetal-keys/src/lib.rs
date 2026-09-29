//! The keymap (nano-like) as a table, and applying editing commands.

mod apply;
mod map;

pub use apply::apply;
pub use map::{Action, Command, KEYMAP, command};
