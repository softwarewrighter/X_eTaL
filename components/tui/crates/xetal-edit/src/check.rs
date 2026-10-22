//! What the bottom pane shows: types or the first diagnostic (live),
//! a run's output (Ctrl-R only, so effects never run on a keystroke),
//! each value laid out as a grid (`xetal-grid`), or the lint report
//! (Ctrl-L).

use xetal_base::Diagnostic;
use xetal_macro::Libraries;

use crate::Editor;
use xetal_panes::Notation;

/// Which the bottom pane shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Bottom {
    Types,
    Output,
    Lint,
}

/// Lines to show, the span of an error to highlight, and the notation
/// calls the rendered pane styles as notation (MC32).
#[derive(Debug, Clone, Default)]
pub(crate) struct Report {
    pub lines: Vec<String>,
    pub mark: Option<(usize, usize)>,
    pub notations: Vec<Notation>,
}

/// The notation calls of the program loaded (its own file), as the
/// panes take them.
fn notations(loaded: &xetal_program::Loaded) -> Vec<Notation> {
    xetal_notate::of(&loaded.sources)
}

fn failed(d: Diagnostic) -> Report {
    let mark = d.span.map(|s| (s.start, s.end));
    Report {
        lines: d.to_string().lines().map(String::from).collect(),
        mark,
        notations: Vec::new(),
    }
}

/// The type of each top-level item (libraries loaded, found from
/// `origin`), or the error.
pub(crate) fn check(src: &str, origin: &str, libs: &dyn Libraries) -> Report {
    let mut loaded = match xetal_program::load_with(origin, src, libs) {
        Ok(l) => l,
        Err(d) => return failed(d),
    };
    let notations = notations(&loaded);
    match xetal_types::check_program(&mut loaded.program) {
        Ok(lines) => Report {
            lines: xetal_program::program_types(lines),
            mark: None,
            notations,
        },
        Err(d) => Report {
            notations,
            ..failed(xetal_program::in_program(&loaded.sources, d))
        },
    }
}

/// The lint findings of `src` as `xetal lint` reports them (no mark:
/// each line names its place), and the text with their fixes made when
/// they carry any (Ctrl-L again makes them).
pub(crate) fn lint(src: &str, origin: &str, libs: &dyn Libraries) -> (Report, Option<String>) {
    let r = xetal_report::report_with(origin, src, libs);
    let report = Report {
        lines: r.lines.into_iter().map(|l| l.text).collect(),
        mark: None,
        notations: Vec::new(),
    };
    (report, r.fixed)
}

/// Printed text as its lines; a value laid out as a grid (type and
/// shape, matrices boxed, higher ranks as slices).
fn shown(event: &xetal_eval::Event) -> Vec<String> {
    match event {
        xetal_eval::Event::Printed(text) => text.lines().map(String::from).collect(),
        xetal_eval::Event::Value(grid) => grid.lines(),
    }
}

/// Type-check and run, collecting what the program shows.
/// `seed` is what `r_oll!` draws from; none takes a fresh one (not in
/// the browser, which has no clock of its own: the host gives one).
pub(crate) fn run(src: &str, origin: &str, libs: &dyn Libraries, seed: Option<u64>) -> Report {
    let loaded = match xetal_program::load_with(origin, src, libs) {
        Ok(l) => l,
        Err(d) => return failed(d),
    };
    let notations = notations(&loaded);
    let xetal_program::Loaded {
        sources,
        mut program,
    } = loaded;
    let here = |d| xetal_program::in_program(&sources, d);
    if let Err(d) = xetal_types::check_program(&mut program) {
        return Report {
            notations,
            ..failed(here(d))
        };
    }
    let (_, events, result) = xetal_eval::eval_events(&program, seed);
    let mut report = Report {
        lines: events.iter().flat_map(shown).collect(),
        mark: None,
        notations,
    };
    if let Err(d) = result {
        let err = failed(here(d));
        report.lines.extend(err.lines);
        report.mark = err.mark;
    }
    report
}

impl Editor {
    /// Ctrl-L: the lint report below; pressed again while its findings
    /// carry fixes, the fixes are made in the text (unsaved) and the
    /// result is linted. The rendered pane keeps its notations.
    pub(crate) fn lint(&mut self) {
        let src = match (self.bottom, self.fix.take()) {
            (Bottom::Lint, Some(fixed)) => {
                self.buffer.replace(&fixed);
                fixed
            }
            _ => self.buffer.text(),
        };
        let (mut report, fix) = lint(&src, &self.origin(), self.libs.as_ref());
        report.notations = std::mem::take(&mut self.report.notations);
        (self.report, self.fix, self.bottom) = (report, fix, Bottom::Lint);
        self.confirm_quit = false;
    }
}
