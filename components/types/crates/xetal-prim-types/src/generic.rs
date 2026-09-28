//! Built-ins whose types are not numeric: identity and the tacks (B9)
//! and `p_rint!`.

use xetal_ty::{Type, Unifier};

use crate::prims::fun;

/// A fresh instance of a type-generic built-in's type, if `name` is one.
pub(crate) fn generic(name: &str, u: &mut Unifier) -> Option<Type> {
    let (a, b) = (u.fresh(), u.fresh());
    Some(match name {
        "i_d" | "p_rint!" => fun(a.clone(), a),
        "l_eft" => fun(a.clone(), fun(b, a)),
        "r_ight" => fun(a, fun(b.clone(), b)),
        _ => return None,
    })
}
