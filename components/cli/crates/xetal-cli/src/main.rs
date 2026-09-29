//! The `xetal` binary: every pipeline stage exposed as deterministic text.

mod args;
mod context;
mod echo;
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
            seed: None,
            echo: false,
            delay: None,
            context: None,
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
