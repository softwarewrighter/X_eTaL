//! Printers: raw ASCII <-> decorated Unicode (lossless), raw ASCII ->
//! LaTeX math for post-processing, and the canonical form (`xetal fmt`).

mod code;
mod comments;
mod inverse;
mod lambda;
mod latex;
mod unicode;

pub use code::{CodePiece, code_pieces};
pub use comments::{CommentKind, comment_kind, comment_marker};
pub use inverse::undecorate;
pub use latex::latex;
pub use unicode::{decorate, gap_text, token_text};
pub use xetal_fmt::canonical;
pub use xetal_glyphs::superscript_word;
