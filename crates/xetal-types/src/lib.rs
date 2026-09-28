//! Types: representation, unification with an occurs check, the `Num`
//! constraint, and type schemes (Hindley-Milner foundations).

mod ty;
mod unify;

pub use ty::{Scheme, Type, TypeVar};
pub use unify::Unifier;

/// Pipeline stage name used in diagnostics and by the CLI.
pub const STAGE: &str = "type";
