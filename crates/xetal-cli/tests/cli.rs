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
        .args(["type", "-e", "1 + 2"])
        .assert()
        .failure()
        .stdout("")
        .stderr("error[unsupported]: stage `type` is not implemented\n");
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

#[test]
fn parse_prints_the_surface_tree() {
    xetal()
        .args(["parse", "-e", "u:s_ub := { _l - _r }; 10 u:s_ub 3"])
        .assert()
        .success()
        .stdout("(:= u:s_ub (lambda (_l _r) (- _l _r)))\n(u:s_ub 10 3)\n");
}

#[test]
fn later_stages_report_parse_errors_before_unsupported() {
    xetal()
        .args(["eval", "-e", "- 3"])
        .assert()
        .code(1)
        .stderr(starts_with("error[symbol-needs-left]: "));
    xetal()
        .args(["type", "-e", "1 + 2"])
        .assert()
        .code(1)
        .stderr("error[unsupported]: stage `type` is not implemented\n");
}

#[test]
fn fmt_prints_the_canonical_form() {
    xetal()
        .args(["fmt", "-e", "x := 3; a f_ b g_ c"])
        .assert()
        .success()
        .stdout("x := 3\n(a f_ (b g_ c))\n");
}

#[test]
fn core_prints_the_desugared_program() {
    xetal()
        .args(["core", "-e", "u:s_quare := { _r * _r }; u:s_quare 7"])
        .assert()
        .success()
        .stdout("(def u:s_quare (lam _r (app2 #* _r _r)))\n(eval (app u:s_quare 7))\n");
}

#[test]
fn eval_prints_each_expression_value() {
    xetal()
        .args(["eval", "-e", "1 + 2"])
        .assert()
        .success()
        .stdout("3\n");
    xetal()
        .args([
            "eval",
            "-e",
            "u:s_quare := { _r * _r }; u:s_quare 7\n10 - 3",
        ])
        .assert()
        .success()
        .stdout("49\n7\n");
}

#[test]
fn runtime_errors_keep_earlier_output() {
    xetal()
        .args(["eval", "-e", "1; 2 / 0; 3"])
        .assert()
        .code(1)
        .stdout("1\n")
        .stderr(starts_with("error[division-by-zero]: "));
}

#[test]
fn warnings_go_to_stderr_without_failing() {
    xetal()
        .args(["eval", "-e", "u:f_ := { r_ev x -> r_ev x }; 1"])
        .assert()
        .success()
        .stdout("1\n")
        .stderr(starts_with("warning[shadows-builtin]: "));
}

#[test]
fn run_and_bare_file_execute_scripts() {
    let dir = std::env::temp_dir().join(format!("xetal-cli-run-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("fact.xtl");
    std::fs::write(
        &path,
        "#!/usr/bin/env xetal\nu:f_act := { n -> n <= 1 ? 1; n * u:f_act n - 1 }\nu:f_act 5\n",
    )
    .unwrap();
    let file = path.to_str().unwrap();
    xetal()
        .args(["run", file])
        .assert()
        .success()
        .stdout("120\n");
    xetal().arg(file).assert().success().stdout("120\n");
    std::fs::remove_dir_all(&dir).unwrap();
}
