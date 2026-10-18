//! The shape lint (type-check-macros approach 8): shapes carried
//! through Core symbolically, from literals, `r_eshape` and the
//! `r_equireShape` checks any macro may emit, by the shape rules the catalog
//! lists beside each built-in's type. It speaks only where it proves
//! something: `shape-mismatch` where shapes surely differ (the program
//! fails there at run time), `shape-check-proven` where a check surely
//! passes. Where data decides, it says nothing and the run-time check
//! stays the backstop.

mod env;
mod ext;
mod infer;
mod rules;
mod walk;

pub use walk::shapes;
