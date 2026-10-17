//! This repository's own programs and libraries lint clean at the
//! default levels, so the gate treats a new finding as an error: fix
//! it, or say why with a `# lint: allow NAME` comment (docs/lint.md).

use std::fs;
use std::path::{Path, PathBuf};

const DIRS: [&str; 3] = ["lib", "userlibs", "demos"];

/// Every `.xtl` and `.xtlm` file under `dir`, at any depth.
fn programs(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.is_dir() {
            programs(&path, out);
        } else if path.extension().is_some_and(|e| e == "xtl" || e == "xtlm") {
            out.push(path);
        }
    }
}

#[test]
fn every_library_and_demo_lints_clean() {
    // From the repository's root, as `xetal run` is run there: libraries
    // are found in userlibs/ beside it. This binary has only this test,
    // so changing the directory affects nothing else.
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..");
    std::env::set_current_dir(&root).unwrap();
    let mut files = Vec::new();
    DIRS.iter().for_each(|d| programs(Path::new(d), &mut files));
    assert!(files.len() > 50, "found only {} programs", files.len());
    let mut found = Vec::new();
    for path in files {
        let name = path.to_string_lossy().to_string();
        let text = fs::read_to_string(&path).unwrap();
        match xetal_linter::lint(&name, &text, &[]) {
            Ok(findings) => found.extend(findings.iter().map(ToString::to_string)),
            Err(e) => found.push(format!("{name}: does not load: {e}")),
        }
    }
    assert!(
        found.is_empty(),
        "xetal lint findings:\n{}",
        found.join("\n")
    );
}
