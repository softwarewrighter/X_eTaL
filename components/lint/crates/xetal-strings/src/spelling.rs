//! The lints on how a literal is spelled, each with a fix to the
//! spelling that reads better: `needless-raw-hashes`,
//! `needless-raw-string` and `escapes-could-be-raw`.

use xetal_base::{Diagnostic, Span};

use crate::form::{Form, fewest_hashes};
use crate::literal::{Literal, holds};

/// A raw string with more `#` than it needs.
pub(crate) fn needless_raw_hashes(lit: &Literal) -> Option<Diagnostic> {
    let Form::Raw(n) = lit.form else {
        return None;
    };
    let fewest = fewest_hashes(lit.body);
    let to = Form::Raw(fewest);
    let message = format!(
        "this raw string has {n} `#` where {fewest} will do: open it with `{}`",
        to.open()
    );
    (fewest < n).then(|| found(lit, "needless-raw-hashes", message, to))
}

/// A raw string with no backslash, quote or newline: plain is the same.
pub(crate) fn needless_raw_string(lit: &Literal) -> Option<Diagnostic> {
    let plain_would_do = !lit.body.contains(['\\', '"', '\n']);
    let message = "nothing in this raw string needs it to be raw: write it plain, `\"...\"`";
    (matches!(lit.form, Form::Raw(_)) && plain_would_do)
        .then(|| found(lit, "needless-raw-string", message.to_string(), Form::Plain))
}

/// A plain string with two or more `\"` or `\\` and no `\n` or `\t`:
/// clearer raw, with the fewest `#` that work.
pub(crate) fn escapes_could_be_raw(lit: &Literal) -> Option<Diagnostic> {
    if lit.form != Form::Plain {
        return None;
    }
    let n = quoting(lit.body).filter(|&n| n >= 2)?;
    let to = Form::Raw(fewest_hashes(&lit.value));
    let spelling = format!("{}{}{}", to.open(), lit.value, to.close());
    let (open, close) = (to.open(), to.close());
    let message =
        format!("its {n} escapes read more clearly in a raw string: write `{open}...{close}`");
    let warning = Diagnostic::warning("escapes-could-be-raw", message).with_span(lit.span);
    Some(match holds(&spelling, &lit.value) {
        true => warning.with_fixes([(lit.span, spelling)]),
        false => warning,
    })
}

/// How many `\"` and `\\` a plain body has; `None` when it has a `\n`
/// or `\t` (a raw string cannot write those on one line).
fn quoting(body: &str) -> Option<usize> {
    let (bytes, mut i, mut n) = (body.as_bytes(), 0, 0);
    while i < bytes.len() {
        if bytes[i] == b'\\' {
            match bytes.get(i + 1) {
                Some(b'"' | b'\\') => n += 1,
                _ => return None,
            }
            i += 1;
        }
        i += 1;
    }
    Some(n)
}

/// Lint `code` on `lit`, fixed by writing its delimiters as form `to`
/// when that keeps its value.
fn found(lit: &Literal, code: &str, message: String, to: Form) -> Diagnostic {
    let warning = Diagnostic::warning(code, message).with_span(lit.span);
    let edits: Vec<(Span, String)> = lit.delimiters(to);
    match holds(&lit.respelled(to), &lit.value) {
        true => warning.with_fixes(edits),
        false => warning,
    }
}
