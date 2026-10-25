//! A notebook in one pass: the program runs once, and the evaluator
//! says when each top-level statement starts, so its source is shown
//! then and its output streams under it; nothing runs twice. A file
//! with warnings or type errors, and the rest of a file after a runtime
//! error, go a statement at a time instead (`live`).

use std::io::Write;

use xetal_base::{Diagnostic, Span};
use xetal_repl::Session;

use crate::live::{Indent, RED, RESET, stream, unclosed};

/// What draws a statement before it runs, onto the output.
pub(crate) type Show<'s> = &'s mut (dyn FnMut(&str, &mut (dyn Write + Send)) + Send);

/// Lines `from..to` of `lines` shown as statements, grouped as the
/// statement-at-a-time notebook groups them; where the last began.
fn cells(
    lines: &[&str],
    from: usize,
    to: usize,
    show: &mut dyn FnMut(&str, &mut (dyn Write + Send)),
    out: &mut (dyn Write + Send),
) -> usize {
    let (mut start, mut last) = (from, from);
    for end in from + 1..=to {
        let text = lines[start..end].join("\n");
        if !unclosed(&text) {
            show(&text, out);
            (last, start) = (start, end);
        }
    }
    last
}

/// The notebook in one pass; `None` when the file must go a statement
/// at a time (it does not check, or has warnings), else whether a
/// statement failed.
pub(crate) fn once(
    origin: &str,
    source: &str,
    run: (u64, bool),
    show: Show,
    out: &mut (dyn Write + Send),
) -> Option<bool> {
    let (seed, untyped) = run;
    let mut loaded = xetal_program::load(origin, source).ok()?;
    if !untyped && xetal_types::check_program(&mut loaded.program).is_err() {
        return None;
    }
    let warnings = xetal_lint::warnings(&loaded.program);
    if !xetal_linter::honored(&loaded.sources, &loaded.program, warnings).is_empty() {
        return None;
    }
    let lines: Vec<&str> = source.lines().collect();
    let (mut next, mut cell) = (0, 0);
    // The evaluator writes to the same output the statements are drawn
    // on; it hands the sink back between statements through `before`.
    let sink = std::sync::Mutex::new(&mut *out);
    let mut indent = Through { sink: &sink };
    let mut before = |span: Span| {
        let at = loaded.sources.locate(span.end.max(span.start + 1) - 1);
        let line = source[..at.offset.min(source.len())].matches('\n').count();
        if at.index == 0 && line >= next {
            let mut out = sink.lock().unwrap_or_else(|e| e.into_inner());
            cell = cells(&lines, next, line + 1, &mut *show, &mut **out);
            next = line + 1;
        }
    };
    let (_, result) = xetal_eval::eval_items(&loaded.program, &mut indent, Some(seed), &mut before);
    let out = sink.into_inner().unwrap_or_else(|e| e.into_inner());
    Some(match result {
        Ok(()) => {
            cells(&lines, next, lines.len(), show, out);
            false
        }
        Err(e) => after_error(
            &loaded.sources,
            e,
            (origin, source),
            (cell, next),
            run,
            (show, out),
        ),
    })
}

/// The output shared between the evaluator and the statements drawn
/// between its writes.
struct Through<'o, 's> {
    sink: &'s std::sync::Mutex<&'o mut (dyn Write + Send)>,
}

impl Write for Through<'_, '_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut out = self.sink.lock().unwrap_or_else(|e| e.into_inner());
        let mut indent = Indent {
            at_start: true,
            out: &mut **out,
        };
        indent.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut out = self.sink.lock().unwrap_or_else(|e| e.into_inner());
        out.flush()
    }
}

/// A runtime error: shown under its statement as the statement-at-a-
/// time notebook shows it; the statement is dropped and the rest of
/// the file goes on a statement at a time, after the statements before
/// it (run once more, silently).
fn after_error(
    sources: &xetal_sources::Sources,
    e: Diagnostic,
    (origin, source): (&str, &str),
    (cell, next): (usize, usize),
    (seed, untyped): (u64, bool),
    (show, out): (Show, &mut (dyn Write + Send)),
) -> bool {
    let lines: Vec<&str> = source.lines().collect();
    let offset: usize = lines[..cell].iter().map(|l| l.len() + 1).sum();
    let mut d = xetal_program::in_program(sources, e);
    d.span = d
        .span
        .map(|s| Span::new(s.start.saturating_sub(offset), s.end.saturating_sub(offset)));
    for line in d.to_string().lines() {
        let _ = writeln!(out, "{RED}{line}{RESET}");
    }
    let mut session = Session::new(origin, seed);
    session.untyped = untyped;
    if cell > 0 {
        session.feed_to(&lines[..cell].join("\n"), &mut std::io::sink());
    }
    stream(&mut session, &lines[next..].join("\n"), show, out);
    true
}
