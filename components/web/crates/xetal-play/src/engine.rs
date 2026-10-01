//! Checking and running a program as the editor's panes show it.

use xetal_base::Diagnostic;
use xetal_macro::StoreLibraries;
use xetal_program::{
    Loaded, in_program, is_library, library_types, load_library_with, load_with, program_types,
};

/// What a run printed, its warnings and error (one per line), and the
/// pictures it showed (SVG documents, `[]S_HOW`), in order.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Run {
    pub out: String,
    pub err: String,
    pub pictures: Vec<String>,
}

/// The name the program is reported by.
const NAME: &str = "main.xtl";

fn loaded(src: &str) -> Result<Loaded, Diagnostic> {
    load_with(NAME, src, &StoreLibraries)
}

/// The type of each top-level item, or the first error; for a library
/// (a file naming `l:`), each export's type, as `xetal type` gives.
pub fn check(src: &str) -> Vec<String> {
    let library = is_library(src);
    let loaded = match library {
        true => load_library_with(NAME, src, &StoreLibraries),
        false => loaded(src),
    };
    let checked = loaded.and_then(|mut l| {
        let lines = xetal_types::check_program(&mut l.program);
        let lines = lines.map_err(|d| in_program(&l.sources, d))?;
        Ok(match library {
            true => library_types(&l.sources, lines),
            false => program_types(lines),
        })
    });
    checked.unwrap_or_else(|d| vec![d.to_string()])
}

/// Check and run `src`, rolling from `seed`; a library has nothing to
/// run, so its exports' types are its output.
pub fn run(src: &str, seed: u64) -> Run {
    if is_library(src) {
        let lines = check(src);
        return Run {
            out: lines.iter().map(|l| format!("{l}\n")).collect(),
            ..Run::default()
        };
    }
    let mut loaded = match loaded(src) {
        Ok(l) => l,
        Err(d) => return failed(d),
    };
    if let Err(d) = xetal_types::check_program(&mut loaded.program) {
        return failed(in_program(&loaded.sources, d));
    }
    let mut out = Vec::new();
    xetal_store::take_shown();
    let (warnings, result) = xetal_eval::eval_program(&loaded.program, &mut out, Some(seed));
    let mut err: Vec<String> = warnings.into_iter().map(|w| w.to_string()).collect();
    err.extend(
        result
            .err()
            .map(|e| in_program(&loaded.sources, e).to_string()),
    );
    Run {
        out: String::from_utf8_lossy(&out).into_owned(),
        err: err.iter().map(|l| format!("{l}\n")).collect(),
        pictures: xetal_store::take_shown(),
    }
}

fn failed(d: Diagnostic) -> Run {
    Run {
        err: format!("{d}\n"),
        ..Run::default()
    }
}
