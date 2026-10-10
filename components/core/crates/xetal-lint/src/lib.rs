//! Lints over Core, clippy style: a catalog of named lints in groups,
//! each with a default level and an explanation (`registry`), and the
//! passes that find them. The warnings `xetal run` prints (L7
//! shadowing, PN2 deprecated privates) are a subset; `xetal lint` (the
//! `xetal-linter` driver) runs them all and applies levels.

mod catalog;
mod naming;
mod registry;
mod unused;
mod walk;
mod warn;

pub use registry::{Group, Level, Lint, all, lint};
pub use warn::{lints, warnings};
