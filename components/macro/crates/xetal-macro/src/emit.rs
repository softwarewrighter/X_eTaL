//! Writing one file into the combined text, and naming the hidden
//! namespaces.

use std::collections::HashMap;

use xetal_base::Diagnostic;
use xetal_names::{Edit, Import};
use xetal_sources::Sources;

/// Hidden namespaces of the k-th library: `LA` and `PA`, `LB` and `PB`, ...
pub(crate) fn hidden(mut k: usize) -> (String, String) {
    let mut letters = String::new();
    loop {
        letters.insert(0, char::from(b'A' + (k % 26) as u8));
        if k < 26 {
            break;
        }
        k = k / 26 - 1;
    }
    (format!("L{letters}"), format!("P{letters}"))
}

/// Append a file's text: import statements removed, names renamed.
pub(crate) fn emit(
    sources: &mut Sources,
    index: usize,
    text: &str,
    found: &[Import],
    mut edits: Vec<Edit>,
) {
    edits.extend(
        found
            .iter()
            .map(|i| (i.span.start..i.span.end, String::new())),
    );
    edits.sort_by_key(|(range, _)| range.start);
    let mut at = 0;
    for (range, new) in edits {
        sources.copy(index, at..range.start);
        sources.replace(index, range.clone(), &new);
        at = range.end;
    }
    sources.copy(index, at..text.len());
}

/// Note that `letters` names the library `key`: one letter for two
/// libraries, or one library under two letters, is an error (MC8 rows
/// 6 and 7).
pub(crate) fn once(
    keys: &mut HashMap<String, String>,
    letters: String,
    key: String,
    import: &Import,
) -> Result<(), Diagnostic> {
    if let Some(other) = keys.insert(letters, key.clone()) {
        let code = if other == key {
            "library-reimported"
        } else {
            "alias-reused"
        };
        return Err(twice(code, import));
    }
    if keys.values().filter(|k| **k == key).count() > 1 {
        return Err(twice("library-reimported", import));
    }
    Ok(())
}

fn twice(code: &str, import: &Import) -> Diagnostic {
    let message = match code {
        "alias-reused" => format!(
            "{} already names another library in this file",
            import.alias
        ),
        _ => format!(
            "{:?} is already imported in this file under another alias",
            import.spec
        ),
    };
    Diagnostic::new(code, message).with_span(import.span)
}

/// A macro library's text with its own prefix, `m:`, written as the
/// alias its caller imported it under (MC33, as MC5 does for `l:`):
/// `m:d_ouble` becomes `w:d_ouble` for a call of `w:t_wice<`. Strings
/// and comments are tokens of their own and stay as written; text that
/// does not lex is left for the caller's errors.
pub(crate) fn aliased(text: String, alias: &str) -> String {
    let Ok(tokens) = xetal_lex::lex(&text) else {
        return text;
    };
    let mut out = String::with_capacity(text.len());
    let mut at = 0;
    for t in tokens {
        if text[t.span.start..t.span.end].starts_with("m:") {
            out.push_str(&text[at..t.span.start]);
            out.push_str(alias);
            out.push(':');
            at = t.span.start + 2;
        }
    }
    out.push_str(&text[at..]);
    out
}
