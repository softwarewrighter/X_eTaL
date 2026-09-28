//! An interactive session over the pipeline.

mod session;
mod stdio;

pub use session::{Reply, Session};
pub use stdio::stdio;
