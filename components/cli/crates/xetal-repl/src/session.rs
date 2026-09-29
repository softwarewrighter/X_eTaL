//! A session keeps the source it has accepted. Each new line is checked
//! and run together with that source (so definitions, types and `!`
//! variables persist), and only the output past what was already shown
//! is returned; a line that fails is not accepted, so its effects roll
//! back. An unclosed bracket waits for more lines.

use std::fmt::Write;

use xetal_base::{Diagnostic, Span};

/// Every replay rolls from the session's seed, so `r_oll!` results of
/// accepted lines keep their values from line to line.
#[derive(Debug)]
pub struct Session {
    accepted: String,
    pending: String,
    shown: usize,
    warned: usize,
    seed: u64,
    /// Where libraries are found from: a file's path, or `-e` for the
    /// working directory.
    origin: String,
}

impl Default for Session {
    /// A session with an unpredictable seed.
    fn default() -> Self {
        Session::new("-e", xetal_eval::Rng::fresh_seed())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Reply {
    /// The input is incomplete (an unclosed bracket); send more lines.
    More,
    /// New output, and new warnings or an error.
    Done { out: String, err: String },
}

/// Output, warnings and the outcome of running `source` from the start,
/// its libraries (found relative to `origin`) loaded first; errors are
/// placed in the program's own text.
fn run(
    source: &str,
    origin: &str,
    seed: u64,
) -> (Vec<u8>, Vec<Diagnostic>, Result<(), Diagnostic>) {
    let loaded = match xetal_program::load(origin, source) {
        Ok(l) => l,
        Err(e) => return (Vec::new(), Vec::new(), Err(e)),
    };
    let (sources, mut program) = (loaded.sources, loaded.program);
    let here = |d| xetal_program::in_program(&sources, d);
    if let Err(e) = xetal_types::check_program(&mut program) {
        return (Vec::new(), Vec::new(), Err(here(e)));
    }
    let mut out = Vec::new();
    let (warnings, result) = xetal_eval::eval_program(&program, &mut out, Some(seed));
    (
        out,
        warnings.into_iter().map(here).collect(),
        result.map_err(here),
    )
}

/// A diagnostic with its span made relative to the new input.
fn shifted(mut d: Diagnostic, offset: usize) -> Diagnostic {
    d.span = d.span.map(|s| match s.start >= offset {
        true => Span::new(s.start - offset, s.end - offset),
        false => s,
    });
    d
}

impl Session {
    /// A session whose libraries are found from `origin` (a file's
    /// path, or `-e` for the working directory) and whose rolls come
    /// from `seed`.
    pub fn new(origin: &str, seed: u64) -> Self {
        Session {
            accepted: String::new(),
            pending: String::new(),
            shown: 0,
            warned: 0,
            seed,
            origin: origin.into(),
        }
    }

    /// Feed one line of input.
    pub fn feed(&mut self, line: &str) -> Reply {
        let text = match self.pending.is_empty() {
            true => line.to_string(),
            false => format!("{}\n{line}", self.pending),
        };
        let source = match self.accepted.is_empty() {
            true => text.clone(),
            false => format!("{}\n{text}", self.accepted),
        };
        let offset = source.len() - text.len();
        let (out, warnings, result) = run(&source, &self.origin, self.seed);
        if matches!(&result, Err(d) if d.code == "unclosed") {
            self.pending = text;
            return Reply::More;
        }
        self.pending.clear();
        let new_out =
            String::from_utf8_lossy(out.get(self.shown..).unwrap_or_default()).into_owned();
        let mut err = String::new();
        match result {
            Ok(()) => {
                for w in &warnings[self.warned.min(warnings.len())..] {
                    let _ = writeln!(err, "{}", shifted(w.clone(), offset));
                }
                (self.accepted, self.shown, self.warned) = (source, out.len(), warnings.len());
            }
            Err(e) => {
                let _ = writeln!(err, "{}", shifted(e, offset));
            }
        }
        Reply::Done { out: new_out, err }
    }
}
