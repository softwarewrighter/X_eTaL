//! Code strings and the contracts of the macros they are passed to
//! (ST5, MC31; docs/code-strings.md): `string-kind` where a string's
//! kind is not the kind its side takes, and `code-check` with the
//! ordinary lints on the inside of a code string a macro takes as
//! code, checked on its own before expansion. Every finding is placed
//! in the program's file as written.

mod args;
mod inside;
mod kind;
mod probe;

use xetal_base::Diagnostic;
use xetal_sources::Sources;

/// The findings for program `text` (named `name`), whose calls were
/// recorded in `sources` when it loaded: spans and fixes are offsets
/// in `text`. A library's code strings are not checked on their own
/// (its calls stand in definitions), only their kind.
pub fn checks(name: &str, text: &str, sources: &Sources, library: bool) -> Vec<Diagnostic> {
    let args = args::args(text, sources);
    let mut out = kind::string_kind(text, &args);
    if !library {
        let probes = args.iter().filter_map(|a| probe::probe(text, a)).collect();
        out.extend(inside::checked(name, text, probes));
    }
    out
}
