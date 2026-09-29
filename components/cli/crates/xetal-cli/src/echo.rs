//! `xetal run --echo`: each statement pretty-printed (decorated and
//! colored), then its output indented below it, like a notebook.

use xetal_base::Diagnostic;

use crate::args::{Command, EvalArgs};
use crate::stages::{evaluate, seed_or_env};
use xetal_view::{ansi, view};

const RED: &str = "\u{1b}[31m";
const RESET: &str = "\u{1b}[0m";

/// Print `source` as a notebook; an error fails the run after
/// everything has been shown.
pub(crate) fn echo(source: &str, seed: Option<u64>) -> Result<String, Diagnostic> {
    let seed = seed.unwrap_or_else(xetal_eval::Rng::fresh_seed);
    let mut failed = false;
    for cell in xetal_repl::notebook(source, seed) {
        println!("{}", ansi(&view(&cell.source)));
        for line in cell.out.lines() {
            println!("  {line}");
        }
        for line in cell.err.lines() {
            failed |= line.starts_with("error[");
            println!("  {RED}{line}{RESET}");
        }
    }
    match failed {
        true => Err(Diagnostic::new(
            "failed",
            "a statement failed (shown above)",
        )),
        false => Ok(String::new()),
    }
}

/// `eval` and `run`, plain or as a notebook; other commands are not
/// evaluations.
pub(crate) fn evaluation(command: &Command, source: &str) -> Option<Result<String, Diagnostic>> {
    Some(match command {
        Command::Eval(EvalArgs {
            echo: true, seed, ..
        })
        | Command::Run {
            echo: true, seed, ..
        } => seed_or_env(*seed).and_then(|seed| echo(source, seed)),
        Command::Eval(EvalArgs { untyped, seed, .. }) | Command::Run { untyped, seed, .. } => {
            seed_or_env(*seed).and_then(|seed| evaluate(source, *untyped, seed))
        }
        _ => return None,
    })
}
