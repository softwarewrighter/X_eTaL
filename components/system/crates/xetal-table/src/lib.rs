//! Tabular data from a TOML file, strings only and data only (RS2):
//! `"file" []L_IST "name"` gives a top-level array of strings as a
//! vector of boxed texts, and `("file" "name") []T_ABLE ("rows" "cols")`
//! gives the table of tables `name` as a matrix of texts whose rows
//! and columns follow the two named top-level lists, a missing cell the
//! empty string. Nothing in the file is ever evaluated; a value that is
//! not a string, a missing list, or a file that does not parse is an
//! error naming the file and the key. The `.xtln` reader (Saga 37) is
//! meant to sit behind these same two names.

mod read;

use xetal_base::{Diagnostic, Span};
use xetal_value::Value;

/// Call `name` on `args`, if it is one of these.
pub fn call<'a>(
    name: &str,
    args: &[Value<'a>],
    span: Span,
) -> Option<Result<Value<'a>, Diagnostic>> {
    let result = match (name, args) {
        ("[]L_IST", [file, key]) => read::list(file, key),
        ("[]T_ABLE", [what, axes]) => read::table(what, axes),
        _ => return None,
    };
    Some(result.map_err(|d| d.with_span(span)))
}
