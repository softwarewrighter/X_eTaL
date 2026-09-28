//! The `xetal` binary: every pipeline stage exposed as deterministic text.

mod args;
mod stages;

use std::process::ExitCode;

use clap::Parser;

use crate::args::{Cli, Command};
use crate::stages::run;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let command = match (cli.command, cli.script) {
        (Some(command), _) => command,
        (None, Some(file)) => Command::Run {
            file,
            untyped: false,
        },
        (None, None) => {
            use clap::CommandFactory;
            Cli::command()
                .error(
                    clap::error::ErrorKind::MissingSubcommand,
                    "give a subcommand or a script FILE",
                )
                .exit()
        }
    };
    match run(&command) {
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
