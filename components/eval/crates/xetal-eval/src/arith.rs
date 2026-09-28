//! Scalar arithmetic with the numeric rules: Bool -> Int in arithmetic
//! (T1), `/` always Float (T2), exact `=` and tolerant `e_q~` (T3),
//! power (D-4, D-10), overflow and division-by-zero errors.

use xetal_base::{Diagnostic, Span};

use crate::err;
use crate::value::Value;

#[derive(Clone, Copy)]
pub enum Num {
    I(i64),
    F(f64),
}

impl Num {
    pub fn f(self) -> f64 {
        match self {
            Num::I(i) => i as f64,
            Num::F(x) => x,
        }
    }
}

/// A number, with Bool as 1 / 0 (T1).
pub fn num(v: &Value, span: Span) -> Result<Num, Diagnostic> {
    match v {
        Value::Int(i) => Ok(Num::I(*i)),
        Value::Float(x) => Ok(Num::F(*x)),
        Value::Bool(b) => Ok(Num::I(i64::from(*b))),
        other => Err(err(
            "not-a-number",
            span,
            format!("expected a number, got {other}"),
        )),
    }
}

/// `+ - * / ^` and the integer operations, by symbol or name.
pub fn binary<'a>(op: &str, a: Num, b: Num, span: Span) -> Result<Value<'a>, Diagnostic> {
    let overflow = || {
        err(
            "integer-overflow",
            span,
            "integer overflow (use a Float, e.g. 2.0)",
        )
    };
    let zero = || err("division-by-zero", span, "division by zero");
    Ok(match (op, a, b) {
        ("+", Num::I(x), Num::I(y)) => Value::Int(x.checked_add(y).ok_or_else(overflow)?),
        ("-", Num::I(x), Num::I(y)) => Value::Int(x.checked_sub(y).ok_or_else(overflow)?),
        ("*", Num::I(x), Num::I(y)) => Value::Int(x.checked_mul(y).ok_or_else(overflow)?),
        ("+", a, b) => Value::Float(a.f() + b.f()),
        ("-", a, b) => Value::Float(a.f() - b.f()),
        ("*", a, b) => Value::Float(a.f() * b.f()),
        ("/", _, b) if b.f() == 0.0 => return Err(zero()),
        ("/", a, b) => Value::Float(a.f() / b.f()),
        ("d_iv" | "m_od", a, b) => integer(op, a, b, span)?,
        ("^", a, b) => power(a, b, span)?,
        ("m_ax", a, b) => {
            if a.f() >= b.f() {
                a.into()
            } else {
                b.into()
            }
        }
        ("m_in", a, b) => {
            if a.f() <= b.f() {
                a.into()
            } else {
                b.into()
            }
        }
        _ => {
            return Err(err(
                "unknown-builtin",
                span,
                format!("no arithmetic for {op}"),
            ));
        }
    })
}

/// Floor division and remainder on Ints, in maths order (B7).
fn integer<'a>(op: &str, a: Num, b: Num, span: Span) -> Result<Value<'a>, Diagnostic> {
    let overflow = || err("integer-overflow", span, "integer overflow");
    match (a, b) {
        (Num::I(_), Num::I(0)) => Err(err("division-by-zero", span, "division by zero")),
        (Num::I(x), Num::I(y)) => {
            let (q, r) = (x.checked_div(y).ok_or_else(overflow)?, x % y);
            let floor = r != 0 && ((r < 0) != (y < 0));
            Ok(Value::Int(if op == "d_iv" {
                if floor { q - 1 } else { q }
            } else if floor {
                r + y
            } else {
                r
            }))
        }
        _ => Err(err("not-an-integer", span, format!("{op} needs integers"))),
    }
}

fn power<'a>(a: Num, b: Num, span: Span) -> Result<Value<'a>, Diagnostic> {
    match (a, b) {
        (Num::I(_), Num::I(e)) if e < 0 => Err(err(
            "negative-exponent",
            span,
            "an Int raised to a negative Int: use a Float base, e.g. 2.0 ^ n",
        )),
        (Num::I(x), Num::I(e)) => u32::try_from(e)
            .ok()
            .and_then(|e| x.checked_pow(e))
            .map(Value::Int)
            .ok_or_else(|| {
                err(
                    "integer-overflow",
                    span,
                    "integer overflow (use a Float, e.g. 2.0 ^ n)",
                )
            }),
        (a, b) if a.f() < 0.0 && b.f().fract() != 0.0 => Err(err(
            "complex-result",
            span,
            "a negative base with a fractional exponent has no real result",
        )),
        (a, b) => Ok(Value::Float(a.f().powf(b.f()))),
    }
}

/// Comparisons: exact, numeric across Int and Float (T3).
pub fn compare<'a>(op: &str, a: Num, b: Num) -> Value<'a> {
    let (x, y) = (a.f(), b.f());
    let exact_eq = match (a, b) {
        (Num::I(i), Num::I(j)) => i == j,
        _ => x == y,
    };
    Value::Bool(match op {
        "=" => exact_eq,
        "!=" => !exact_eq,
        "<" => x < y,
        ">" => x > y,
        "<=" => x <= y,
        ">=" => x >= y,
        _ => (x - y).abs() <= 1e-14 * x.abs().max(y.abs()),
    })
}

impl<'a> From<Num> for Value<'a> {
    fn from(n: Num) -> Self {
        match n {
            Num::I(i) => Value::Int(i),
            Num::F(x) => Value::Float(x),
        }
    }
}
