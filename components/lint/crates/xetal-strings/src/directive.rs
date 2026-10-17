//! `string-holds-directive`: a line of a string literal that reads as a
//! `# lint:` or `# lint-file:` comment. Inside a string it is text and
//! sets no level; the author probably meant a comment.

use xetal_base::{Diagnostic, Span};

use crate::literal::Literal;

/// One finding for each line of `lit` that reads as a directive, at its
/// text (leading and trailing blanks left out). No fix: only the author
/// knows whether the text belongs in the string.
pub(crate) fn string_holds_directive(lit: &Literal) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    let mut at = lit.body_at;
    for line in lit.body.split('\n') {
        let trimmed = line.trim_start_matches([' ', '\t']);
        let start = at + line.len() - trimmed.len();
        let text = trimmed.trim_end_matches([' ', '\t', '\r']);
        if text.starts_with("# lint:") || text.starts_with("# lint-file:") {
            let message = format!(
                "`{text}` is inside a string, where it is text and sets no level: to set one, write it as a comment outside the string"
            );
            let span = Span::new(start, start + text.len());
            out.push(Diagnostic::warning("string-holds-directive", message).with_span(span));
        }
        at += line.len() + 1;
    }
    out
}
