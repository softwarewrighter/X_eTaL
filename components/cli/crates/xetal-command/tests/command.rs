//! The command line as a library: arguments parsed as the binary parses
//! them, a command run against a host of the caller's (its files, its
//! output, its seed), the interactive commands left to the host.

use std::collections::HashMap;

use xetal_command::{Command, Files, Host, parse, run};

/// Files kept in memory, as a browser's store keeps them.
struct Memory(HashMap<&'static str, &'static str>);

impl Files for Memory {
    fn read(&self, path: &str) -> Result<String, String> {
        self.0
            .get(path)
            .map(|t| t.to_string())
            .ok_or_else(|| format!("{path}: no such file"))
    }
}

fn args(line: &str) -> Vec<String> {
    std::iter::once("xetal")
        .chain(line.split_whitespace())
        .map(String::from)
        .collect()
}

/// Run `line` against `files`; what it gave back, its output and its
/// errors.
fn exec(line: &str, files: &Memory) -> (Result<String, String>, String, String) {
    let cli = parse(args(line)).expect("the arguments parse");
    let command = cli.command.expect("a subcommand");
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let result = {
        let mut host = Host {
            files,
            out: &mut out,
            err: &mut err,
            seed: Some(7),
        };
        run(&command, &mut host).map_err(|d| d.to_string())
    };
    let text = |v: Vec<u8>| String::from_utf8_lossy(&v).into_owned();
    (result, text(out), text(err))
}

#[test]
fn eval_writes_results_to_the_hosts_output() {
    let files = Memory(HashMap::new());
    let (result, out, err) = exec("eval -e 1+2", &files);
    assert_eq!(result, Ok(String::new()));
    assert_eq!(out, "3\n");
    assert_eq!(err, "");
}

#[test]
fn run_reads_the_file_from_the_hosts_files() {
    let files = Memory(HashMap::from([("hello.xtl", "1 + 1\n")]));
    let (result, out, _) = exec("run hello.xtl", &files);
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(out, "2\n");
    let (result, _, _) = exec("run missing.xtl", &files);
    let message = result.unwrap_err();
    assert!(
        message.contains("cannot read missing.xtl: missing.xtl: no such file"),
        "{message}"
    );
}

#[test]
fn stages_give_their_text_back_and_lint_writes_its_findings() {
    let files = Memory(HashMap::from([(
        "k.xtl",
        "u:k_ := { a b -> a }\n1 u:k_ 2\n",
    )]));
    let (result, _, _) = exec("fmt -e 1+2", &files);
    assert_eq!(result, Ok("(1 + 2)".to_string()));
    let (result, _, _) = exec("render -e r_ev", &files);
    assert_eq!(result, Ok("r\u{332}ev".to_string()));
    let (result, _, err) = exec("lint k.xtl", &files);
    assert!(result.is_ok(), "{result:?}");
    assert!(err.contains("warning[unused-binding]"), "{err}");
}

#[test]
fn the_interactive_commands_are_the_hosts() {
    let files = Memory(HashMap::new());
    let (result, _, _) = exec("repl", &files);
    assert!(result.unwrap_err().contains("repl"));
    assert!(matches!(
        parse(args("edit x.xtl")).unwrap().command,
        Some(Command::Edit { .. })
    ));
}

#[test]
fn a_run_without_a_seed_rolls_from_the_hosts() {
    let files = Memory(HashMap::new());
    let first = exec("eval -e r_oll!_6_10", &files).1;
    let again = exec("eval -e r_oll!_6_10", &files).1;
    assert_eq!(first, again, "the same host seed, the same rolls");
}
