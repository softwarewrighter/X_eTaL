//! The macro library's definitions as the passes see them: each with
//! the macro's name (`w_ith<`) when it is a macro, and its doc block.

use xetal_base::Diagnostic;
use xetal_doccom::{Doc, doc_above};
use xetal_ir::{Expr, Item, Program};
use xetal_sources::Sources;

use crate::{example, fresh, reject};

/// One definition in the linted file: the macro's name when it is a
/// macro, its value and the doc block above it.
pub(crate) struct Def<'p> {
    pub macro_name: Option<&'p str>,
    pub value: &'p Expr,
    pub doc: Option<Doc>,
}

/// `found` (the other passes' findings for `program`) with the macro
/// library lints' added; an `unused-binding` finding at the place of a
/// `gensym-unused` one, which says more, is dropped.
pub fn merged(found: Vec<Diagnostic>, program: &Program, sources: &Sources) -> Vec<Diagnostic> {
    let mut own = Vec::new();
    for def in defs(program, sources) {
        own.extend(fresh::fresh(program, def.value));
        own.extend(reject::rejects(&def));
        own.extend(example::example(program, &def));
    }
    let said = |d: &Diagnostic| {
        d.code == "unused-binding"
            && own
                .iter()
                .any(|o| o.code == "gensym-unused" && o.span == d.span)
    };
    let mut out: Vec<Diagnostic> = found.into_iter().filter(|d| !said(d)).collect();
    out.extend(own);
    out
}

/// The definitions written in the linted file (its first source), not
/// in what it imports.
fn defs<'p>(program: &'p Program, sources: &Sources) -> Vec<Def<'p>> {
    let text = sources.written_text(0);
    let mut out = Vec::new();
    for item in &program.items {
        let Item::Def { name, value } = item else {
            continue;
        };
        let at = sources.locate(value.span.start);
        if at.index != 0 {
            continue;
        }
        let macro_name = name
            .split_once(':')
            .map(|(_, key)| key)
            .filter(|key| key.ends_with('<'));
        let doc = doc_above(text, at.line);
        out.push(Def {
            macro_name,
            value,
            doc,
        });
    }
    out
}
