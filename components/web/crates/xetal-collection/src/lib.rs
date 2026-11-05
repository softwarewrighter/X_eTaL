//! The X_eTaL-libraries collection in the live demo: its libraries and
//! their demos, vendored at a pinned commit (vendor/libraries, kept by
//! scripts/sync-libraries.sh) and embedded, so `"e:" u_se< "Equations"`
//! works in the page as it does at a terminal with the collection on
//! XETAL_PATH. A program's libraries are found in your files first,
//! then among the standard libraries, then here ([`install`]).

include!(concat!(env!("OUT_DIR"), "/collection.rs"));

/// The library file `file` (`Equations.xtl`, `Equations.xtlm`).
pub fn library(file: &str) -> Option<&'static str> {
    LIBRARIES.iter().find(|(n, _)| *n == file).map(|(_, t)| *t)
}

/// The demo `path` (`Equations/integrals.xtl`).
pub fn demo(path: &str) -> Option<&'static str> {
    DEMOS.iter().find(|(n, _)| *n == path).map(|(_, t)| *t)
}

/// Make the collection the last place libraries are found, in this
/// thread's program (the page, and the worker that runs programs).
pub fn install() {
    xetal_macro::add_collection(library);
}
