//! The command line: subcommands, options and inputs.

use clap::{Args, Parser, Subcommand};
use xetal_base::{Diagnostic, LANG_NAME};

use crate::stages::read_input;

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
                  source is plain ASCII. An underlined letter makes a name a \
                  function (`r_ev` is reverse), `_digits` give its axes \
                  (`o_-_2` rotates along axis 2), and a quoted function is \
                  an operand (`'+ r_/ v` reduces v by plus).",
    after_long_help = include_str!("cli_help.txt"),
    args_conflicts_with_subcommands = true
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Option<Command>,
    /// A script to run (`xetal FILE` is `xetal run FILE`; for shebangs).
    pub(crate) script: Option<String>,
}

/// Source given inline with `-e` or as a file path.
#[derive(Args)]
pub(crate) struct Input {
    /// Source text to process.
    #[arg(
        short = 'e',
        long = "expr",
        conflicts_with = "file",
        allow_hyphen_values = true
    )]
    pub(crate) expr: Option<String>,
    /// Source file to process.
    pub(crate) file: Option<String>,
}

/// `eval` and `run` options: type-checked unless `--untyped`.
#[derive(Args)]
pub(crate) struct EvalArgs {
    #[command(flatten)]
    pub(crate) input: Input,
    /// Skip the type checker (for experiments such as the Y combinator).
    #[arg(long)]
    pub(crate) untyped: bool,
    /// Seed `r_oll!` so a run can be repeated (default: XETAL_SEED, else
    /// unpredictable).
    #[arg(long)]
    pub(crate) seed: Option<u64>,
}

/// `render` options: decorated Unicode by default.
#[derive(Args)]
pub(crate) struct RenderArgs {
    #[command(flatten)]
    pub(crate) input: Input,
    /// Convert decorated Unicode back to raw ASCII (validated by lexing).
    #[arg(long)]
    pub(crate) raw: bool,
    /// Print LaTeX math for a post-processor (KaTeX, MathJax, pdflatex).
    #[arg(long, conflicts_with = "raw")]
    pub(crate) latex: bool,
    /// Color the decorated form for a terminal; text that does not lex
    /// is shown as typed, in red, and never stops the rendering.
    #[arg(long, conflicts_with_all = ["raw", "latex"])]
    pub(crate) color: bool,
}

#[derive(Subcommand)]
pub(crate) enum Command {
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
    /// Print the inferred type of each top-level item.
    Type(Input),
    /// Type-check, evaluate and print each result.
    Eval(EvalArgs),
    /// Type-check and run a program file.
    Run {
        file: String,
        /// Skip the type checker.
        #[arg(long)]
        untyped: bool,
        /// Seed `r_oll!` (default: XETAL_SEED, else unpredictable).
        #[arg(long)]
        seed: Option<u64>,
    },
    /// Start an interactive session.
    Repl,
    /// Edit a file: ASCII left, live decorated view right, types below
    /// (Ctrl-S save, Ctrl-R run, Ctrl-Q quit).
    Edit {
        /// The file to edit (created on the first save if missing).
        file: String,
    },
}

impl Command {
    pub(crate) fn stage(&self) -> &'static str {
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
            Command::Edit { .. } => "edit",
        }
    }

    /// The source this command operates on, if it takes one.
    pub(crate) fn source(&self) -> Option<Result<String, Diagnostic>> {
        match self {
            Command::Lex(i)
            | Command::Parse(i)
            | Command::Fmt(i)
            | Command::Core(i)
            | Command::Type(i)
            | Command::Eval(EvalArgs { input: i, .. }) => {
                Some(read_input(i.expr.as_deref(), i.file.as_deref()))
            }
            Command::Render(r) => {
                Some(read_input(r.input.expr.as_deref(), r.input.file.as_deref()))
            }
            Command::Run { file, .. } => Some(read_input(None, Some(file))),
            Command::Repl | Command::Edit { .. } => None,
        }
    }
}
