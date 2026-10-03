//! The terminal's screen (Saga 25), after web-sw-tos's `ui.rs`: text
//! written as lines of styled character cells, wrapped at the width,
//! kept in a bounded scrollback, shown as the last rows that fit.

mod cell;
mod screen;

pub use cell::{Cell, Color, Style};
pub use screen::Screen;
