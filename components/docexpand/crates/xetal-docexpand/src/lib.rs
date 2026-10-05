//! `xetal doc`'s macro expansions: a file is expanded as the program
//! loader expands it (its macro libraries and the system macros run),
//! each call's text remembered; then every call written in the file is
//! shown with the text that replaced it (parenthesized inside an
//! expression), and the calls in that text in turn, to the depth limit.

mod expand;
mod json;
mod record;
mod tree;

pub use expand::expansions;
pub use tree::Expansion;
