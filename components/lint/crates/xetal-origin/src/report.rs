//! Origin-aware reporting: a finding in code a macro wrote is placed at
//! the call, where the caller cannot change it, so it is reported under
//! a lint of its own instead of the one that found it.

use xetal_base::Diagnostic;
use xetal_sources::{Site, Sources};

use crate::site::{parsed_from, written_by};

/// `d` as the caller should see it: unchanged when it starts in text
/// the caller wrote; in code the macro wrote, `unused-anaphor` for an
/// unread name the macro declares it binds for the call's text, else
/// `macro-internals` (both off by default), without a fix. A parsing
/// macro's whole expansion is its own (`## uses: parsed`): what it
/// repeats of the call's text is notation read, not code copied.
pub fn placed(d: Diagnostic, sources: &Sources) -> Diagnostic {
    let Some(span) = d.span else {
        return d;
    };
    let (code, message) = match (written_by(sources, span), parsed_from(sources, span)) {
        (Some((site, _)), _) => match anaphor(&d, &site, sources) {
            Some(name) => (
                "unused-anaphor",
                format!(
                    "{} binds `{name}` for your text, which never reads it",
                    site.name
                ),
            ),
            None => internal(&d, &site),
        },
        (None, Some(site)) => internal(&d, &site),
        (None, None) => return d,
    };
    let mut out = Diagnostic::warning(code, message);
    out.span = d.span;
    out
}

/// The name of an unread binding in `d` that `site` declares it binds.
fn anaphor<'s>(d: &Diagnostic, site: &Site, sources: &'s Sources) -> Option<&'s str> {
    let name = d.span.and_then(|s| sources.combined().get(s.start..s.end));
    name.filter(|n| d.code == "unused-binding" && site.binds.iter().any(|b| b == n))
}

/// `d` as a `macro-internals` finding of the macro `site`.
fn internal(d: &Diagnostic, site: &Site) -> (&'static str, String) {
    let message = format!("in code {} wrote: {}: {}", site.name, d.code, d.message);
    ("macro-internals", message)
}
