//! Loading a program.

use xetal_base::Diagnostic;
use xetal_core::Program;
use xetal_macro::{FsLibraries, Libraries, MacroError, expand};
use xetal_sources::Sources;

use crate::run::Running;

/// A program with its libraries, in Core, and its source map.
#[derive(Debug)]
pub struct Loaded {
    pub sources: Sources,
    pub program: Program,
}

/// Load the program `text`, reported as `name` (a file path, or `-e`
/// for text given on the command line; libraries are looked for
/// beside it).
pub fn load(name: &str, text: &str) -> Result<Loaded, Diagnostic> {
    load_with(name, text, &FsLibraries::from_env())
}

/// [`load`], its libraries found by `libs` (the live demo's come from
/// the browser's storage and the standard libraries).
pub fn load_with(name: &str, text: &str, libs: &dyn Libraries) -> Result<Loaded, Diagnostic> {
    lowered(expand(name, text, &Running(libs)))
}

/// The expanded program lowered to Core; errors located.
pub(crate) fn lowered(expanded: Result<Sources, Box<MacroError>>) -> Result<Loaded, Diagnostic> {
    let sources = expanded.map_err(|e| {
        let d = e.diagnostic.clone().placed_in(&e.file, &e.text);
        match e.main {
            true => d,
            false => Diagnostic { span: None, ..d },
        }
    })?;
    let program = xetal_core::lower(sources.combined()).map_err(|d| located(&sources, d))?;
    Ok(Loaded { sources, program })
}

/// `d` located in the file it came from, with its file, line and
/// column (Saga 27): for one file, its span is also moved from the
/// expanded text to the text as written; otherwise it has no span (a
/// combined offset means nothing to a reader), and hidden namespaces
/// in its message are written as that file writes them.
pub fn located(sources: &Sources, d: Diagnostic) -> Diagnostic {
    if sources.file_count() < 2 || d.span.is_none() {
        return in_program(sources, d);
    }
    let file = d.span.map_or(0, |s| sources.locate(s.start).index);
    let message = sources.as_written(file, &d.message);
    let d = labels_written(sources, d).place_with(|span| sources.place(span));
    Diagnostic {
        message,
        span: None,
        ..d
    }
}

/// `d` for tools that show the program's own text (the REPL, the
/// editor): an error in the program keeps a span in the program file
/// (the combined text starts with the libraries); one in a library is
/// located there, as [`located`] does. Either way it is placed.
pub fn in_program(sources: &Sources, d: Diagnostic) -> Diagnostic {
    let d = labels_written(sources, d);
    let Some(span) = d.span else {
        return d.place_with(|span| sources.place(span));
    };
    let (start, end) = (
        sources.locate(span.start),
        sources.locate(span.end.max(span.start + 1) - 1),
    );
    match (start.index, end.index) {
        (0, 0) => {
            let mut out = d.place_with(|span| sources.place(span));
            out.span = Some(xetal_base::Span::new(
                start.offset,
                end.to.max(start.offset + 1),
            ));
            out
        }
        _ => located(sources, d),
    }
}

/// `d` with each label's words naming hidden namespaces as the file
/// the label points into writes them.
fn labels_written(sources: &Sources, mut d: Diagnostic) -> Diagnostic {
    for label in d.labels_mut() {
        let file = sources.locate(label.span.start).index;
        label.text = sources.as_written(file, &label.text);
    }
    d
}
