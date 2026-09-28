//! Types of the scalar built-ins (T5): number literals and variables are
//! `Num`, conditions `Truthy`, comparisons return a `Truthy` value.

use xetal_base::{Diagnostic, Span};

use xetal_ty::Type;
use xetal_ty::Unifier;

use crate::generic::generic;

pub(crate) fn fun(a: Type, b: Type) -> Type {
    Type::Fn(Box::new(a), Box::new(b))
}

/// A fresh instance of a built-in's type.
pub fn prim_type(name: &str, u: &mut Unifier, span: Span) -> Result<Type, Diagnostic> {
    if let Some(t) = generic(name, u) {
        return Ok(t);
    }
    let binary = |a: &Type, r: Type| fun(a.clone(), fun(a.clone(), r));
    Ok(match name {
        "+" | "-" | "*" | "^" | "m_ax" | "m_in" => {
            let a = u.fresh_num();
            binary(&a, a.clone())
        }
        "/" => binary(&u.fresh_num(), Type::Float),
        "d_iv" | "m_od" => binary(&Type::Int, Type::Int),
        "=" | "!=" | "<" | ">" | "<=" | ">=" | "e_q~" => {
            let (a, t) = (u.fresh_num(), u.fresh_truthy());
            binary(&a, t)
        }
        "&" | "|" => {
            let (a, t) = (u.fresh_truthy(), u.fresh_truthy());
            binary(&a, t)
        }
        "n_ot" => fun(u.fresh_truthy(), u.fresh_truthy()),
        "n_eg" | "a_bs" => {
            let a = u.fresh_num();
            fun(a.clone(), a)
        }
        "f_loor" | "c_eiling" => fun(u.fresh_num(), Type::Int),
        "e_xp" | "l_og" | "f_loat" => fun(u.fresh_num(), Type::Float),
        other if LATER.contains(&other) => {
            return Err(Diagnostic::new("unsupported", format!("the built-in {other} arrives with a later saga (arrays, higher-order functions)")).with_span(span));
        }
        other => {
            return Err(Diagnostic::new(
                "unknown-builtin",
                format!("there is no built-in {other}"),
            )
            .with_span(span));
        }
    })
}

/// Built-ins that exist but are typed with the arrays and higher-order
/// sagas.
const LATER: &[&str] = &[
    "r_ev", "o_-", "r_/", "s_\\", "e_ach", "t_able", "i_nner", "c_ompose", "s_wap", "i_ndexOf",
    "m_ember?", "u_nique", "s_ort", "g_rade", "w_here", "r_oll!",
];
