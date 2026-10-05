//! `xetal doc`'s search, in the browser with no build tooling: an
//! index (`search-index.js`, written by Rust) of every item of the
//! documented files and every built-in of the catalog, with its type
//! normalized Hoogle-like (no constraints, type variables renamed in
//! order), and a plain script (`search.js`) matching a query by name
//! or, when it reads as a type, by type.

mod index;
mod normalize;
mod write;

pub use index::{Entry, entries, script};
pub use normalize::normalize;
pub use write::document;
