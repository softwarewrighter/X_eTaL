//! The built-in dictionary: which names exist (B1-B7 and the symbols),
//! which are implemented for scalars, and their arity.

use std::io::Write;

use xetal_base::{Diagnostic, Span};

use crate::arith::{binary, compare, num};
use crate::err;
use crate::value::Value;

/// Built-ins implemented for scalars, with their arity.
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
];

/// Built-ins that exist but arrive with arrays or higher-order functions.
pub const LATER: &[&str] = &[
    "r_ev", "o_-", "r_/", "s_\\", "s_hape", "r_eshape", "r_ange", "o_ffsets", "t_ally", "f_irst",
    "t_ake", "d_rop", "s_elect", "r_avel", "c_at", "e_ach", "t_able", "i_nner", "c_ompose",
    "s_wap", "i_ndexOf", "m_ember?", "u_nique", "s_ort", "g_rade", "w_here", "r_oll!",
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
    match (name, args) {
        ("p_rint!", [v]) => {
            writeln!(out, "{v}").map_err(|e| err("io", span, e.to_string()))?;
            Ok(v.clone())
        }
        ("&" | "|", [a, b]) => {
            let (a, b) = (truth(a, span)?, truth(b, span)?);
            Ok(Value::Bool(if name == "&" { a && b } else { a || b }))
        }
        ("n_ot", [a]) => Ok(Value::Bool(!truth(a, span)?)),
        ("=" | "!=" | "<" | ">" | "<=" | ">=" | "e_q~", [a, b]) => {
            Ok(compare(name, num(a, span)?, num(b, span)?))
        }
        (_, [a, b]) => binary(name, num(a, span)?, num(b, span)?, span),
        (_, [a]) => unary(name, a, span),
        _ => Err(err("unknown-builtin", span, format!("bad call of {name}"))),
    }
}

fn unary<'a>(name: &str, a: &Value<'a>, span: Span) -> Result<Value<'a>, Diagnostic> {
    use crate::arith::Num::{F, I};
    let overflow = || err("integer-overflow", span, "integer overflow");
    let whole = |x: f64| {
        if x.is_finite() && x.abs() < 9.0e18 {
            Ok(Value::Int(x as i64))
        } else {
            Err(overflow())
        }
    };
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

/// A Bool: true / false, or the Ints 1 / 0; anything else is an error (T1).
pub fn truth(v: &Value, span: Span) -> Result<bool, Diagnostic> {
    match v {
        Value::Bool(b) => Ok(*b),
        Value::Int(1) => Ok(true),
        Value::Int(0) => Ok(false),
        other => Err(err(
            "not-a-bool",
            span,
            format!("expected a Bool (or 1 / 0), got {other}"),
        )),
    }
}
