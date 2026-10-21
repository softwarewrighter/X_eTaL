//! The `xetal lint` driver, as clippy-driver is to clippy's lints: the
//! lints and their catalog live in `xetal-lint` (over Core); this crate
//! applies levels to them, from the command line (`-A`, `-W`, `-D`) and
//! from `# lint:` comments in the program, places each finding in the
//! file as written, and reports. See docs/lint.md.

mod command;
mod directive;
mod driver;
mod fix;
mod rule;
mod scope;

pub use command::{Request, command, json_string};
pub use directive::{Directive, directives};
pub use driver::{Finding, honored, lint, lint_with};
pub use fix::{Edit, apply, fixed};
pub use rule::{Rule, Scope, command_line, known, resolve};
