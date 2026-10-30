//! The browser terminal's shell, driven by keys without a browser: it
//! opens the host's file in the editor, runs xetal's commands at its
//! prompt, the REPL, and its own words; and it splits words, reads
//! colors and dumps a screen as text.

use std::collections::BTreeMap;

use ratatui::Terminal;
use ratatui::backend::TestBackend;
use xetal_keys::{Key, keys};
use xetal_shell::{Catalog, Flow, Shell, text, words};

struct Files(BTreeMap<&'static str, &'static str>);

impl Catalog for Files {
    fn read(&self, path: &str) -> Option<String> {
        self.0.get(path).map(|t| t.to_string())
    }

    fn list(&self) -> Vec<String> {
        self.0.keys().map(|k| k.to_string()).collect()
    }
}

fn shell(name: &str, text: &str) -> Shell {
    let files = Files(BTreeMap::from([
        ("demo.xtl", "1 + 1\n"),
        ("Two.xtl", "l:t_wo := 2\n"),
    ]));
    Shell::new(name, text, Box::new(files))
}

/// Press the keys `spec` names; the last key's flow.
fn press(shell: &mut Shell, spec: &str) -> Flow {
    let mut flow = Flow::Continue;
    for key in keys(spec).unwrap() {
        flow = shell.handle(key);
    }
    flow
}

/// The shell's screen, as text.
fn shown(shell: &Shell) -> String {
    let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
    terminal.draw(|f| shell.draw(f)).unwrap();
    text(terminal.backend().buffer()).join("\n")
}

#[test]
fn it_opens_the_hosts_text_in_the_editor_and_keeps_it_unsaved() {
    let mut sh = shell("mine.xtl", "1 + 2");
    assert!(shown(&sh).contains("ASCII"), "{}", shown(&sh));
    press(&mut sh, "<End> * 3<C-q>");
    assert!(shown(&sh).contains("^Q again"), "unsaved: a second Ctrl-Q");
    press(&mut sh, "<C-q>");
    assert!(
        shown(&sh).contains("$ xetal edit mine.xtl"),
        "{}",
        shown(&sh)
    );
    assert_eq!(sh.text_of("mine.xtl").as_deref(), Some("1 + 2 * 3"));
    press(&mut sh, "xetal edit mine.xtl<Enter>");
    assert!(shown(&sh).contains("1 + 2 * 3"), "the buffer again");
}

#[test]
fn xetal_commands_run_at_the_prompt() {
    let mut sh = shell("x.xtl", "");
    press(&mut sh, "<C-q>xetal eval -e '6 * 7'<Enter>");
    assert!(shown(&sh).contains("42"), "{}", shown(&sh));
    press(
        &mut sh,
        "xetal run demo.xtl<Enter>xetal run nope.xtl<Enter>",
    );
    let screen = shown(&sh);
    assert!(
        screen.contains("\n2\n") || screen.contains("2\n$"),
        "{screen}"
    );
    assert!(screen.contains("cannot read nope.xtl"), "{screen}");
    press(&mut sh, "xetal type -e 'u:f_ := { _r + 1 }'<Enter>");
    assert!(
        shown(&sh).contains("u:f_ : Num a => a -> a"),
        "{}",
        shown(&sh)
    );
}

#[test]
fn the_repl_keeps_its_definitions_until_ctrl_d() {
    let mut sh = shell("x.xtl", "");
    press(&mut sh, "<C-q>xetal repl<Enter>x := 5<Enter>x * 2<Enter>");
    let screen = shown(&sh);
    assert!(
        screen.contains("xetal> x \u{d7} 2") && screen.contains("10"),
        "{screen}"
    );
    assert_eq!(press(&mut sh, "<C-d>"), Flow::Continue, "the REPL ends");
    assert!(shown(&sh).trim_end().ends_with("$"), "{}", shown(&sh));
    assert_eq!(press(&mut sh, "<C-d>"), Flow::Exit, "the shell ends");
}

#[test]
fn its_own_words() {
    let mut sh = shell("x.xtl", "");
    press(
        &mut sh,
        "<C-q>ls<Enter>cat Two.xtl<Enter>frob<Enter>help<Enter>",
    );
    let screen = shown(&sh);
    assert!(
        screen.contains("Two.xtl") && screen.contains("demo.xtl"),
        "{screen}"
    );
    assert!(screen.contains("l:t_wo := 2"), "{screen}");
    assert!(screen.contains("frob: command not found"), "{screen}");
    assert!(screen.contains("xetal repl"), "help: {screen}");
    press(&mut sh, "clear<Enter>");
    assert_eq!(shown(&sh).trim(), "$");
    assert_eq!(press(&mut sh, "exit<Enter>"), Flow::Exit);
}

#[test]
fn opening_a_file_keeps_the_editors_text() {
    let mut sh = shell("a.xtl", "1");
    press(&mut sh, "<End>2");
    sh.open("demo.xtl");
    assert!(shown(&sh).contains("1 + 1"), "{}", shown(&sh));
    assert_eq!(sh.text_of("a.xtl").as_deref(), Some("12"));
}

#[test]
fn words_split_as_a_shell_splits_them() {
    assert_eq!(
        words(r#"xetal eval -e '1 + 2' "a \"b\"" c\ d"#).unwrap(),
        vec!["xetal", "eval", "-e", "1 + 2", "a \"b\"", "c d"]
    );
    assert!(words("echo 'open").unwrap_err().contains("not closed"));
}

#[test]
fn colors_become_styles() {
    use ratatui::style::Color;
    let line = xetal_shell::line("\u{1b}[31merror\u{1b}[0m plain\u{1b}[2K");
    assert_eq!(line.spans.len(), 2);
    assert_eq!(line.spans[0].content, "error");
    assert_eq!(line.spans[0].style.fg, Some(Color::Red));
    assert_eq!(line.spans[1].content, " plain");
    let _ = Key::ctrl('q');
}
