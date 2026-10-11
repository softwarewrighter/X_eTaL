//! The effect lints over Core (lang-choices section 16: a `!` ends the
//! name of a function with a side effect): `missing-bang`, a function
//! calling an effect directly without `!` in its name, and
//! `needless-bang`, a `!` function that could not have an effect. Their
//! catalog entries are in xetal-lint, which runs them.

mod bang;
mod effect;

pub use bang::bangs;
