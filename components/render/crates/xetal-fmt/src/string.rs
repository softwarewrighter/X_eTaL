//! String literals: a raw string (ST4) keeps the form written, any
//! other is printed with its escapes.

use xetal_base::Span;

/// The literal for `text`, written at `span` of `src`.
pub(crate) fn string_text(src: &str, span: Span, text: &str) -> String {
    match src.get(span.start..span.end) {
        Some(written) if written.starts_with('r') => written.to_string(),
        _ => {
            let escaped = text
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\t', "\\t");
            format!("\"{escaped}\"")
        }
    }
}
