//! An interactive session over the pipeline, and a file run as a
//! notebook (each statement followed by its output).

mod notebook;
mod session;
mod stdio;

pub use notebook::{Cell, notebook};
pub use session::{Reply, Session};
pub use stdio::stdio;
