//! String literals: a raw string (ST4) keeps the form written, any
//! other is printed with its escapes; a code string on a macro's side
//! (ST5) takes the canonical form of its inside.

use xetal_base::Span;

/// The literal for `text`, written at `span` of `src`.
pub(crate) fn string_text(src: &str, span: Span, text: &str) -> String {
    match src.get(span.start..span.end) {
        // Raw and code strings keep the form written (ST4, ST5): a code
        // string in a value position is a string that happens to lex.
        Some(written) if written.starts_with(['r', 'c']) => written.to_string(),
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

/// The literal for `text` written at `span` of `src`, a macro's side: a
/// code string whose inside parses is written with that inside in
/// canonical form, in the form it was written in; any other as usual.
pub(crate) fn code_text(src: &str, span: Span, text: &str) -> String {
    let written = src.get(span.start..span.end).unwrap_or_default();
    let (Some(rest), Some(body)) = (written.strip_prefix('c'), crate::canonical::body(text)) else {
        return string_text(src, span, text);
    };
    let hashes = &rest[..rest.len() - rest.trim_start_matches('#').len()];
    match hashes.is_empty() {
        true => format!("c\"{}\"", body.replace('\\', "\\\\").replace('"', "\\\"")),
        false if !body.contains(&format!("\"{hashes}")) => format!("c{hashes}\"{body}\"{hashes}"),
        false => written.to_string(),
    }
}
