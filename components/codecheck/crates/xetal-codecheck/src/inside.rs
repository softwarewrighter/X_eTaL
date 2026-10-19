//! Checking the probes with the program: the program's file with the
//! probes after it is loaded and type-checked; an error inside a probe
//! is a `code-check` finding there, and that probe is set aside for the
//! next round; once all load and check, the ordinary lints' findings
//! inside the probes of macros that parse their text are kept.

use xetal_base::{Diagnostic, Span};
use xetal_macro::FsLibraries;
use xetal_program::Running;
use xetal_sources::Sources;

use crate::probe::Probe;

/// What one round found: an error at a place in the file with the
/// probes (or none to place), or the lints' findings with their places.
enum Round {
    Error(Diagnostic, Option<usize>),
    Lints(Vec<(Diagnostic, Span)>),
}

/// The findings for `probes` of program `text` (named `name`), placed
/// in `text`.
pub(crate) fn checked(name: &str, text: &str, mut probes: Vec<Probe>) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    while !probes.is_empty() {
        let (whole, starts) = with_probes(text, &probes);
        match round(name, &whole) {
            Round::Error(d, Some(at)) => {
                // In a probe, or in the program inside a code string
                // being checked (its expansion's error, placed there).
                let (k, span) = match (within(&probes, &starts, at), written(&probes, at)) {
                    (Some(k), _) => (k, placed(&probes[k], starts[k], Span::new(at, at + 1))),
                    (None, Some(k)) => (k, Span::new(at, at + 1)),
                    (None, None) => break,
                };
                probes.remove(k);
                let message = format!(
                    "{} (error[{}], this code checked before the program runs)",
                    d.message, d.code
                );
                out.push(Diagnostic::new("code-check", message).with_span(span));
            }
            Round::Error(_, None) => break,
            Round::Lints(found) => {
                out.extend(linted(found, &probes, &starts));
                break;
            }
        }
    }
    out
}

/// The probe whose inside holds offset `at`.
fn within(probes: &[Probe], starts: &[usize], at: usize) -> Option<usize> {
    (0..probes.len()).find(|&k| (starts[k]..=starts[k] + probes[k].len).contains(&at))
}

/// The probe whose code string, as written, holds offset `at`.
fn written(probes: &[Probe], at: usize) -> Option<usize> {
    (0..probes.len())
        .find(|&k| (probes[k].at..probes[k].at + probes[k].map[probes[k].len]).contains(&at))
}

/// `text` with the probes on lines after it, and where each inside starts.
fn with_probes(text: &str, probes: &[Probe]) -> (String, Vec<usize>) {
    let mut whole = text.to_string();
    let mut starts = Vec::new();
    for p in probes {
        whole.push('\n');
        starts.push(whole.len() + p.body);
        whole.push_str(&p.line);
    }
    whole.push('\n');
    (whole, starts)
}

/// Load and check `whole`: its first error placed in it, or the lints.
fn round(name: &str, whole: &str) -> Round {
    let libs = FsLibraries::from_env();
    let sources = match xetal_macro::expand(name, whole, &Running(&libs)) {
        Ok(sources) => sources,
        Err(e) => {
            let at = e.main.then(|| e.diagnostic.span.map(|s| s.start)).flatten();
            return Round::Error(e.diagnostic, at);
        }
    };
    let at = |sources: &Sources, d: &Diagnostic| {
        let place = sources.locate(d.span?.start);
        (place.index == 0).then_some(place.offset)
    };
    let mut program = match xetal_core::lower(sources.combined()) {
        Ok(program) => program,
        Err(d) => return Round::Error(d.clone(), at(&sources, &d)),
    };
    if let Err(d) = xetal_types::check_program(&mut program) {
        return Round::Error(d.clone(), at(&sources, &d));
    }
    let placed = |d: Diagnostic| {
        let span = d.span?;
        let (start, end) = (
            sources.locate(span.start),
            sources.locate(span.end.max(span.start + 1) - 1),
        );
        (start.index == 0 && end.index == 0).then(|| (d, Span::new(start.offset, end.offset + 1)))
    };
    Round::Lints(
        xetal_lint::lints(&program)
            .into_iter()
            .filter_map(placed)
            .collect(),
    )
}

/// The lints' findings inside the probes of macros that parse their
/// text (a copying macro's are found on its expansion), placed.
fn linted(found: Vec<(Diagnostic, Span)>, probes: &[Probe], starts: &[usize]) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for (mut d, span) in found {
        let k = within(probes, starts, span.start);
        if let Some(k) = k.filter(|&k| probes[k].parsed && span.end <= starts[k] + probes[k].len) {
            d.span = Some(placed(&probes[k], starts[k], span));
            out.push(Diagnostic { located: None, ..d });
        }
    }
    out
}

/// `span`, in a probe whose inside starts at `start`, placed in the
/// file as written.
fn placed(p: &Probe, start: usize, span: Span) -> Span {
    let at = |o: usize| p.at + p.map[o.saturating_sub(start).min(p.len)];
    Span::new(at(span.start), at(span.end).max(at(span.start) + 1))
}
