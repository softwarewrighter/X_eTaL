//! A session keeps the source it has accepted. Each new line is checked
//! and run together with that source (so definitions, types and `!`
//! variables persist), and only the output past what was already shown
//! is returned (and pictures shown before are not shown again); a line
//! that fails is not accepted, so its effects roll back. An unclosed
//! bracket waits for more lines.

use std::fmt::Write;

use xetal_base::{Diagnostic, Place, Span};

use crate::live::Live;

/// Every replay rolls from the session's seed, so `r_oll!` results of
/// accepted lines keep their values from line to line.
#[derive(Debug)]
pub struct Session {
    accepted: String,
    pending: String,
    shown: usize,
    warned: usize,
    /// Pictures (`[]S_HOW`) the accepted source showed; a replay skips them.
    pictures: usize,
    seed: u64,
    /// Where libraries are found from: a file's path, or `-e` for the
    /// working directory.
    origin: String,
    /// The name its places are reported by: the file's (a notebook),
    /// or `repl` for one typed in; and how many lines were fed before,
    /// so a place's line is counted from the first line fed.
    name: String,
    fed: usize,
    /// The line fed that each line of the accepted source was.
    numbers: Vec<usize>,
    /// Run without the type checker (`--untyped`).
    pub untyped: bool,
}

impl Default for Session {
    /// A session with an unpredictable seed.
    fn default() -> Self {
        Session {
            name: "repl".into(),
            ..Session::new("-e", xetal_eval::Rng::fresh_seed())
        }
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
    session: &Session,
    sink: &mut (dyn std::io::Write + Send),
) -> (Vec<u8>, Vec<Diagnostic>, Result<(), Diagnostic>) {
    let (origin, seed, untyped) = (&session.origin, session.seed, session.untyped);
    // A session may unbind a name with []E_X (M4); a file never can.
    xetal_core::interactive(true);
    let loaded = xetal_program::load(origin, source);
    xetal_core::interactive(false);
    let loaded = match loaded {
        Ok(l) => l,
        Err(e) => return (Vec::new(), Vec::new(), Err(e)),
    };
    let (sources, mut program) = (loaded.sources, loaded.program);
    let here = |d| xetal_program::in_program(&sources, d);
    let checked = match untyped {
        true => Ok(Vec::new()),
        false => xetal_types::check_program(&mut program),
    };
    if let Err(e) = checked {
        return (Vec::new(), Vec::new(), Err(here(e)));
    }
    let mut out = Live {
        skip: session.shown,
        all: Vec::new(),
        sink,
    };
    xetal_store::replay(session.pictures);
    let (warnings, result) = xetal_eval::eval_program(&program, &mut out, Some(seed));
    // Those a `# lint:` comment allows are not shown (S12).
    let warnings = xetal_linter::honored(&sources, &program, warnings);
    (
        out.all,
        warnings.into_iter().map(here).collect(),
        result.map_err(here),
    )
}

/// A diagnostic with its span made relative to the new input `text`,
/// and placed there: in file `name`, after `before` lines fed. A label
/// in an earlier input is placed in the accepted `source`, whose lines
/// were the lines fed `numbers`.
fn shifted(
    mut d: Diagnostic,
    offset: usize,
    (name, before, text): (&str, usize, &str),
    (source, numbers): (&str, &[usize]),
) -> Diagnostic {
    let inside = |s: Span| (s.start >= offset).then(|| Span::new(s.start - offset, s.end - offset));
    let at = |span| {
        let place = Place::of(name, text, span);
        Place {
            line: place.line + before,
            ..place
        }
    };
    if let Some(span) = d.span.and_then(inside) {
        d.span = Some(span);
        d.set_place(Some(at(span)));
    }
    let earlier = |span| {
        let place = Place::of(name, source, span);
        let line = numbers.get(place.line - 1).copied().unwrap_or(place.line);
        Place { line, ..place }
    };
    for label in d.labels_mut() {
        label.place = Some(match inside(label.span) {
            Some(span) => at(span),
            None => earlier(label.span),
        });
    }
    d
}

impl Session {
    /// A session whose libraries are found from `origin` (a file's
    /// path, or `-e` for the working directory) and whose rolls come
    /// from `seed`.
    pub fn new(origin: &str, seed: u64) -> Self {
        xetal_store::forget_lines();
        Session {
            accepted: String::new(),
            pending: String::new(),
            shown: 0,
            warned: 0,
            pictures: 0,
            seed,
            origin: origin.into(),
            name: origin.into(),
            fed: 0,
            numbers: Vec::new(),
            untyped: false,
        }
    }

    /// Feed one line of input.
    pub fn feed(&mut self, line: &str) -> Reply {
        self.feed_to(line, &mut std::io::sink())
    }

    /// Feed one line of input, its new output also written to `sink`
    /// as the program writes it (to watch a long computation).
    pub fn feed_to(&mut self, line: &str, sink: &mut (dyn std::io::Write + Send)) -> Reply {
        let text = match self.pending.is_empty() {
            true => line.to_string(),
            false => format!("{}\n{line}", self.pending),
        };
        let source = match self.accepted.is_empty() {
            true => text.clone(),
            false => format!("{}\n{text}", self.accepted),
        };
        let offset = source.len() - text.len();
        self.fed += line.split('\n').count();
        let at = (
            self.name.as_str(),
            self.fed - text.split('\n').count(),
            text.as_str(),
        );
        let (out, warnings, result) = run(&source, self, sink);
        let earlier = (source.as_str(), self.numbers.as_slice());
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
                    let _ = writeln!(err, "{}", shifted(w.clone(), offset, at, earlier));
                }
                let lines = at.1 + 1..=at.1 + text.split('\n').count();
                self.numbers.extend(lines);
                (self.accepted, self.shown, self.warned) = (source, out.len(), warnings.len());
                self.pictures = xetal_store::shown();
            }
            Err(e) => {
                let _ = writeln!(err, "{}", shifted(e, offset, at, earlier));
            }
        }
        Reply::Done { out: new_out, err }
    }
}
