//! Printers: raw ASCII <-> decorated Unicode (lossless), raw ASCII ->
//! LaTeX math for post-processing, and the canonical form (`xetal fmt`).

mod canonical;
mod glyphs;
mod inverse;
mod latex;
mod unicode;

pub use canonical::canonical;
pub use inverse::undecorate;
pub use latex::latex;
pub use unicode::decorate;

/// Pipeline stage name used in diagnostics and by the CLI.
pub const STAGE: &str = "render";
