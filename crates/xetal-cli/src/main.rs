//! The `xetal` binary: every pipeline stage exposed as deterministic text.

use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};
use xetal_base::{Diagnostic, LANG_NAME};

/// Full `-V` / `--version` block: version, copyright, license,
/// repository, then build information from `build.rs`.
const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "\nCopyright (c) 2026 Michael A Wright\n",
    "License: ",
    env!("CARGO_PKG_LICENSE"),
    "\nRepository: ",
    env!("CARGO_PKG_REPOSITORY"),
    "\n\nBuild Information:\n  Host: ",
    env!("BUILD_HOST"),
    "\n  Commit: ",
    env!("GIT_HASH"),
    "\n  Timestamp: ",
    env!("BUILD_TIMESTAMP"),
);

/// A terse, statically typed, functional array language in ASCII.
#[derive(Parser)]
#[command(
    name = LANG_NAME,
    bin_name = "xetal",
    version = VERSION,
    about,
    long_about = "A terse, statically typed, functional array language whose \
                  source is plain ASCII. Typographic decoration changes what \
                  a name means: `t` is a noun, `t_` the rotate function, \
                  `t_2` rotate along axis 2, `+^r` reduce by `+`.",
    after_long_help = include_str!("cli_help.txt")
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Source given inline with `-e` or as a file path.
#[derive(Args)]
struct Input {
    /// Source text to process.
    #[arg(
        short = 'e',
        long = "expr",
        conflicts_with = "file",
        allow_hyphen_values = true
    )]
    expr: Option<String>,
    /// Source file to process.
    file: Option<String>,
}

/// `render` options: decorated Unicode by default.
#[derive(Args)]
struct RenderArgs {
    #[command(flatten)]
    input: Input,
    /// Convert decorated Unicode back to raw ASCII (validated by lexing).
    #[arg(long)]
    raw: bool,
    /// Print LaTeX math for a post-processor (KaTeX, MathJax, pdflatex).
    #[arg(long, conflicts_with = "raw")]
    latex: bool,
}

#[derive(Subcommand)]
enum Command {
    /// Print the token stream.
    Lex(Input),
    /// Print the decorated Unicode form (or --raw, --latex).
    Render(RenderArgs),
    /// Print the surface AST or an ambiguity report.
    Parse(Input),
    /// Print the canonical form.
    Fmt(Input),
    /// Print the Core IR.
    Core(Input),
    /// Print the inferred type.
    Type(Input),
    /// Evaluate and print the result.
    Eval(Input),
    /// Run a program file.
    Run { file: String },
    /// Start an interactive session.
    Repl,
}

impl Command {
    fn stage(&self) -> &'static str {
        match self {
            Command::Lex(_) => "lex",
            Command::Render(_) => "render",
            Command::Parse(_) => "parse",
            Command::Fmt(_) => "fmt",
            Command::Core(_) => "core",
            Command::Type(_) => "type",
            Command::Eval(_) => "eval",
            Command::Run { .. } => "run",
            Command::Repl => "repl",
        }
    }

    /// The source this command operates on, if it takes one.
    fn source(&self) -> Option<Result<String, Diagnostic>> {
        match self {
            Command::Lex(i)
            | Command::Parse(i)
            | Command::Fmt(i)
            | Command::Core(i)
            | Command::Type(i)
            | Command::Eval(i) => Some(read_input(i.expr.as_deref(), i.file.as_deref())),
            Command::Render(r) => {
                Some(read_input(r.input.expr.as_deref(), r.input.file.as_deref()))
            }
            Command::Run { file } => Some(read_input(None, Some(file))),
            Command::Repl => None,
        }
    }
}

fn read_input(expr: Option<&str>, file: Option<&str>) -> Result<String, Diagnostic> {
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
fn run(command: &Command) -> Result<String, Diagnostic> {
    let Some(source) = command.source() else {
        return Err(Diagnostic::unsupported(command.stage()));
    };
    let source = source?;
    if let Command::Render(args) = command {
        return render(args, &source);
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
        _ => {
            xetal_syntax::parse(&source)?;
            Err(Diagnostic::unsupported(command.stage()))
        }
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

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(&cli.command) {
        Ok(text) => {
            if !text.is_empty() {
                println!("{text}");
            }
            ExitCode::SUCCESS
        }
        Err(diag) => {
            eprintln!("{diag}");
            ExitCode::FAILURE
        }
    }
}
