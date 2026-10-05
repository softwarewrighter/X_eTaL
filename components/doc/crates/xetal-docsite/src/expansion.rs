//! Macro calls expanded in place: under the line (or the item) holding
//! a call, a collapsible block with the text its macro gave, drawn
//! decorated and linked, and the calls in that text in turn.

use xetal_doc::Expansion;

use crate::context::Ctx;

/// The expansions of the calls written on lines `first..=last` of file
/// `file`.
pub(crate) fn on_lines(cx: &Ctx, file: usize, first: usize, last: usize) -> String {
    let found = cx.files[file].expansions.iter();
    let here = found.filter(|e| (first..=last).contains(&e.line));
    here.map(|e| expansion(cx, file, e)).collect()
}

/// One call's expansion, collapsed: the macro (linked to where it is
/// defined), then its text and the expansions in it.
fn expansion(cx: &Ctx, file: usize, e: &Expansion) -> String {
    let nested: String = e.nested.iter().map(|n| expansion(cx, file, n)).collect();
    format!(
        "<details class=\"expansion\"><summary><code>{}</code> expands to</summary>\
         <pre class=\"code\">{}</pre>{nested}</details>\n",
        cx.draw(file, &e.name),
        cx.draw(file, &e.text)
    )
}
