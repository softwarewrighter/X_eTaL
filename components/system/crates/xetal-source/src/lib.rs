//! Source text as data (RS5): `[]V_IEW "1 o_- v"` gives the source as
//! xetal-view classifies it for the editor, the HTML export and the
//! poster, a matrix of boxed texts with a row per segment and two
//! columns: the segment's decorated text (underlines drawn, as `xetal
//! render` shows it) and its class, spelled as the HTML export's CSS
//! classes are without the prefix (`builtin`, `userfunc`, `libfunc`,
//! `variable`, `lambdaarg`, `number`, `exponent`, `string`, `symbol`,
//! `quote`, `punct`, `unit`, `comment`, `space`, `macro`, `error`).
//! Runs of one class are one row. A program maps the classes to colors
//! itself, so the view and its coloring never drift from the language.

use std::rc::Rc;

use xetal_base::{Diagnostic, Span};
use xetal_value::{Value, as_array};

/// Call `name` on `args`, if it is one of these.
pub fn call<'a>(
    name: &str,
    args: &[Value<'a>],
    span: Span,
) -> Option<Result<Value<'a>, Diagnostic>> {
    match (name, args) {
        ("[]V_IEW", [src]) => Some(view(src).map_err(|d| d.with_span(span))),
        _ => None,
    }
}

/// The characters of a text value, or a domain error.
fn text(v: &Value<'_>) -> Result<String, Diagnostic> {
    as_array(v)
        .data()
        .iter()
        .map(|c| match c {
            Value::Char(c) => Ok(*c),
            other => Err(Diagnostic::new(
                "domain",
                format!("expected text, got {other}"),
            )),
        })
        .collect()
}

/// A text as a boxed Char vector.
fn boxed<'a>(t: &str) -> Value<'a> {
    Value::Boxed(Rc::new(xetal_value::to_value(xetal_array::Array::vector(
        t.chars().map(Value::Char).collect(),
    ))))
}

/// `[]V_IEW src`: the segments, runs of one class merged, as an n x 2
/// matrix of boxed texts (decorated text, class).
fn view<'a>(src: &Value<'a>) -> Result<Value<'a>, Diagnostic> {
    let segments = xetal_view::view(&text(src)?);
    let mut cells: Vec<Value<'a>> = Vec::new();
    let mut rest = segments.as_slice();
    while let Some(first) = rest.first() {
        let n = rest.iter().take_while(|s| s.class == first.class).count();
        let run: String = rest[..n].iter().map(|s| s.text.as_str()).collect();
        cells.push(boxed(&run));
        cells.push(boxed(&format!("{:?}", first.class).to_lowercase()));
        rest = &rest[n..];
    }
    let rows = cells.len() / 2;
    let matrix = xetal_array::Array::new(vec![rows, 2], cells)
        .map_err(|e| Diagnostic::new("internal", format!("{e:?}")))?;
    Ok(xetal_value::to_value(matrix))
}
