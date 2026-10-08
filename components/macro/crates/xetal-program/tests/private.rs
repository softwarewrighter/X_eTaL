//! No library here has a bare top-level function (PN2): the gate treats
//! the deprecated-private warning as an error. A new one is fixed with
//! `xetal migrate FILE`.

use std::fs;
use std::path::Path;

use xetal_program::{is_library, load_library};

const DIRS: [&str; 3] = ["lib", "userlibs", "demos/rosetta"];

#[test]
fn no_library_here_has_a_bare_function() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..");
    let mut bare = Vec::new();
    for dir in DIRS {
        for entry in fs::read_dir(root.join(dir)).unwrap() {
            let path = entry.unwrap().path();
            let text = fs::read_to_string(&path).unwrap_or_default();
            if path.extension().is_none_or(|e| e != "xtl") || !is_library(&text) {
                continue;
            }
            let name = path.to_string_lossy().to_string();
            let loaded = load_library(&name, &text).unwrap();
            let warnings = xetal_lint::warnings(&loaded.program);
            let found = warnings.iter().filter(|d| d.code == "deprecated-private");
            bare.extend(found.map(|d| format!("{name}: {}", d.message)));
        }
    }
    assert!(
        bare.is_empty(),
        "run xetal migrate on:\n{}",
        bare.join("\n")
    );
}
