//! Printers: raw ASCII <-> decorated Unicode (lossless), and raw ASCII
//! -> LaTeX math for post-processing. Canonical and expanded printers
//! join later.

mod glyphs;
mod inverse;
mod latex;
mod unicode;

pub use inverse::undecorate;
pub use latex::latex;
pub use unicode::decorate;

/// Pipeline stage name used in diagnostics and by the CLI.
pub const STAGE: &str = "render";
