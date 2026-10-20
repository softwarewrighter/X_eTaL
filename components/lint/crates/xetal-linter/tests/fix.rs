//! Machine-applicable suggestions: a finding's edit to the file as
//! written, `--fix` applying them, and `--format json` for editors.

use std::path::PathBuf;

use xetal_linter::{Edit, Request, command, fixed, lint};

const REDUNDANT: &str = "u:f_ := { v -> '{ x -> n_eg x } e_ach v }";

#[test]
fn a_redundant_lambda_carries_its_edit() {
    let found = lint("t.xtl", REDUNDANT, &[]).expect("loads");
    let edit = &found[0].fix[0];
    assert_eq!(&REDUNDANT[edit.start..edit.end], "{ x -> n_eg x }");
    assert_eq!(edit.replacement, "n_eg");
}

#[test]
fn findings_without_a_sure_edit_carry_none() {
    let found = lint("t.xtl", "u:f_ := { a b -> a }", &[]).expect("loads");
    assert!(found[0].fix.is_empty());
}

#[test]
fn edits_apply_from_the_end_and_are_counted() {
    let text = "v := 1 2 3\n'{ x -> x } e_ach v\n'{ a -> n_eg a } e_ach v";
    let found = lint("t.xtl", text, &[]).expect("loads");
    let (out, n) = fixed(text, &found);
    assert_eq!(out, "v := 1 2 3\n'i_d e_ach v\n'n_eg e_ach v");
    assert_eq!(n, 2);
}

#[test]
fn an_unused_local_is_fixed_to_a_discard() {
    let text = "u:s_ay! := { v -> shown := p_rint! v\n  v }";
    let found = lint("t.xtl", text, &[]).expect("loads");
    let (out, n) = fixed(text, &found);
    assert_eq!(out, "u:s_ay! := { v -> _ := p_rint! v\n  v }");
    assert_eq!(n, 1);
}

#[test]
fn overlapping_edits_apply_once() {
    let text = "u:f_ := { v -> 1 }";
    let edits = [vec![Edit::new(0, 4, "a")], vec![Edit::new(2, 6, "b")]];
    assert_eq!(
        xetal_linter::apply(text, &edits),
        ("a := { v -> 1 }".into(), 1)
    );
}

/// A file of this test's own, in the system's temporary directory.
fn scratch(name: &str, text: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("xetal-fix-{}-{name}.xtl", std::process::id()));
    std::fs::write(&path, text).unwrap();
    path
}

#[test]
fn fix_rewrites_the_file_and_reports_what_is_left() {
    let path = scratch(
        "rewrites",
        &format!("{REDUNDANT}\nu:g_ := {{ a b -> a }}\n"),
    );
    let name = path.to_string_lossy().to_string();
    let text = std::fs::read_to_string(&path).unwrap();
    let req = Request {
        input: Some((name, text)),
        fix: true,
        allow_dirty: true,
        ..Request::default()
    };
    let mut err = Vec::new();
    command(&req, &mut Vec::new(), &mut err).unwrap();
    let after = std::fs::read_to_string(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    assert_eq!(
        after,
        "u:f_ := { v -> 'n_eg e_ach v }\nu:g_ := { a b -> a }\n"
    );
    let err = String::from_utf8(err).unwrap();
    assert!(err.contains("fixed 1 finding(s) in "), "{err}");
    assert!(err.contains("warning[unused-binding]"), "{err}");
    assert!(!err.contains("redundant-lambda"), "{err}");
}

#[test]
fn fix_needs_a_file() {
    let req = Request {
        input: Some(("-e".into(), REDUNDANT.into())),
        fix: true,
        ..Request::default()
    };
    let got = command(&req, &mut Vec::new(), &mut Vec::new());
    assert_eq!(got.unwrap_err().code, "fix-needs-file");
}

#[test]
fn json_is_one_object_a_line_on_standard_output() {
    let req = Request {
        input: Some(("t.xtl".into(), format!("{REDUNDANT}\nCount! := 0"))),
        json: true,
        ..Request::default()
    };
    let (mut out, mut err) = (Vec::new(), Vec::new());
    command(&req, &mut out, &mut err).unwrap();
    assert!(err.is_empty());
    let out = String::from_utf8(out).unwrap();
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 2, "{out}");
    assert_eq!(
        lines[0],
        concat!(
            r#"{"lint":"redundant-lambda","group":"complexity","level":"warn","#,
            r#""file":"t.xtl","line":1,"col":17,"end_line":1,"end_col":32,"#,
            r#""message":"this lambda only passes its arguments to `n_eg`: write `'n_eg`","#,
            r#""fix":[{"start":16,"end":31,"replacement":"n_eg"}]}"#
        )
    );
    assert!(
        lines[1].contains(r#""lint":"mutable-constant""#),
        "{}",
        lines[1]
    );
    assert!(lines[1].contains(r#""fix":[{"#), "{}", lines[1]);
}

#[test]
fn json_escapes_what_json_must() {
    assert_eq!(
        xetal_linter::json_string("a\"b\\c\nd\u{1}"),
        r#""a\"b\\c\nd\u0001""#
    );
}

#[test]
fn a_finding_spans_to_its_end_on_a_later_line() {
    let text = "u:f_ := { v -> '{ x ->\n  n_eg x } e_ach v }";
    let found = lint("t.xtl", text, &[]).expect("loads");
    let f = &found[0];
    assert_eq!((f.line, f.col, f.end_line, f.end_col), (1, 17, 2, 11));
}

/// `--fix` on `path` without `--allow-dirty`: the error code, if any.
fn fix_strict(path: &std::path::Path) -> Option<String> {
    let name = path.to_string_lossy().to_string();
    let text = std::fs::read_to_string(path).unwrap();
    let req = Request {
        input: Some((name, text)),
        fix: true,
        ..Request::default()
    };
    command(&req, &mut Vec::new(), &mut Vec::new())
        .err()
        .map(|d| d.code)
}

#[test]
fn fix_refuses_a_file_git_cannot_restore() {
    let path = scratch("dirty", REDUNDANT);
    let got = fix_strict(&path);
    let after = std::fs::read_to_string(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    assert_eq!(got.as_deref(), Some("fix-dirty"));
    assert_eq!(after, REDUNDANT, "left as it was");
}

#[test]
fn fix_rewrites_a_committed_file_but_not_a_changed_one() {
    let dir = std::env::temp_dir().join(format!("xetal-fix-git-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let git = |args: &[&str]| {
        let ok = std::process::Command::new("git")
            .args([
                "-c",
                "user.name=t",
                "-c",
                "user.email=t@t",
                "-c",
                "commit.gpgsign=false",
            ])
            .arg("-C")
            .arg(&dir)
            .args(args)
            .output()
            .unwrap();
        assert!(ok.status.success(), "git {args:?}");
    };
    let path = dir.join("p.xtl");
    std::fs::write(&path, format!("{REDUNDANT}\n")).unwrap();
    git(&["init", "-q"]);
    git(&["add", "p.xtl"]);
    git(&["commit", "-q", "-m", "p"]);
    assert_eq!(fix_strict(&path), None, "committed: fixed");
    let changed = std::fs::read_to_string(&path).unwrap();
    std::fs::write(&path, format!("{changed}'{{ x -> n_eg x }} e_ach 1 2\n")).unwrap();
    let got = fix_strict(&path);
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(got.as_deref(), Some("fix-dirty"), "changed since: refused");
}

#[test]
fn a_mutable_constant_is_renamed_everywhere_together() {
    let text = "Count! := 0\nCount! := Count! + 1\nu:f_ := { v -> v + Count! }\nu:f_ Count!";
    let found = lint("t.xtl", text, &[]).expect("loads");
    let (out, n) = fixed(text, &found);
    let want = "count! := 0\ncount! := count! + 1\nu:f_ := { v -> v + count! }\nu:f_ count!";
    assert_eq!((out.as_str(), n), (want, 1));
}

#[test]
fn a_rename_onto_a_name_in_use_has_no_fix() {
    let found = lint("t.xtl", "Count! := 0\ncount! := 1\nCount! + count!", &[]).expect("loads");
    assert_eq!(found[0].lint.name, "mutable-constant");
    assert!(found[0].fix.is_empty());
}

#[test]
fn a_function_with_an_effect_gains_its_bang_at_every_call() {
    let text = "u:s_ay := { v -> p_rint! v }\nu:s_ay 1\n'u:s_ay e_ach 2 3";
    let found = lint("t.xtl", text, &[]).expect("loads");
    let (out, n) = fixed(text, &found);
    let want = "u:s_ay! := { v -> p_rint! v }\nu:s_ay! 1\n'u:s_ay! e_ach 2 3";
    assert_eq!((out.as_str(), n), (want, 1));
}

/// A code string on a side the macro takes as text stops the program
/// from loading with `string-kind` (B2), an error that carries its fix:
/// `--fix` drops the `c` as it would a finding's edit, then lints the
/// file that now loads.
#[test]
fn fix_drops_the_c_of_a_code_string_on_a_text_side() {
    let path = scratch("text-side", "@ f_ormat< c\"from 3..4\"\n");
    let name = path.to_string_lossy().to_string();
    let text = std::fs::read_to_string(&path).unwrap();
    let req = Request {
        input: Some((name, text)),
        fix: true,
        allow_dirty: true,
        ..Request::default()
    };
    let mut err = Vec::new();
    command(&req, &mut Vec::new(), &mut err).unwrap();
    let after = std::fs::read_to_string(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    assert_eq!(after, "@ f_ormat< \"from 3..4\"\n");
    let err = String::from_utf8(err).unwrap();
    assert!(err.contains("fixed 1 finding(s) in "), "{err}");
    assert!(!err.contains("string-kind"), "{err}");
}
