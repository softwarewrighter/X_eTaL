//! Style-guide names (lang-choices section 16): functions lowerCamel,
//! a single capital only for a combinator, constants UpperCamel and
//! mutable variables lowerCamel.

use xetal_base::{Diagnostic, Span};
use xetal_ir::{Item, Program};

use crate::walk::{binders, name_at};

/// The naming lints of every binding in `program`, each at the name.
pub(crate) fn naming(program: &Program) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for item in &program.items {
        let value = match item {
            Item::Def { name, value } | Item::Let { name, value, .. } => {
                let at = program.name_span(value.id).unwrap_or(value.span);
                out.extend(check(program, name, at));
                value
            }
            Item::Set { value, .. } | Item::Eval(value) => value,
        };
        binders(value, &mut |name, e| {
            out.extend(check(program, name, name_at(program, e)))
        });
    }
    out
}

/// The naming lint `name` breaks, if any; `span` is where it is bound.
/// A mutable variable's fix renames it everywhere it is written.
fn check(program: &Program, name: &str, span: Span) -> Option<Diagnostic> {
    let base = name.rsplit_once(':').map_or(name, |(_, key)| key);
    let first = base.chars().next().filter(char::is_ascii_uppercase)?;
    let lower = format!("{}{}", first.to_ascii_lowercase(), &base[1..]);
    let (code, message) = match base.split_once('_') {
        Some((head, tail)) if head.len() > 1 || tail.chars().any(|c| c.is_ascii_alphabetic()) => (
            "function-name-case",
            format!(
                "function `{base}` is UpperCamel: write `{lower}` (a capital start is for combinators)"
            ),
        ),
        Some(_) => return None,
        None if base.ends_with('!') => (
            "mutable-constant",
            format!("`{base}` is mutable but named like a constant: write `{lower}`"),
        ),
        None => upper(base)?,
    };
    let found = Diagnostic::warning(code, message).with_span(span);
    let renamed = format!("{}{lower}", &name[..name.len() - base.len()]);
    let fix = (code == "mutable-constant").then(|| program.renamed(name, &renamed));
    Some(found.with_fixes(fix.flatten().unwrap_or_default()))
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
