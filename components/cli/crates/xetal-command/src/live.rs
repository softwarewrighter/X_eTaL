//! A notebook run shown as it goes, laid out like an APL session: each
//! statement is shown indented, where APL prompts for input, just
//! before it runs; its output follows flush left as the program writes
//! it (a long computation can report its progress), then its errors.

use std::io::{self, Write};

use xetal_repl::{Reply, Session};

pub(crate) const RED: &str = "\u{1b}[31m";
pub(crate) const RESET: &str = "\u{1b}[0m";

/// A statement's output, written as it comes (flushed at once).
pub(crate) struct Indent<'o> {
    pub(crate) at_start: bool,
    pub(crate) out: &'o mut (dyn Write + Send),
}

/// How far a statement is indented: APL's prompt, six spaces.
pub(crate) const PROMPT: &str = "      ";

impl Write for Indent<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.out.write_all(buf)?;
        self.at_start = buf.ends_with(b"\n");
        self.out.flush()?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.out.flush()
    }
}

/// Run `source` a statement at a time, `show` drawing each statement
/// before it runs, output and errors to `out`; whether any statement
/// failed.
pub(crate) fn stream(
    session: &mut Session,
    source: &str,
    show: &mut dyn FnMut(&str, &mut (dyn Write + Send)),
    out: &mut (dyn Write + Send),
) -> bool {
    let (mut failed, mut pending) = (false, Vec::new());
    for line in source.lines() {
        pending.push(line);
        let text = pending.join("\n");
        if unclosed(&text) {
            continue;
        }
        show(&text, out);
        failed |= run(session, &pending, out);
        pending.clear();
    }
    if !pending.is_empty() {
        show(&pending.join("\n"), out);
        let _ = writeln!(
            out,
            "{RED}error[unclosed]: the file ends inside a bracket{RESET}"
        );
        failed = true;
    }
    failed
}

/// Feed one statement's lines; print its errors; whether it failed.
fn run(session: &mut Session, lines: &[&str], out: &mut (dyn Write + Send)) -> bool {
    let mut reply = Reply::More;
    for line in lines {
        let mut indent = Indent {
            at_start: true,
            out,
        };
        reply = session.feed_to(line, &mut indent);
    }
    let Reply::Done { err, .. } = reply else {
        return false;
    };
    let mut failed = false;
    for line in err.lines() {
        failed |= line.starts_with("error[");
        let _ = writeln!(out, "{RED}{line}{RESET}");
    }
    failed
}

/// Whether `text` stops inside a bracket (more lines to come).
pub(crate) fn unclosed(text: &str) -> bool {
    matches!(xetal_syntax::parse(text), Err(d) if d.code == "unclosed")
}
