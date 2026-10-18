//! The string lints, over the program's file as written (not Core): how
//! each string literal is spelled, plain `"..."` or raw `r#"..."#`
//! (ST4), and whether another spelling of the same value reads better.
//! Every fix keeps the literal's value byte for byte (D156): it is
//! offered only when the new spelling lexes to the same string. Kept
//! apart from `xetal-lint` (Core passes); see docs/lint.md.

mod directive;
mod form;
mod literal;
mod scan;
mod spelling;

pub use form::{Form, fewest_hashes};
pub use literal::{Literal, holds, literals};
pub use scan::{line_col, strings};
