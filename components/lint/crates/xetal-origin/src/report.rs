//! Origin-aware reporting: a finding in code a macro wrote is placed at
//! the call, where the caller cannot change it, so it is reported under
//! a lint of its own instead of the one that found it.

use xetal_base::Diagnostic;
use xetal_sources::Sources;

use crate::site::written_by;

/// `d` as the caller should see it: unchanged when it starts in text
/// the caller wrote; in code the macro wrote, `unused-anaphor` for an
/// unread name the macro declares it binds for the call's text, else
/// `macro-internals` (both off by default), without a fix.
pub fn placed(d: Diagnostic, sources: &Sources) -> Diagnostic {
    let Some((site, _)) = d.span.and_then(|s| written_by(sources, s)) else {
        return d;
    };
    let name = d.span.and_then(|s| sources.combined().get(s.start..s.end));
    let anaphor = name.filter(|n| d.code == "unused-binding" && site.binds.iter().any(|b| b == n));
    let (code, message) = match anaphor {
        Some(name) => (
            "unused-anaphor",
            format!(
                "{} binds `{name}` for your text, which never reads it",
                site.name
            ),
        ),
        None => (
            "macro-internals",
            format!("in code {} wrote: {}: {}", site.name, d.code, d.message),
        ),
    };
    let mut out = Diagnostic::warning(code, message);
    out.span = d.span;
    out
}
