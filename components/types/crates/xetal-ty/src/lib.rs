//! Type foundations: types, unification with an occurs check, the
//! `Num` and `Truthy` classes, schemes and defaulting (T5).

mod defaulting;
mod scheme;
mod ty;
mod unify;

pub use scheme::mono;
pub use ty::{Scheme, Type, TypeVar};
pub use unify::Unifier;
