//! The session on standard input and output. A prompt is shown only
//! when input is a terminal, so piped sessions print just results.

use std::io::{BufRead, IsTerminal, Write};

use crate::{Reply, Session};

fn prompt(more: bool) {
    print!("{}", if more { "    ...  " } else { "xetal> " });
    let _ = std::io::stdout().flush();
}

/// Read lines until end of input; results go to stdout, warnings and
/// errors to stderr.
pub fn stdio() -> std::io::Result<()> {
    let tty = std::io::stdin().is_terminal();
    let mut session = Session::default();
    if tty {
        prompt(false);
    }
    for line in std::io::stdin().lock().lines() {
        let more = match session.feed(&line?) {
            Reply::More => true,
            Reply::Done { out, err } => {
                print!("{out}");
                eprint!("{err}");
                false
            }
        };
        if tty {
            prompt(more);
        }
    }
    Ok(())
}
