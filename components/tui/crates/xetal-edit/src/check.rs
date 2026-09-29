//! What the bottom pane shows: types or the first diagnostic (live),
//! or a run's output (Ctrl-R only, so effects never run on a keystroke).

use xetal_base::Diagnostic;

/// Lines to show, and the span of an error to highlight.
#[derive(Debug, Clone, Default)]
pub(crate) struct Report {
    pub lines: Vec<String>,
    pub mark: Option<(usize, usize)>,
}

fn failed(d: Diagnostic) -> Report {
    let mark = d.span.map(|s| (s.start, s.end));
    Report {
        lines: vec![d.to_string()],
        mark,
    }
}

/// The type of each top-level item, or the error.
pub(crate) fn check(src: &str) -> Report {
    match xetal_types::check_source(src) {
        Ok(lines) => Report { lines, mark: None },
        Err(d) => failed(d),
    }
}

/// Type-check and run, collecting what the program prints.
pub(crate) fn run(src: &str) -> Report {
    let mut program = match xetal_core::lower(src) {
        Ok(p) => p,
        Err(d) => return failed(d),
    };
    if let Err(d) = xetal_types::check_program(&mut program) {
        return failed(d);
    }
    let mut out = Vec::new();
    let (_, result) = xetal_eval::eval_program(&program, &mut out, None);
    let mut report = Report {
        lines: String::from_utf8_lossy(&out)
            .lines()
            .map(String::from)
            .collect(),
        mark: None,
    };
    if let Err(d) = result {
        let err = failed(d);
        report.lines.extend(err.lines);
        report.mark = err.mark;
    }
    report
}
