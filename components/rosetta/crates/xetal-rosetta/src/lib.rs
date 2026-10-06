//! The Rosetta stone's page (docs/rosetta.md): the X_eTaL program
//! demos/rosetta/rosetta.xtl run by the live demo's worker, its frames
//! shown as they come, the page's pointer, keys and controls sent to it
//! as events (RS1), and what the program says about where it stands
//! (`at IDIOM TOP BOTTOM`) read back for the controls and the address
//! bar. The page knows no geometry, traversal or language: those are
//! the program's.

mod app;
mod events;
mod follow;
mod program;
mod zoom;

pub use app::App;

/// Start the page in the body.
pub fn start() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
