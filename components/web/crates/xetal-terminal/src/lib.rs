//! The terminal pane (X_eTaL-games' ask G23): a program run in a
//! worker, its output on a cell grid (so `[]A_T`, `[]C_LS` and colors
//! work), typed lines for `[]R_EAD`, single keys for `[]K_EY`, keys,
//! ticks and clicks as `[]E_VENT` lines, `[]S_HOW` pictures in place
//! with clicks in the picture's own pixels, and the transcript as text
//! for tests. The live demo's output pane is drawn by `body`; a page of
//! its own takes the `Terminal` component. Its stylesheet is
//! `terminal.css`, scoped to `.xetal-terminal`.

mod pane;
mod transcript;
mod view;

pub use pane::{Terminal, TerminalProps};
pub use transcript::{screen_text, transcript};
pub use view::{body, picture};
