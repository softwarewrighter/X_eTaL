//! Reading the file and taking the lists and tables out of it.

use std::rc::Rc;

use xetal_base::Diagnostic;
use xetal_value::{Value, as_array};

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

/// The two texts of a strand of two strings (each boxed), or a domain
/// error naming the built-in.
fn pair(v: &Value<'_>, what: &str) -> Result<(String, String), Diagnostic> {
    let items = as_array(v);
    let open = |x: &Value<'_>| match x {
        Value::Boxed(b) => text(b),
        other => text(other),
    };
    match items.data() {
        [a, b] => Ok((open(a)?, open(b)?)),
        other => Err(Diagnostic::new(
            "domain",
            format!("{what} takes two strings, got {} items", other.len()),
        )),
    }
}

/// The file parsed as TOML, or an error naming it.
fn parsed(file: &str) -> Result<toml::Table, Diagnostic> {
    let source =
        xetal_store::read(file).map_err(|e| Diagnostic::new("io", format!("[]T_ABLE: {e}")))?;
    source
        .parse::<toml::Table>()
        .map_err(|e| Diagnostic::new("bad-table", format!("{file}: {}", e.message())))
}

/// A top-level array of strings, as texts.
fn strings(doc: &toml::Table, file: &str, key: &str) -> Result<Vec<String>, Diagnostic> {
    let bad = |why: &str| Diagnostic::new("bad-table", format!("{file}: {key}: {why}"));
    let items = doc
        .get(key)
        .ok_or_else(|| bad("no such list at the top level"))?
        .as_array()
        .ok_or_else(|| bad("not a list"))?;
    items
        .iter()
        .map(|v| {
            v.as_str()
                .map(String::from)
                .ok_or_else(|| bad("an item is not a string"))
        })
        .collect()
}

/// Boxed texts as a vector value.
fn boxed<'a>(texts: impl Iterator<Item = String>) -> Vec<Value<'a>> {
    texts
        .map(|t| {
            Value::Boxed(Rc::new(xetal_value::to_value(xetal_array::Array::vector(
                t.chars().map(Value::Char).collect(),
            ))))
        })
        .collect()
}

/// `"file" []L_IST "name"`.
pub(crate) fn list<'a>(file: &Value<'a>, key: &Value<'a>) -> Result<Value<'a>, Diagnostic> {
    let (file, key) = (text(file)?, text(key)?);
    let items = strings(&parsed(&file)?, &file, &key)?;
    Ok(xetal_value::to_value(xetal_array::Array::vector(boxed(
        items.into_iter(),
    ))))
}

/// `("file" "name") []T_ABLE ("rows" "cols")`: the cells row by row.
pub(crate) fn table<'a>(what: &Value<'a>, axes: &Value<'a>) -> Result<Value<'a>, Diagnostic> {
    let (file, name) = pair(what, "[]T_ABLE's left")?;
    let (rows, cols) = pair(axes, "[]T_ABLE's right")?;
    let doc = parsed(&file)?;
    let (rows, cols) = (strings(&doc, &file, &rows)?, strings(&doc, &file, &cols)?);
    let bad = |why: String| Diagnostic::new("bad-table", format!("{file}: {name}: {why}"));
    let table = doc
        .get(&name)
        .ok_or_else(|| bad("no such table at the top level".into()))?
        .as_table()
        .ok_or_else(|| bad("not a table".into()))?;
    let mut cells = Vec::with_capacity(rows.len() * cols.len());
    for r in &rows {
        let row = match table.get(r) {
            None => None,
            Some(v) => Some(
                v.as_table()
                    .ok_or_else(|| bad(format!("{r}: not a table")))?,
            ),
        };
        for c in &cols {
            cells.push(match row.and_then(|t| t.get(c)) {
                None => String::new(),
                Some(v) => v
                    .as_str()
                    .map(String::from)
                    .ok_or_else(|| bad(format!("{r}.{c}: not a string")))?,
            });
        }
    }
    let shape = vec![rows.len(), cols.len()];
    let matrix = xetal_array::Array::new(shape, boxed(cells.into_iter()))
        .map_err(|e| Diagnostic::new("internal", format!("{e:?}")))?;
    Ok(xetal_value::to_value(matrix))
}
