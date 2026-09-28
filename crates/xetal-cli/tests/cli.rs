use assert_cmd::Command;
use predicates::str::starts_with;
use xetal_base::LANG_NAME;

fn xetal() -> Command {
    Command::new(env!("CARGO_BIN_EXE_xetal"))
}

fn stdout_of(args: &[&str]) -> String {
    let out = xetal().args(args).assert().success();
    String::from_utf8(out.get_output().stdout.clone()).expect("utf-8")
}

#[test]
fn version_block_has_name_copyright_license_repo_and_build_info() {
    let text = stdout_of(&["--version"]);
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(
        lines[..4],
        [
            format!("{LANG_NAME} {}", env!("CARGO_PKG_VERSION")).as_str(),
            "Copyright (c) 2026 Michael A Wright",
            "License: MIT",
            "Repository: https://github.com/softwarewrighter/X_eTaL",
        ]
    );
    for field in [
        "Build Information:",
        "  Host: ",
        "  Commit: ",
        "  Timestamp: ",
    ] {
        assert!(text.contains(field), "missing {field:?} in\n{text}");
    }
    assert_eq!(stdout_of(&["-V"]), text);
}

#[test]
fn long_help_extends_short_help_with_agent_instructions() {
    let short = stdout_of(&["-h"]);
    let long = stdout_of(&["--help"]);
    assert!(long.len() > short.len());
    assert!(long.contains("AI CODING AGENT INSTRUCTIONS:"));
    assert!(!short.contains("AI CODING AGENT INSTRUCTIONS:"));
}

#[test]
fn unimplemented_stage_reports_unsupported() {
    xetal()
        .args(["eval", "-e", "1 + 2"])
        .assert()
        .failure()
        .stdout("")
        .stderr("error[unsupported]: stage `eval` is not implemented\n");
}

#[test]
fn missing_subcommand_is_a_usage_error() {
    xetal().assert().failure().code(2);
}

#[test]
fn lex_dumps_tokens_with_spans() {
    xetal()
        .args(["lex", "-e", "u:s_quare 7"])
        .assert()
        .success()
        .stdout("0..9 Func(u:s_quare)\n10..11 Num(7)\n");
}

#[test]
fn lex_reads_a_file() {
    let dir = std::env::temp_dir().join(format!("xetal-cli-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("one.xtl");
    std::fs::write(&path, "r_/_2\n").unwrap();
    xetal()
        .args(["lex", path.to_str().unwrap()])
        .assert()
        .success()
        .stdout("0..5 Func(r_/, axes=[2])\n5..6 Newline\n");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn lex_error_is_a_diagnostic_on_stderr() {
    xetal()
        .args(["lex", "-e", "3-1"])
        .assert()
        .code(1)
        .stdout("")
        .stderr(starts_with("error[ambiguous-minus]: "));
}

#[test]
fn later_stages_report_lex_errors_before_unsupported() {
    xetal()
        .args(["eval", "-e", "a_b_c"])
        .assert()
        .code(1)
        .stderr(starts_with("error[bad-name]: "));
}

#[test]
fn missing_input_is_an_error() {
    xetal()
        .arg("lex")
        .assert()
        .code(1)
        .stderr("error[no-input]: give source with -e EXPR or a FILE path\n");
}

#[test]
fn unreadable_file_is_an_error() {
    xetal()
        .args(["lex", "/nonexistent/x.xtl"])
        .assert()
        .code(1)
        .stderr(starts_with("error[io]: cannot read /nonexistent/x.xtl"));
}

#[test]
fn expression_may_start_with_a_negative_literal() {
    xetal()
        .args(["lex", "-e", "-1 0"])
        .assert()
        .success()
        .stdout("0..2 Num(-1)\n3..4 Num(0)\n");
}

#[test]
fn render_decorates_raw_source() {
    xetal()
        .args(["render", "-e", "o_-_12 x^2 r_ev"])
        .assert()
        .success()
        .stdout("o\u{332}-\u{2081}\u{2082} x\u{b2} r\u{332}ev\n");
}

#[test]
fn render_raw_inverts_and_validates() {
    xetal()
        .args(["render", "--raw", "-e", "o\u{332}-\u{2081}\u{2082} x\u{b2}"])
        .assert()
        .success()
        .stdout("o_-_12 x^2\n");
    xetal()
        .args(["render", "--raw", "-e", "o\u{332}-\u{2080}"])
        .assert()
        .code(1)
        .stderr(starts_with("error[bad-axis]: "));
}

#[test]
fn render_latex_is_one_way_math() {
    xetal()
        .args(["render", "--latex", "-e", "r_/_2 x"])
        .assert()
        .success()
        .stdout("{\\mathrm{\\underline{r}}/_{2}}\\ {\\mathrm{x}}\n");
    xetal()
        .args(["render", "--latex", "--raw", "-e", "x"])
        .assert()
        .code(2);
}
