//! The string arguments of the macro calls written in a file, each with
//! what its macro's contract says about its side.

use xetal_base::Span;
use xetal_lex::lex;
use xetal_sources::{Site, Sources};

/// One string argument of a call written in the program's file.
#[derive(Debug, Clone)]
pub(crate) struct Arg {
    /// The macro as written (`x:s_tage<`), and the side (`true`, right).
    pub name: String,
    pub right: bool,
    /// The string literal, as written in the file.
    pub span: Span,
    /// It is a code string (`c"..."`, `c#"..."#`).
    pub code: bool,
    /// The contract has a `takes:` line; it says code on this side.
    pub takes: bool,
    pub takes_code: bool,
    /// The names the macro binds for its text; it parses its text.
    pub binds: Vec<String>,
    pub parsed: bool,
}

/// The string arguments of the calls in `text` that were expanded (a
/// call recorded among `sources`' sites, in the program's file).
pub(crate) fn args(text: &str, sources: &Sources) -> Vec<Arg> {
    let Ok(tokens) = lex(text) else {
        return Vec::new();
    };
    let Ok(calls) = xetal_expand::calls(&tokens) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for call in calls {
        let at = call.span.start..call.span.end;
        let mine = |s: &&Site| s.file == 0 && s.span == at && s.name == call.name;
        let Some(site) = sources.sites().iter().find(mine) else {
            continue;
        };
        let sides = [
            (call.texts.0, call.codes.0, call.left, false),
            (call.texts.1, call.codes.1, call.right, true),
        ];
        for (text, code, span, right) in sides.into_iter().filter(|s| s.0) {
            let side = |p: (bool, bool)| if right { p.1 } else { p.0 };
            out.push(Arg {
                name: call.name.clone(),
                right,
                span,
                code,
                takes: site.takes.is_some_and(side) && text,
                takes_code: side(site.code),
                binds: site.binds.clone(),
                parsed: site.parsed,
            });
        }
    }
    out
}
