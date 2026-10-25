//! Running the pipeline as far as a command asks.

use xetal_base::Diagnostic;

use crate::args::{Command, RenderArgs};
use crate::host::Host;

/// Run the pipeline as far as `command` asks against `host`. Every
/// stage runs the earlier ones first, so an early error is reported by
/// any command. The interactive commands (`repl`, `edit`) are the
/// host's own and are reported unsupported here.
pub fn run(command: &Command, host: &mut Host) -> Result<String, Diagnostic> {
    if let Command::Lint(args) = command {
        let request = args.request(host)?;
        let Host { out, err, .. } = host;
        return xetal_linter::command(&request, out, err);
    }
    let Some(source) = command.source(host) else {
        return Err(Diagnostic::unsupported(command.stage()));
    };
    let source = source?;
    if let Some(result) = whole(command, &source, host) {
        return result;
    }
    if let Some(result) = crate::echo::evaluation(command, &source, host) {
        return result;
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
        _ => Err(Diagnostic::unsupported(command.stage())),
    }
}

/// The commands that take the whole source rather than its tokens:
/// render, diagram, expand, migrate and doc (its model or its tests).
fn whole(command: &Command, source: &str, host: &mut Host) -> Option<Result<String, Diagnostic>> {
    let named = |input: &crate::args::Input| input.file.clone().unwrap_or("-e".into());
    Some(match command {
        Command::Render(args) => render(args, source),
        Command::Diagram(_) => xetal_diagram::diagram(source),
        Command::Expand(input) => xetal_program::expanded(&named(input), source),
        Command::Migrate(_) => xetal_names::migrate(source).map(|s| s.trim_end().to_string()),
        Command::Doc(args) if args.test => xetal_doctest::test(&named(&args.input), source),
        Command::Doc(args) => doc(args, &named(&args.input), source, host),
        _ => return None,
    })
}

/// `xetal doc`: the model as JSON, or the site with its search written
/// into `--out` (the paths written, one a line); of FILE and any MORE.
fn doc(
    args: &crate::args::DocArgs,
    name: &str,
    source: &str,
    host: &mut Host,
) -> Result<String, Diagnostic> {
    let mut inputs = vec![(name.to_string(), source.to_string())];
    for more in &args.more {
        inputs.push((more.clone(), host.input(None, Some(more))?));
    }
    let files = xetal_doc::models(&inputs)?;
    for w in xetal_doc::warnings(&files) {
        host.warn(w);
    }
    match &args.out {
        Some(dir) => xetal_docsearch::document(&inputs, std::path::Path::new(dir)),
        None => Ok(xetal_doc::to_json(&files)),
    }
}

/// `xetal render`: the decorated form, or it in ANSI colors or HTML
/// spans; in those two a notation call (MC32) is drawn as its `draws:`
/// macro wrote it when the file loads: the HTML as one math box (the
/// LaTeX in a `c-notation` span), the terminal as typed, styled as
/// notation.
fn render(args: &RenderArgs, source: &str) -> Result<String, Diagnostic> {
    let notations = || {
        let name = args.input.file.clone().unwrap_or("-e".into());
        xetal_notate::found(&name, source, &xetal_notate::FsLibraries::from_env())
    };
    if args.raw {
        let raw = xetal_render::undecorate(source)?;
        xetal_lex::lex(&raw)?;
        Ok(raw)
    } else if args.color {
        let segments = xetal_notate::marked(xetal_view::view(source), &notations());
        Ok(xetal_view::ansi(&segments))
    } else if args.html {
        let segments = xetal_notate::drawn(xetal_view::view(source), &notations());
        Ok(xetal_view::html(&segments))
    } else if args.latex {
        xetal_render::latex(source)
    } else {
        xetal_render::decorate(source)
    }
}

/// The `--seed` given, else `XETAL_SEED`, else the host's, else none
/// (unpredictable: a fresh one from the clock).
pub(crate) fn seed_or_env(seed: Option<u64>, host: &Host) -> Result<Option<u64>, Diagnostic> {
    match (seed, std::env::var("XETAL_SEED")) {
        (Some(s), _) => Ok(Some(s)),
        (None, Ok(text)) => text.trim().parse().map(Some).map_err(|_| {
            Diagnostic::new(
                "bad-seed",
                format!("XETAL_SEED must be a whole number, got {text:?}"),
            )
        }),
        (None, Err(_)) => Ok(host.seed),
    }
}

/// Type-check (unless `untyped`), then evaluate, streaming results to
/// the host's output; warnings go to its error stream.
pub(crate) fn evaluate(
    source: &str,
    name: &str,
    untyped: bool,
    seed: Option<u64>,
    host: &mut Host,
) -> Result<String, Diagnostic> {
    let xetal_program::Loaded {
        sources,
        mut program,
    } = xetal_program::load(name, source)?;
    let at = |d| xetal_program::located(&sources, d);
    for warning in sources.warnings() {
        host.warn(warning);
    }
    if !untyped {
        xetal_types::check_program(&mut program).map_err(at)?;
    }
    let (warnings, result) = xetal_eval::eval_program(&program, host.out, seed);
    let _ = host.out.flush();
    for warning in xetal_linter::honored(&sources, &program, warnings) {
        host.warn(at(warning));
    }
    result.map(|()| String::new()).map_err(at)
}
