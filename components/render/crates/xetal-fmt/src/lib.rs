//! The canonical form (`xetal fmt`): raw ASCII, one statement per line,
//! every application in parentheses, reparsing to the same tree.

mod canonical;
mod names;
mod string;

pub use canonical::canonical;
