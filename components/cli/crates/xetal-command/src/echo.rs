//! `xetal run --echo`: each statement pretty-printed (decorated and
//! colored), then its output below it, like a notebook laid out as an
//! APL session: statements indented six spaces, output flush left. And
//! `xetal run --context FILE BLOCK`: BLOCK run after the program in
//! FILE, as one session, showing only BLOCK's output (org-babel
//! sessions: a block sees what the blocks before it defined).

use std::io::Write;

use xetal_base::Diagnostic;

use crate::args::{Command, EvalArgs};
use crate::host::Host;
use crate::stages::{evaluate, seed_or_env};
use xetal_view::{ansi, view};

/// Print `source` as a notebook; an error fails the run after
/// everything has been shown.
pub(crate) fn echo(
    source: &str,
    origin: &str,
    (seed, delay, untyped): (Option<u64>, Option<u64>, bool),
    host: &mut Host,
) -> Result<String, Diagnostic> {
    let seed = seed.unwrap_or_else(xetal_eval::Rng::fresh_seed);
    let mut show = |text: &str, out: &mut (dyn Write + Send)| {
        pause(delay, text, out);
        let shown = ansi(&view(text));
        if text.trim().is_empty() {
            let _ = writeln!(out);
        }
        for line in shown.lines().filter(|_| !text.trim().is_empty()) {
            let _ = writeln!(out, "{}{line}", crate::live::PROMPT);
        }
    };
    let out = &mut *host.out;
    let failed = match crate::once::once(origin, source, (seed, untyped), &mut show, out) {
        Some(failed) => failed,
        None => {
            let mut session = xetal_repl::Session::new(origin, seed);
            session.untyped = untyped;
            crate::live::stream(&mut session, source, &mut show, out)
        }
    };
    match failed {
        true => Err(Diagnostic::new(
            "failed",
            "a statement failed (shown above)",
        )),
        false => Ok(String::new()),
    }
}

/// With `--delay`, wait before showing a statement that is not blank,
/// output shown so far flushed first (for recording a run as it goes).
fn pause(delay: Option<u64>, source: &str, out: &mut dyn Write) {
    if let Some(ms) = delay.filter(|_| !source.trim().is_empty()) {
        let _ = out.flush();
        std::thread::sleep(std::time::Duration::from_millis(ms));
    }
}

/// `eval` and `run`, plain or as a notebook (a library run lists its
/// exports' types); other commands are not evaluations.
pub(crate) fn evaluation(
    command: &Command,
    source: &str,
    host: &mut Host,
) -> Option<Result<String, Diagnostic>> {
    // A library is not run: running one lists its exports, as `type`.
    if matches!(command, Command::Run { .. })
        && xetal_program::is_library_named(&origin(command), source)
    {
        return Some(typed(source, &origin(command)));
    }
    Some(match command {
        Command::Run {
            context: Some(path),
            seed,
            ..
        } => seed_or_env(*seed, host)
            .and_then(|seed| after_context(path, &origin(command), source, seed, host)),
        Command::Eval(EvalArgs {
            echo: true,
            seed,
            delay,
            untyped,
            ..
        })
        | Command::Run {
            echo: true,
            seed,
            delay,
            untyped,
            ..
        } => seed_or_env(*seed, host)
            .and_then(|seed| echo(source, &origin(command), (seed, *delay, *untyped), host)),
        Command::Eval(EvalArgs { untyped, seed, .. }) | Command::Run { untyped, seed, .. } => {
            seed_or_env(*seed, host)
                .and_then(|seed| evaluate(source, &origin(command), *untyped, seed, host))
        }
        Command::Type(_) => typed(source, &origin(command)),
        _ => return None,
    })
}

/// Run `source` after the context file `path`; errors go to the error
/// stream and fail the run.
fn after_context(
    path: &str,
    origin: &str,
    source: &str,
    seed: Option<u64>,
    host: &mut Host,
) -> Result<String, Diagnostic> {
    let context = host
        .files
        .read(path)
        .map_err(|e| Diagnostic::new("unreadable", format!("cannot read {path}: {e}")))?;
    let seed = seed.unwrap_or_else(xetal_eval::Rng::fresh_seed);
    let cell = xetal_repl::continued(origin, &context, source, seed);
    let _ = write!(host.out, "{}", cell.out);
    let _ = write!(host.err, "{}", cell.err);
    match cell.err.lines().any(|l| l.starts_with("error[")) {
        true => Err(Diagnostic::new("failed", "the block failed (shown above)")),
        false => Ok(String::new()),
    }
}

/// The name a program is reported by, and beside which its libraries
/// are looked for: its file, or `-e` for text on the command line.
fn origin(command: &Command) -> String {
    match command {
        Command::Run { file, .. } => file.clone(),
        Command::Eval(EvalArgs { input, .. }) | Command::Type(input) => {
            input.file.clone().unwrap_or_else(|| "-e".into())
        }
        _ => "-e".into(),
    }
}

/// `xetal type`: the type of each top-level item of a program, or of
/// each export of a library (a file naming `l:`).
fn typed(source: &str, name: &str) -> Result<String, Diagnostic> {
    let library = xetal_program::is_library_named(name, source);
    let mut loaded = match library {
        true => xetal_program::load_library(name, source)?,
        false => xetal_program::load(name, source)?,
    };
    let lines = xetal_types::check_program(&mut loaded.program)
        .map_err(|d| xetal_program::located(&loaded.sources, d))?;
    Ok(match library {
        true => xetal_program::library_types(&loaded.sources, lines),
        false => xetal_program::program_types(lines),
    }
    .join("\n"))
}
