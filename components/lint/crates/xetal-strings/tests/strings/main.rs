//! String-lint tests (one test binary so helpers are shared): each lint
//! firing and not, its fix, and that every fix keeps the value (D156).

mod common;
mod directive;
mod escapes;
mod hashes;
mod place;
mod raw;
