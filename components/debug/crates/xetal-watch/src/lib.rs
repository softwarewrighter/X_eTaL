//! A watch checked as the program was (Saga 43): the type checker
//! elaborates numeric code (a function over any number takes a hidden
//! argument, a zero of the right type, and every call passes it), so a
//! watch that calls the program's functions must be checked against the
//! program too. The watch is appended to the main file's text inside
//! lambdas whose parameters are the stopped frame's locals, each given
//! a placeholder of its value's type (`0`, `0.0`, `"a"`, ...); the whole
//! is loaded and checked, and the elaborated watch is taken out of its
//! lambdas, to run against the real values where the program stopped.
//! Also a breakpoint's hit condition ([`hit`]).

use xetal_core::{Expr, Item, Kind};
use xetal_value::Value;

/// A literal of `v`'s type, to stand for a local while checking; none
/// for a function (a watch cannot name a local function).
pub fn placeholder(v: &Value) -> Option<String> {
    Some(match v {
        Value::Int(_) => "0".into(),
        Value::Float(_) => "0.0".into(),
        Value::Bool(_) => "(1 = 1)".into(),
        Value::Char(_) => "\"a\"".into(),
        Value::Array(a) => placeholder(a.data().first()?)?,
        Value::Boxed(inner) => format!("(e_nclose {})", placeholder(inner)?),
        Value::Tuple(parts) => {
            let shown: Option<Vec<String>> = parts.iter().map(placeholder).collect();
            format!("({})", shown?.join(", "))
        }
        _ => return None,
    })
}

/// `watch` checked in file `name` whose text is `text`, with `locals`
/// (name, placeholder) in scope, innermost last: the elaborated watch.
pub fn checked(
    name: &str,
    text: &str,
    locals: &[(String, String)],
    watch: &str,
) -> Result<Expr, String> {
    let named: Vec<&(String, String)> = locals.iter().filter(|(n, _)| n != "_r").collect();
    let right = locals.iter().find(|(n, _)| n == "_r");
    let mut source = format!("({watch})");
    if let Some((_, p)) = right {
        source = format!("{{ {source} }} {p}");
    }
    for (n, p) in named.iter().rev() {
        source = format!("{{ {n} -> {source} }} {p}");
    }
    let program = format!("{}\n{source}\n", text.trim_end());
    let mut loaded = xetal_program::load(name, &program).map_err(|d| d.message)?;
    xetal_types::check_program(&mut loaded.program).map_err(|d| d.message)?;
    let Some(Item::Eval(mut e)) = loaded.program.items.pop() else {
        return Err("the watch is not an expression".into());
    };
    for _ in 0..named.len() + usize::from(right.is_some()) {
        e = unwrap(e).ok_or("the watch's frame did not check")?;
    }
    Ok(e)
}

/// `{ x -> body } p` gives `body`.
fn unwrap(e: Expr) -> Option<Expr> {
    match e.kind {
        Kind::App(f, _) => match f.kind {
            Kind::Lam { body, .. } => Some(*body),
            _ => None,
        },
        _ => None,
    }
}

/// Whether hit `n` (from 1) breaks for the hit condition `spec`.
pub fn hit(spec: &str, n: usize) -> bool {
    let spec = spec.trim();
    let num = |s: &str| s.trim().parse::<usize>().ok();
    let (op, rest) = match spec.find(|c: char| c.is_ascii_digit()) {
        Some(i) => spec.split_at(i),
        None => return true,
    };
    let Some(k) = num(rest) else {
        return true;
    };
    match op.trim() {
        "" | ">=" => n >= k,
        "==" | "=" => n == k,
        ">" => n > k,
        "%" => k > 0 && n.is_multiple_of(k),
        _ => true,
    }
}
