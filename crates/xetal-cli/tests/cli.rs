use assert_cmd::Command;
use xetal_base::LANG_NAME;

fn xetal() -> Command {
    Command::new(env!("CARGO_BIN_EXE_xetal"))
}

#[test]
fn version_prints_language_name_and_version() {
    let expected = format!("{LANG_NAME} {}\n", env!("CARGO_PKG_VERSION"));
    xetal().arg("--version").assert().success().stdout(expected);
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
