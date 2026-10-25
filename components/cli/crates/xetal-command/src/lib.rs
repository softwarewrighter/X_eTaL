//! The `xetal` command line as a library (X_eTaL-extensions' ask E3):
//! the arguments parsed into a [`Command`] and each command run against
//! a [`Host`], which says where files are read from, where output and
//! errors go, and what seed a run rolls from when none is given. The
//! binary is a thin shell over it with the process's files, standard
//! output and error; the browser's terminal gives it the page's store,
//! its screen and a seed of its own. The interactive commands (`repl`,
//! `edit`) are the host's: [`run`] reports them unsupported.

mod args;
mod echo;
mod host;
mod live;
mod once;
mod stages;

pub use args::{Cli, Command, DocArgs, EvalArgs, Input, LintArgs, RenderArgs};
pub use host::{Files, Host, Local};
pub use stages::run;

/// The command line `args` (the program's name first) parsed; a clap
/// error (help and version among them) is given back to print.
pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Cli, clap::Error> {
    use clap::Parser;
    Cli::try_parse_from(args)
}
