//! A session keeps the source it has accepted. Each new line is checked
//! and run together with that source (so definitions, types and `!`
//! variables persist), and only the output past what was already shown
//! is returned; a line that fails is not accepted, so its effects roll
//! back. An unclosed bracket waits for more lines.

use std::fmt::Write;

use xetal_base::{Diagnostic, Span};

#[derive(Debug, Default)]
pub struct Session {
    accepted: String,
    pending: String,
    shown: usize,
    warned: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Reply {
    /// The input is incomplete (an unclosed bracket); send more lines.
    More,
    /// New output, and new warnings or an error.
    Done { out: String, err: String },
}

/// Output, warnings and the outcome of running `source` from the start.
fn run(source: &str) -> (Vec<u8>, Vec<Diagnostic>, Result<(), Diagnostic>) {
    let mut program = match xetal_core::lower(source) {
        Ok(p) => p,
        Err(e) => return (Vec::new(), Vec::new(), Err(e)),
    };
    if let Err(e) = xetal_types::check_program(&mut program) {
        return (Vec::new(), Vec::new(), Err(e));
    }
    let mut out = Vec::new();
    let (warnings, result) = xetal_eval::eval_program(&program, &mut out);
    (out, warnings, result)
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
        let (out, warnings, result) = run(&source);
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
