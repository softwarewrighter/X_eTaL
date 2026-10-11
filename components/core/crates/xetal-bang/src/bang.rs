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
        let finding = match (marked, direct(value)) {
            (false, Some(effect)) => Some((
                "missing-bang",
                format!(
                    "function `{name}` calls `{effect}`, an effect, but its name has no `!`: write `{name}!`"
                ),
            )),
            (true, None) if !possible(value) => Some((
                "needless-bang",
                format!(
                    "function `{name}` is named for an effect (`!`) but calls none: write `{}`",
                    name.trim_end_matches('!')
                ),
            )),
            _ => None,
        };
        out.extend(finding.map(|(code, m)| Diagnostic::warning(code, m).with_span(value.span)));
    }
    out
}
