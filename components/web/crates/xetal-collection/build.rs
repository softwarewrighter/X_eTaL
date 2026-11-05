//! Embeds vendor/libraries (scripts/sync-libraries.sh): `LIBRARIES`
//! (file name, text) from libs/, `DEMOS` (Library/demo.xtl, text) from
//! demos/, `DATA` (file name, text) from data/, and `PIN`, the commit
//! they come from.

use std::fmt::Write;
use std::path::{Path, PathBuf};

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../vendor/libraries");
    println!("cargo:rerun-if-changed={}", root.display());
    let pin = std::fs::read_to_string(root.join("PIN")).unwrap_or_default();
    let mut out = format!(
        "/// The X_eTaL-libraries commit the collection comes from.\npub const PIN: &str = {:?};\n",
        pin.trim()
    );
    let libs = files(&root.join("libs"), "");
    out.push_str(&table(
        "LIBRARIES",
        "The libraries: (file name, text)",
        &libs,
    ));
    let mut demos = Vec::new();
    for dir in files(&root.join("demos"), "") {
        let name = dir.0.clone();
        demos.extend(files(&dir.1, &format!("{name}/")));
    }
    out.push_str(&table(
        "DEMOS",
        "The demos: (Library/demo.xtl, text)",
        &demos,
    ));
    let data = files(&root.join("data"), "");
    out.push_str(&table(
        "DATA",
        "The demos' data files: (file name, text)",
        &data,
    ));
    let target = Path::new(&std::env::var("OUT_DIR").unwrap_or_default()).join("collection.rs");
    std::fs::write(target, out).unwrap_or_else(|e| panic!("cannot write collection.rs: {e}"));
}

/// The entries of `dir`, sorted, as (prefix + name, path).
fn files(dir: &Path, prefix: &str) -> Vec<(String, PathBuf)> {
    let mut out: Vec<(String, PathBuf)> = std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok().map(|e| e.path()))
                .map(|p| {
                    (
                        format!(
                            "{prefix}{}",
                            p.file_name().unwrap_or_default().to_string_lossy()
                        ),
                        p,
                    )
                })
                .collect()
        })
        .unwrap_or_default();
    out.sort();
    out
}

/// A constant `name` of (name, include_str!(path)).
fn table(name: &str, doc: &str, entries: &[(String, PathBuf)]) -> String {
    let mut out = format!("/// {doc}.\npub const {name}: &[(&str, &str)] = &[\n");
    for (label, path) in entries {
        println!("cargo:rerun-if-changed={}", path.display());
        let full = path.canonicalize().unwrap_or_else(|_| path.clone());
        let _ = writeln!(
            out,
            "    ({label:?}, include_str!({:?})),",
            full.display().to_string()
        );
    }
    out.push_str("];\n");
    out
}
