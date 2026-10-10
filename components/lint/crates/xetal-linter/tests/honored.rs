//! The warnings `xetal run` prints honor `# lint:` comments (S12) in
//! the file each warning is in; nothing else changes (L7).

use std::path::PathBuf;

/// What `xetal run` would print for program `text`, named `name`.
fn printed(name: &str, text: &str) -> Vec<String> {
    let loaded = xetal_program::load(name, text).expect("loads");
    let warnings = xetal_lint::warnings(&loaded.program);
    let kept = xetal_linter::honored(&loaded.sources, &loaded.program, warnings);
    kept.into_iter().map(|d| d.code).collect()
}

#[test]
fn an_allowed_warning_is_not_printed() {
    let text =
        "# lint: allow shadows-builtin\nu:a_pply := { r_ev -> r_ev 1 2 3 }\n{ r_ev -> r_ev } 4";
    assert_eq!(printed("t.xtl", text), ["shadows-builtin"]);
    assert!(printed("t.xtl", "# lint-file: allow suspicious\n{ r_ev -> r_ev } 4").is_empty());
}

#[test]
fn deny_and_warn_still_print_a_warning() {
    let text = "# lint-file: deny shadows-builtin\n{ r_ev -> r_ev } 4";
    assert_eq!(printed("t.xtl", text), ["shadows-builtin"]);
}

#[test]
fn style_lints_are_never_printed_by_run() {
    assert!(printed("t.xtl", "# lint-file: warn all\nCount! := 0").is_empty());
}

/// A directory of its own for `test` (tests run in parallel).
fn dir(test: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("xetal-honored-{}-{test}", std::process::id()));
    std::fs::create_dir_all(&d).expect("temp dir");
    d
}

#[test]
fn a_library_directive_covers_its_own_warnings_where_imported() {
    let d = dir("library");
    let lib = "# lint-file: allow deprecated-private\nd_ouble := { _r * 2 }\nl:q_uad := { d_ouble d_ouble _r }\n";
    std::fs::write(d.join("Lib.xtl"), lib).expect("write");
    let main = d.join("main.xtl");
    let text = "\"q:\" u_se< \"Lib\"\nq:q_uad 3";
    assert!(printed(main.to_str().unwrap(), text).is_empty());
    let bare = lib.replace("# lint-file: allow deprecated-private\n", "\n");
    std::fs::write(d.join("Lib.xtl"), bare).expect("write");
    assert_eq!(
        printed(main.to_str().unwrap(), text),
        ["deprecated-private"]
    );
    std::fs::remove_dir_all(&d).ok();
}
