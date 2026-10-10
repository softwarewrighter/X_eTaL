//! Style-guide names (lang-choices section 16): functions lowerCamel,
//! a single capital only for a combinator, constants UpperCamel and
//! mutable variables lowerCamel.

use xetal_base::{Diagnostic, Span};
use xetal_ir::{Item, Program};

use crate::walk::binders;

/// The naming lints of every binding in `program`.
pub(crate) fn naming(program: &Program) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for item in &program.items {
        let value = match item {
            Item::Def { name, value } | Item::Let { name, value, .. } => {
                out.extend(check(name, value.span));
                value
            }
            Item::Set { value, .. } | Item::Eval(value) => value,
        };
        binders(value, &mut |name, e| out.extend(check(name, e.span)));
    }
    out
}

/// The naming lint `name` breaks, if any; `span` is where it is bound.
fn check(name: &str, span: Span) -> Option<Diagnostic> {
    let base = name.rsplit_once(':').map_or(name, |(_, key)| key);
    let first = base.chars().next().filter(char::is_ascii_uppercase)?;
    let (code, message) = match base.split_once('_') {
        Some((head, tail)) if head.len() > 1 || tail.chars().any(|c| c.is_ascii_alphabetic()) => (
            "function-name-case",
            format!(
                "function `{base}` is UpperCamel: write `{}{}` (a single capital such as `K_` is a combinator)",
                first.to_ascii_lowercase(),
                &base[1..]
            ),
        ),
        Some(_) => return None,
        None if base.ends_with('!') => (
            "mutable-constant",
            format!(
                "`{base}` is mutable but named like a constant: write `{}{}`",
                first.to_ascii_lowercase(),
                &base[1..]
            ),
        ),
        None => upper(base)?,
    };
    Some(Diagnostic::warning(code, message).with_span(span))
}

/// `upper-case-name`: two or more letters, every one a capital.
fn upper(base: &str) -> Option<(&'static str, String)> {
    let letters = base.chars().filter(char::is_ascii_alphabetic);
    if letters.clone().count() < 2 || letters.clone().any(|c| c.is_ascii_lowercase()) {
        return None;
    }
    let message = format!(
        "`{base}` is all capitals: write a constant UpperCamel (`MaxSize`) or a variable lowerCamel (`maxSize`)"
    );
    Some(("upper-case-name", message))
}
