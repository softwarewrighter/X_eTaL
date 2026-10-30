//! An interactive session over the pipeline, and a file run as a
//! notebook (each statement followed by its output).

mod live;
mod notebook;
mod session;
#[cfg(feature = "terminal")]
mod stdio;

pub use notebook::{Cell, continued, notebook};
pub use session::{Reply, Session};
#[cfg(feature = "terminal")]
pub use stdio::stdio;
