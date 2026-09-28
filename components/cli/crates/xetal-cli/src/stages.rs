//! Running the pipeline as far as a command asks.

use xetal_base::Diagnostic;

use crate::args::{Command, EvalArgs, RenderArgs};

pub(crate) fn read_input(expr: Option<&str>, file: Option<&str>) -> Result<String, Diagnostic> {
    match (expr, file) {
        (Some(expr), _) => Ok(expr.to_string()),
        (None, Some(path)) => std::fs::read_to_string(path)
            .map_err(|e| Diagnostic::new("io", format!("cannot read {path}: {e}"))),
        (None, None) => Err(Diagnostic::new(
            "no-input",
            "give source with -e EXPR or a FILE path",
        )),
    }
}

/// Run the pipeline as far as `command` asks. Every stage runs the
/// earlier ones first, so an early error is reported by any command.
pub(crate) fn run(command: &Command) -> Result<String, Diagnostic> {
    let Some(source) = command.source() else {
        return Err(Diagnostic::unsupported(command.stage()));
    };
    let source = source?;
    if let Command::Render(args) = command {
        return render(args, &source);
    }
    match command {
        Command::Eval(EvalArgs { untyped, .. }) | Command::Run { untyped, .. } => {
            return evaluate(&source, *untyped);
        }
        _ => {}
    }
    let tokens = xetal_lex::lex(&source)?;
    match command {
        Command::Lex(_) => Ok(tokens
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")),
        Command::Parse(_) => Ok(xetal_syntax::parse(&source)?.to_string()),
        Command::Fmt(_) => xetal_render::canonical(&source),
        Command::Core(_) => Ok(xetal_core::lower(&source)?.to_string()),
        Command::Type(_) => Ok(xetal_types::check_source(&source)?.join("\n")),
        _ => Err(Diagnostic::unsupported(command.stage())),
    }
}

fn render(args: &RenderArgs, source: &str) -> Result<String, Diagnostic> {
    if args.raw {
        let raw = xetal_render::undecorate(source)?;
        xetal_lex::lex(&raw)?;
        Ok(raw)
    } else if args.latex {
        xetal_render::latex(source)
    } else {
        xetal_render::decorate(source)
    }
}

/// Type-check (unless `untyped`), then evaluate, streaming results to
/// stdout; warnings go to stderr.
fn evaluate(source: &str, untyped: bool) -> Result<String, Diagnostic> {
    let mut program = xetal_core::lower(source)?;
    if !untyped {
        xetal_types::check_program(&mut program)?;
    }
    let mut stdout = std::io::stdout();
    let (warnings, result) = xetal_eval::eval_program(&program, &mut stdout);
    for warning in warnings {
        eprintln!("{warning}");
    }
    result.map(|()| String::new())
}
