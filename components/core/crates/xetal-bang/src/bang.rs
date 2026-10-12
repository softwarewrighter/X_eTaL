//! The findings: each top-level function definition against its name.

use xetal_base::Diagnostic;
use xetal_ir::{Item, Program};

use crate::effect::{direct, possible};

/// `missing-bang` and `needless-bang` for the program's functions.
pub fn bangs(program: &Program) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for item in &program.items {
        let Item::Def { name, value } = item else {
            continue;
        };
        let marked = name.ends_with('!');
        let renamed = match marked {
            true => name.trim_end_matches('!').to_string(),
            false => format!("{name}!"),
        };
        let (code, effect) = match (marked, direct(value)) {
            (false, Some(effect)) => ("missing-bang", effect),
            (true, None) if !possible(value) => ("needless-bang", ""),
            _ => continue,
        };
        let message = match code {
            "missing-bang" => format!(
                "function `{name}` calls `{effect}`, an effect, but its name has no `!`: write `{renamed}`"
            ),
            _ => format!(
                "function `{name}` is named for an effect (`!`) but calls none: write `{renamed}`"
            ),
        };
        let at = program.name_span(value.id).unwrap_or(value.span);
        let found = Diagnostic::warning(code, message).with_span(at);
        out.push(found.with_fixes(rename(program, name, &renamed)));
    }
    out
}

/// The edits renaming function `name` to `renamed` everywhere it is
/// written; none for a library's exported function (its callers are in
/// other files) or when the rename cannot be made surely.
fn rename(program: &Program, name: &str, renamed: &str) -> Vec<(xetal_base::Span, String)> {
    let local = name.starts_with("u:") || name.starts_with("h:");
    let edits = local.then(|| program.renamed(name, renamed));
    edits.flatten().unwrap_or_default()
}
