//! The complexity lints over Core: code that does something simple in
//! a complex way. `redundant-lambda`, a lambda that only passes its
//! arguments on to a named function (`'{ x -> n_eg x }` is `'n_eg`) or
//! returns its argument (`'{ x -> x }` is `'i_d`). Its catalog entry is
//! in xetal-lint, which runs it.

mod chain;
mod redundant;

pub use redundant::redundant;
