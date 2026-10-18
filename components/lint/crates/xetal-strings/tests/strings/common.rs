//! Helpers the string-lint tests share: findings by code, fixes made,
//! and the string values a text lexes to.

use xetal_lex::{TokenKind, lex};

/// The codes `strings` reports for `text`, in order.
pub fn codes(text: &str) -> Vec<String> {
    xetal_strings::strings(text)
        .into_iter()
        .map(|d| d.code)
        .collect()
}

/// `text` with every fix of lint `code` made (each fix's edits from the
/// end, so offsets stay true; the lints' fixes never overlap).
pub fn fixed(text: &str, code: &str) -> String {
    let mut edits: Vec<_> = xetal_strings::strings(text)
        .into_iter()
        .filter(|d| d.code == code)
        .flat_map(|d| d.fix().to_vec())
        .collect();
    edits.sort_by_key(|f| std::cmp::Reverse(f.span.start));
    let mut out = text.to_string();
    for f in edits {
        out.replace_range(f.span.start..f.span.end, &f.replacement);
    }
    out
}

/// The values of the string literals `text` lexes to, in order.
pub fn values(text: &str) -> Vec<String> {
    let tokens = lex(text).expect("lexes");
    tokens
        .into_iter()
        .filter_map(|t| match t.kind {
            TokenKind::Str(s) => Some(s),
            _ => None,
        })
        .collect()
}

/// Every fix of `code` in `text` leaves each string's value byte for
/// byte as it was (D156); returns the fixed text.
pub fn preserved(text: &str, code: &str) -> String {
    let after = fixed(text, code);
    assert_eq!(values(&after), values(text), "{text:?} -> {after:?}");
    after
}
