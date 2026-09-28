//! The built-in dictionary: which names exist (B1-B7 and the symbols),
//! which are implemented for scalars, and their arity.

use std::io::Write;

use xetal_base::{Diagnostic, Span};

use crate::err;
use xetal_arith::{binary, compare, lift1, lift2, num, truth};
use xetal_value::Value;

/// Implemented built-ins with their arity: the scalar functions,
/// `p_rint!`, identity and the tacks (B9), and the structural
/// built-ins (B4, B5).
const SCALAR: &[(&str, usize)] = &[
    ("+", 2),
    ("-", 2),
    ("*", 2),
    ("/", 2),
    ("^", 2),
    ("=", 2),
    ("!=", 2),
    ("<", 2),
    (">", 2),
    ("<=", 2),
    (">=", 2),
    ("&", 2),
    ("|", 2),
    ("m_ax", 2),
    ("m_in", 2),
    ("d_iv", 2),
    ("m_od", 2),
    ("e_q~", 2),
    ("n_eg", 1),
    ("a_bs", 1),
    ("f_loor", 1),
    ("c_eiling", 1),
    ("n_ot", 1),
    ("e_xp", 1),
    ("l_og", 1),
    ("f_loat", 1),
    ("p_rint!", 1),
    ("i_d", 1),
    ("l_eft", 2),
    ("r_ight", 2),
    ("s_hape", 1),
    ("t_ally", 1),
    ("r_avel", 1),
    ("f_irst", 1),
    ("r_ange", 1),
    ("o_ffsets", 1),
    ("r_eshape", 2),
    ("t_ake", 2),
    ("d_rop", 2),
    ("s_elect", 2),
    ("c_at", 2),
];

/// Built-ins that exist but arrive with arrays or higher-order functions.
pub const LATER: &[&str] = &[
    "r_ev", "o_-", "r_/", "s_\\", "e_ach", "t_able", "i_nner", "c_ompose", "s_wap", "i_ndexOf",
    "m_ember?", "u_nique", "s_ort", "g_rade", "w_here", "r_oll!",
];

/// Every built-in name, for the shadowing warning (L7).
pub fn is_builtin(name: &str) -> bool {
    SCALAR.iter().any(|(n, _)| *n == name) || LATER.contains(&name)
}

/// The arity of an implemented built-in, or an error.
pub fn arity(name: &str, span: Span) -> Result<(&'static str, usize), Diagnostic> {
    if let Some((n, a)) = SCALAR.iter().find(|(n, _)| *n == name) {
        return Ok((n, *a));
    }
    if LATER.contains(&name) {
        return Err(err(
            "unsupported",
            span,
            format!(
                "the built-in {name} arrives with a later saga (arrays, higher-order functions)"
            ),
        ));
    }
    Err(err(
        "unknown-builtin",
        span,
        format!("there is no built-in {name}"),
    ))
}

/// Call a fully applied built-in.
pub fn call<'a>(
    name: &str,
    args: &[Value<'a>],
    span: Span,
    out: &mut dyn Write,
) -> Result<Value<'a>, Diagnostic> {
    if let Some(result) = xetal_struct::call(name, args, span) {
        return result;
    }
    match (name, args) {
        ("p_rint!", [v]) => {
            writeln!(out, "{v}").map_err(|e| err("io", span, e.to_string()))?;
            Ok(v.clone())
        }
        ("i_d", [a]) | ("l_eft", [a, _]) | ("r_ight", [_, a]) => Ok(a.clone()),
        (_, [a, b]) => lift2(a, b, span, |x, y| scalar2(name, x, y, span)),
        (_, [a]) => lift1(a, |x| unary(name, x, span)),
        _ => Err(err("unknown-builtin", span, format!("bad call of {name}"))),
    }
}

/// A dyadic scalar built-in on two scalars.
fn scalar2<'a>(
    name: &str,
    a: &Value<'a>,
    b: &Value<'a>,
    span: Span,
) -> Result<Value<'a>, Diagnostic> {
    match name {
        "&" | "|" => {
            let (a, b) = (truth(a, span)?, truth(b, span)?);
            Ok(Value::Bool(if name == "&" { a && b } else { a || b }))
        }
        "=" | "!=" | "<" | ">" | "<=" | ">=" | "e_q~" => {
            Ok(compare(name, num(a, span)?, num(b, span)?))
        }
        _ => binary(name, num(a, span)?, num(b, span)?, span),
    }
}

fn unary<'a>(name: &str, a: &Value<'a>, span: Span) -> Result<Value<'a>, Diagnostic> {
    use xetal_arith::Num::{F, I};
    let overflow = || err("integer-overflow", span, "integer overflow");
    let whole = |x: f64| {
        if x.is_finite() && x.abs() < 9.0e18 {
            Ok(Value::Int(x as i64))
        } else {
            Err(overflow())
        }
    };
    if name == "n_ot" {
        return Ok(Value::Bool(!truth(a, span)?));
    }
    match (name, num(a, span)?) {
        ("n_eg", I(i)) => i.checked_neg().map(Value::Int).ok_or_else(overflow),
        ("n_eg", F(x)) => Ok(Value::Float(-x)),
        ("a_bs", I(i)) => i.checked_abs().map(Value::Int).ok_or_else(overflow),
        ("a_bs", F(x)) => Ok(Value::Float(x.abs())),
        ("f_loor", I(i)) | ("c_eiling", I(i)) => Ok(Value::Int(i)),
        ("f_loor", F(x)) => whole(x.floor()),
        ("c_eiling", F(x)) => whole(x.ceil()),
        ("e_xp", n) => Ok(Value::Float(n.f().exp())),
        ("f_loat", n) => Ok(Value::Float(n.f())),
        ("l_og", n) if n.f() <= 0.0 => Err(err("domain", span, "l_og needs a positive number")),
        ("l_og", n) => Ok(Value::Float(n.f().ln())),
        _ => Err(err("unknown-builtin", span, format!("bad call of {name}"))),
    }
}
