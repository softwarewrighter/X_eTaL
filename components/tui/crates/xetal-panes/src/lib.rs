//! Terminal widgets over the view model: the ASCII source as typed and
//! the decorated rendering side by side, both highlighted by token
//! class, with the cursor mapped into each and shared scrolling.

mod lines;
mod panes;
mod theme;

pub use panes::Panes;
pub use theme::style;
