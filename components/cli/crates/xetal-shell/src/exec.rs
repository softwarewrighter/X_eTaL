//! A line typed at the prompt: `xetal` and its subcommands, or one of
//! the shell's own words (`help`, `ls`, `cat`, `clear`, `exit`).

use xetal_base::Diagnostic;
use xetal_command::{Command, Files, Host};

use crate::ansi::red;
use crate::shell::{Flow, Shell};

const HELP: &str = "\
This is a shell for xetal, as at a terminal:
  xetal eval -e '1 + 2'        evaluate (also run, type, lint, fmt, render, expand, ...)
  xetal run life.xtl           run a file (a demo, a library or one of your files)
  xetal repl                   the REPL (Ctrl-D ends it)
  xetal edit FILE              the editor (Ctrl-S save, Ctrl-R or F5 run, Ctrl-Q quit)
  xetal --help                 every command and option
  ls, cat FILE, clear, help    the files, a file's text, an empty screen, this
  exit (or Ctrl-D)             back to the three panes, with the file's text";

/// Run the command line `text`.
pub(crate) fn command(shell: &mut Shell, text: &str) -> Flow {
    let words = match crate::words(text) {
        Ok(w) => w,
        Err(e) => {
            shell.print(&red(&format!("xetal-shell: {e}")));
            return Flow::Continue;
        }
    };
    let Some(first) = words.first() else {
        return Flow::Continue;
    };
    match first.as_str() {
        "xetal" => xetal(shell, words),
        "help" => shell.print(HELP),
        "ls" => {
            let names = shell.files_list().join("\n");
            shell.print(&names);
        }
        "cat" => cat(shell, &words[1..]),
        "clear" => shell.lines.clear(),
        "exit" | "logout" => return Flow::Exit,
        other => shell.print(&red(&format!(
            "xetal-shell: {other}: command not found (try help)"
        ))),
    }
    Flow::Continue
}

/// `cat FILE...`: each file's text.
fn cat(shell: &mut Shell, names: &[String]) {
    for name in names {
        match shell.text_of(name) {
            Some(text) => shell.print(&text),
            None => shell.print(&red(&format!("cat: {name}: no such file"))),
        }
    }
}

/// `xetal ...`: parsed as the binary parses its arguments; the REPL and
/// the editor take the screen, the rest run through `xetal-command`.
fn xetal(shell: &mut Shell, words: Vec<String>) {
    let cli = match xetal_command::parse(words) {
        Ok(cli) => cli,
        Err(e) => return shell.print(&e.to_string()),
    };
    xetal_grid::set_ascii(cli.ascii);
    xetal_grid::set_boxed(cli.boxed);
    xetal_system::set_flags(cli.cfg.clone());
    let command = match (cli.command, cli.script) {
        (Some(c), None) => c,
        (None, Some(file)) => Command::Run {
            file,
            untyped: false,
            seed: None,
            echo: false,
            delay: None,
            context: None,
        },
        _ => return shell.print(&red("error: give a subcommand or a script FILE")),
    };
    match &command {
        Command::Repl => shell.repl(),
        Command::Edit { file, .. } => shell.edit(file),
        _ => run(shell, &command),
    }
}

/// The shell's files as the commands read them: its buffers, then the
/// host's catalog.
struct ShellFiles<'s>(&'s Shell);

impl Files for ShellFiles<'_> {
    fn read(&self, path: &str) -> Result<String, String> {
        self.0
            .text_of(path)
            .ok_or_else(|| "no such file".to_string())
    }
}

/// Run a command; print what it wrote, what it gave back, or its error
/// placed in its source.
fn run(shell: &mut Shell, command: &Command) {
    let (mut out, mut err) = (Vec::new(), Vec::new());
    xetal_store::take_shown();
    let (result, placed) = {
        let files = ShellFiles(shell);
        let mut host = Host {
            files: &files,
            out: &mut out,
            err: &mut err,
            seed: Some(shell.seed),
        };
        let result = xetal_command::run(command, &mut host);
        let placed = result
            .as_ref()
            .err()
            .map(|d| place(d.clone(), command, &host));
        (result, placed)
    };
    shell.print(&String::from_utf8_lossy(&out));
    shell.print(&red(&String::from_utf8_lossy(&err)));
    match (result, placed) {
        (Ok(text), _) => shell.print(&text),
        (Err(_), Some(d)) => shell.print(&red(&d.to_string())),
        (Err(d), None) => shell.print(&red(&d.to_string())),
    }
    let pictures = xetal_store::take_shown().len();
    if pictures > 0 {
        shell.print(&format!(
            "({pictures} picture(s) drawn: the three panes' Run shows them)"
        ));
    }
}

/// `d` with its file, line and column, as the binary places it.
fn place(d: Diagnostic, command: &Command, host: &Host) -> Diagnostic {
    match (d.place(), d.span, command.source(host)) {
        (None, Some(_), Some(Ok(text))) => d.placed_in(&command.origin(), &text),
        _ => d,
    }
}
