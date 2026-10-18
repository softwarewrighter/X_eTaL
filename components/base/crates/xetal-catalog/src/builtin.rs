//! One catalog entry, and lookup by name.

use crate::BUILTINS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Builtin {
    pub name: &'static str,
    /// How many arguments it takes (0 for one that arrives later).
    pub arity: usize,
    /// Its type, e.g. `Num a => a -> a -> a` (empty when later).
    pub sig: &'static str,
    /// The lang-choices rules that define it.
    pub rule: &'static str,
    /// How its result's shape follows from its arguments', for the
    /// shape lint: `scalar` (item by item, a scalar extends), `shape`
    /// (`s_hape`: a vector of the rank, whose value is the shape),
    /// `count` (a single number), `reshape` (the left's values), `cat`
    /// (`c_at`: a vector's values joined), `check` (`r_equireShape`: the
    /// left's values, which the right must have), `inner` (A's leading
    /// axes then B's trailing ones, A's last equal to B's first),
    /// `reduce` (the right's shape less its leading axis); empty when
    /// unknown.
    pub shape: &'static str,
    pub implemented: bool,
}

/// The built-in spelled `name`.
pub fn find(name: &str) -> Option<&'static Builtin> {
    BUILTINS.iter().find(|b| b.name == name)
}
