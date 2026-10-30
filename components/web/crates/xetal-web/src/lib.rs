//! The live demo: the xetal editor in the browser. The ASCII source on
//! the left, drawn decorated on the right as you type, the types (or,
//! after Run, the output) below, as in `xetal edit`; Open (the demos,
//! the standard libraries and your files), Save, Run and Zoom. Files
//! are the browser's local storage; a program reading a line
//! (`[]R_EAD`) waits for it in the output pane, a terminal. The
//! language itself is `xetal-play`.

mod app;
mod demos;
mod files;
mod keys;
mod panes;
mod storage;

pub use app::App;
pub use demos::{DEMOS, Demo, choices, open, seed};
pub use keys::{Action, action};

/// Start the app in the page's body.
pub fn start() {
    console_error_panic_hook::set_once();
    xetal_store::install(std::sync::Arc::new(storage::Local));
    // The terminal page runs programs on the page itself: the browser's
    // clock, as the worker installs for its runs.
    xetal_clock::install(std::sync::Arc::new(xetal_webclock::Browser));
    // No disk here: a program's libraries are found in the store.
    xetal_macro::libraries_in_store(true);
    seed();
    yew::Renderer::<App>::new().render();
}
