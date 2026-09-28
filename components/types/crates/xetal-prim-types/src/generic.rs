//! Built-ins whose types are not numeric: identity and the tacks (B9),
//! `p_rint!`, and the structural built-ins (B4, B5; rank-erased, T7).

use xetal_ty::{Type, Unifier};

use crate::prims::fun;

/// A fresh instance of a type-generic built-in's type, if `name` is one.
pub(crate) fn generic(name: &str, u: &mut Unifier) -> Option<Type> {
    let (a, b) = (u.fresh(), u.fresh());
    Some(match name {
        "i_d" | "p_rint!" => fun(a.clone(), a),
        "l_eft" => fun(a.clone(), fun(b, a)),
        "r_ight" => fun(a, fun(b.clone(), b)),
        "s_hape" | "t_ally" => fun(a, Type::Int),
        "r_ange" | "o_ffsets" => fun(Type::Int, Type::Int),
        "f_irst" | "r_avel" => fun(a.clone(), a),
        "r_eshape" | "t_ake" | "d_rop" | "s_elect" => fun(Type::Int, fun(a.clone(), a)),
        "c_at" => fun(a.clone(), fun(a.clone(), a)),
        _ => return None,
    })
}
