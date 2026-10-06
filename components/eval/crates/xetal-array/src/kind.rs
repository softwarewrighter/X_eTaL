//! What kind of items an array holds, remembered when it is empty (T9,
//! as APL2's prototype): `""` is an empty character vector and
//! `0 r_eshape 1` an empty number vector, and each prints by its kind.

use crate::Array;

/// The kind of an array's items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Kind {
    /// Numbers (Int, Float, Bool): the kind an array has unless told.
    #[default]
    Number,
    /// Characters.
    Char,
    /// Boxes (enclosed items).
    Box,
}

impl<T> Array<T> {
    /// The same array, its items of `kind` (what it says when empty).
    pub fn with_kind(mut self, kind: Kind) -> Self {
        self.kind = kind;
        self
    }

    /// The kind of the items (T9).
    pub fn kind(&self) -> Kind {
        self.kind
    }
}

/// Two arrays are equal when their shapes and items are; empty ones
/// must also be of one kind (`""` is not `0 r_eshape 1`).
impl<T: PartialEq> PartialEq for Array<T> {
    fn eq(&self, other: &Self) -> bool {
        self.shape == other.shape
            && self.data == other.data
            && (!self.data.is_empty() || self.kind == other.kind)
    }
}
