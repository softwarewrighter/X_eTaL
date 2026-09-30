//! The live demo's engine: what the browser app asks of the language.
//! A program's libraries come from the store the host installed (the
//! browser's local storage), then the standard libraries built in, and
//! its files (`[]N_GET`, `[]N_PUT`) are in that store too. Everything
//! here also runs natively, so it is tested without a browser.

mod engine;

pub use engine::{Run, check, run};
pub use xetal_program::is_library;
pub use xetal_view::{Class, Segment, view as decorate};
