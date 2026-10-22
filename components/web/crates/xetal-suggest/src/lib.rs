//! The completion list in the live demo's ASCII pane: after a pause in
//! typing, the candidates for the word at the cursor (`xetal-complete`)
//! in a list under the text; Tab or Enter writes the chosen one over
//! the word, Up and Down choose, Escape closes, and a click on a row
//! takes it. The hook gives the pane the list, the key handler and what
//! to call after each input.

mod hook;
mod list;

pub use hook::{Suggest, use_suggest};
