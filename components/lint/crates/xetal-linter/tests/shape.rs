//! The shape lint through the driver: `shape-mismatch` on by default
//! and placed at the application in the file; `shape-check-proven`
//! only on request; a `# lint:` comment allows either.

use xetal_linter::{command_line, lint};

fn found(text: &str, warn: &[&str]) -> Vec<(String, usize, usize)> {
    let warn: Vec<String> = warn.iter().map(|s| s.to_string()).collect();
    let rules = command_line([&[], &warn, &[]]).unwrap();
    let findings = lint("t.xtl", text, &rules).expect("loads");
    findings
        .into_iter()
        .map(|f| (f.lint.name.to_string(), f.line, f.col))
        .collect()
}

const MISMATCH: &str = "X := 2 3 r_eshape 0\nX + 3 2 r_eshape 1\n";
const PROVEN: &str = "X := 2 3 r_eshape 0\n2 3 r_equireShape X\n";

#[test]
fn a_mismatch_is_found_by_default_at_its_place() {
    assert_eq!(found(MISMATCH, &[]), [("shape-mismatch".to_string(), 2, 1)]);
}

#[test]
fn a_proven_check_is_pedantic() {
    assert!(found(PROVEN, &[]).is_empty());
    let got = found(PROVEN, &["pedantic"]);
    assert_eq!(got, [("shape-check-proven".to_string(), 2, 1)]);
}

#[test]
fn a_comment_allows_it() {
    let text = "X := 2 3 r_eshape 0\nX + 3 2 r_eshape 1 # lint: allow shape-mismatch\n";
    assert!(found(text, &[]).is_empty());
}
