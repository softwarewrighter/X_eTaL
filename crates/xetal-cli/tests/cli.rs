use assert_cmd::Command;
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
