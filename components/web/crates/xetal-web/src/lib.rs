//! The live demo: the xetal editor in the browser. The ASCII source on
//! the left, drawn decorated on the right as you type, the types (or,
//! after Run, the output) below, as in `xetal edit`; a drop-down of the
//! demos, Run and Zoom. The language itself is `xetal-play`.

mod app;
mod demos;
mod keys;
mod panes;

pub use app::App;
pub use demos::{DEMOS, Demo};
pub use keys::{Action, action};

/// Start the app in the page's body.
pub fn start() {
    console_error_panic_hook::set_once();
    let store = std::sync::Arc::new(xetal_store::Memory::default());
    xetal_store::install(store);
    yew::Renderer::<App>::new().render();
}
