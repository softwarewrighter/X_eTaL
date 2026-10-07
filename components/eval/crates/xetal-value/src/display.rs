//! Printed results (lang-choices 10a). A tuple prints on one line,
//! `(1 2 3, 4.5)`, when every part does; otherwise one block per part,
//! introduced by its position, a tall part's lines indented under it
//! (TU6):
//!
//! ```text
//! (1:
//!  1 2 3
//!  4 5 6
//! , 2: 4.5)
//! ```

use std::fmt;

use xetal_array::layout;

use crate::Value;
use crate::shown::{nested, shown};

impl fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{i}"),
            Value::Float(x) if x.is_finite() && x.fract() == 0.0 => write!(f, "{x}.0"),
            Value::Float(x) => write!(f, "{x}"),
            Value::Bool(b) => f.write_str(if *b { "1" } else { "0" }),
            Value::Char(c) => write!(f, "{c}"),
            Value::Tag(ty, i) => f.write_str(&crate::tags::name(ty, *i)),
            Value::Event(e) => write!(f, "{e}"),
            Value::Unit => f.write_str("@"),
            Value::Boxed(_) | Value::Array(_) if nested(self) => {
                f.write_str(&xetal_grid::display(&shown(self)).join("\n"))
            }
            Value::Boxed(x) => write!(f, "{x}"),
            Value::Array(a) => {
                let cells: Vec<String> = a.data().iter().map(ToString::to_string).collect();
                let chars = a.data().iter().all(|v| matches!(v, Value::Char(_)));
                let sep = if chars && !cells.is_empty() { "" } else { " " };
                f.write_str(&layout(a.shape(), &cells, sep))
            }
            Value::Closure(_) | Value::Prim(_) => f.write_str("<function>"),
            Value::Error(e) => write!(f, "error[{}]: {}", e.code, e.message),
            Value::Outcome(_) => f.write_str("<outcome>"),
            Value::Tuple(parts) => f.write_str(&tuple_text(parts)),
        }
    }
}

/// Any value as DISPLAY draws it, flat arrays framed too (`d_isplay`);
/// a simple scalar is its printed text.
pub fn picture(v: &Value<'_>) -> Vec<String> {
    xetal_grid::display(&shown(v))
}

/// A value as a result prints: boxed when `--box` (or Boxed) is on and
/// it is an array, else as it displays.
pub fn printed(v: &Value<'_>) -> String {
    match v {
        Value::Array(_) | Value::Boxed(_) if xetal_grid::boxed() => picture(v).join("\n"),
        other => other.to_string(),
    }
}

/// The printed text of a tuple with these parts.
fn tuple_text(parts: &[Value<'_>]) -> String {
    let texts: Vec<String> = parts.iter().map(printed).collect();
    if texts.iter().all(|t| !t.contains('\n')) {
        return format!("({})", texts.join(", "));
    }
    let mut out = String::new();
    for (i, part) in texts.iter().enumerate() {
        let lead = if i == 0 { "(" } else { "\n, " };
        out.push_str(&format!("{lead}{}:", i + 1));
        out.push_str(&block(part));
    }
    let tall_last = texts.last().is_some_and(|t| t.contains('\n'));
    out.push_str(if tall_last { "\n)" } else { ")" });
    out
}

/// A part after its position: a line on the same line, a block on the
/// lines below, each indented by one space.
fn block(part: &str) -> String {
    if part.contains('\n') {
        part.lines().map(|l| format!("\n {l}")).collect()
    } else {
        format!(" {part}")
    }
}
