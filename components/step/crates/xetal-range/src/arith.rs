//! A progression shifted or scaled by an Int, and its sum (`'+ r_/`).

use xetal_value::Value;

pub(crate) fn call<'a>(name: &str, args: &[Value<'a>]) -> Option<Value<'a>> {
    let p = match (name, args) {
        ("+", [Value::Range(p), Value::Int(d)] | [Value::Int(d), Value::Range(p)]) => p.shifted(*d),
        ("-", [Value::Range(p), Value::Int(d)]) => p.shifted(d.checked_neg()?),
        ("-", [Value::Int(d), Value::Range(p)]) => p.scaled(-1)?.shifted(*d),
        ("*", [Value::Range(p), Value::Int(k)] | [Value::Int(k), Value::Range(p)]) => p.scaled(*k),
        ("r_/", [Value::Prim(f), Value::Range(p)]) if plus(f) => {
            return p.sum().map(Value::Int);
        }
        _ => return None,
    };
    p.map(Value::Range)
}

/// `'+` itself, not partly applied.
fn plus(f: &xetal_value::Prim<'_>) -> bool {
    f.name == "+" && f.args.is_empty()
}
