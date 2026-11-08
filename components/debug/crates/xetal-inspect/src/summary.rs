//! A value on one line: its type and shape, then its first items.

use xetal_value::Value;

/// How many items a summary shows before `...`.
const SHOWN: usize = 8;

/// The element type's name: `Int`, `Float`, `Char`, `Bool`, `Box`, a
/// tuple's parts, `function`, `@`.
pub fn type_name(v: &Value) -> String {
    match v {
        Value::Int(_) => "Int".into(),
        Value::Float(_) => "Float".into(),
        Value::Bool(_) => "Bool".into(),
        Value::Char(_) => "Char".into(),
        Value::Unit => "Unit".into(),
        Value::Boxed(_) => "Box".into(),
        Value::Closure(_) | Value::Prim(_) => "function".into(),
        Value::Tuple(parts) => {
            let names: Vec<String> = parts.iter().map(type_name).collect();
            format!("({})", names.join(", "))
        }
        Value::Array(a) => a.data().first().map_or("Array".into(), type_name),
        other => match other {
            Value::Tag(ty, _) => (*ty).to_string(),
            _ => "value".into(),
        },
    }
}

/// `Float 4 3: 0.42 0.45 0.07 ...`, a string as its text in quotes, a
/// scalar as it prints, a tuple as its parts' summaries.
pub fn summary(v: &Value) -> String {
    match v {
        Value::Array(a) => {
            let shape: Vec<String> = a.shape().iter().map(ToString::to_string).collect();
            let chars =
                !a.data().is_empty() && a.data().iter().all(|x| matches!(x, Value::Char(_)));
            if chars && a.rank() == 1 {
                let text: String = a.data().iter().map(ToString::to_string).collect();
                return format!("Char {}: {text:?}", shape.join(" "));
            }
            let items: Vec<String> = a.data().iter().take(SHOWN).map(short).collect();
            let more = if a.data().len() > SHOWN { " ..." } else { "" };
            format!(
                "{} {}: {}{more}",
                type_name(v),
                shape.join(" "),
                items.join(" ")
            )
        }
        Value::Tuple(parts) => {
            let shown: Vec<String> = parts.iter().map(summary).collect();
            format!("({})", shown.join(", "))
        }
        Value::Boxed(inner) => format!("Box of {}", summary(inner)),
        Value::Closure(_) => "<function>".into(),
        Value::Prim(p) => format!("<built-in {}>", p.name),
        other => short(other),
    }
}

/// An item inside a summary: a scalar as it prints, else a mark.
fn short(v: &Value) -> String {
    match v {
        Value::Boxed(_) => "[box]".into(),
        Value::Array(_) | Value::Tuple(_) => "[...]".into(),
        other => other.to_string(),
    }
}

/// A breakpoint condition's answer: 1, a true Bool or a non-zero
/// number, as a single value or the first of an array.
pub fn truthy(v: &Value) -> bool {
    match v {
        Value::Bool(b) => *b,
        Value::Int(i) => *i != 0,
        Value::Float(x) => *x != 0.0,
        Value::Array(a) => a.data().first().is_some_and(truthy),
        _ => false,
    }
}
