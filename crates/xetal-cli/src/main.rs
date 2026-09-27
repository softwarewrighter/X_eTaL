//! The `xetal` binary: every pipeline stage exposed as deterministic text.

use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};
use xetal_base::{Diagnostic, LANG_NAME};

#[derive(Parser)]
#[command(name = LANG_NAME, bin_name = "xetal", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Source given inline with `-e` or as a file path.
#[derive(Args)]
struct Input {
    /// Source text to process.
    #[arg(short = 'e', long = "expr", conflicts_with = "file")]
    expr: Option<String>,
    /// Source file to process.
    file: Option<String>,
}

#[derive(Subcommand)]
enum Command {
    /// Print the token stream.
    Lex(Input),
    /// Print the decorated Unicode form.
    Render(Input),
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
}

fn run(command: &Command) -> Result<String, Diagnostic> {
    Err(Diagnostic::unsupported(command.stage()))
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(&cli.command) {
        Ok(text) => {
            println!("{text}");
            ExitCode::SUCCESS
        }
        Err(diag) => {
            eprintln!("{diag}");
            ExitCode::FAILURE
        }
    }
}
