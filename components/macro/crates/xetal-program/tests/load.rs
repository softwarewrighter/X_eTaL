//! Loading programs with standard and local libraries.

use xetal_program::{load, located};

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("xetal-program-{}-{name}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn a_program_without_imports_loads_as_before() {
    let loaded = load("-e", "1 + 2").unwrap();
    assert_eq!(loaded.sources.file_count(), 1);
    assert_eq!(
        loaded.program.to_string(),
        xetal_core::lower("1 + 2").unwrap().to_string()
    );
}

#[test]
fn a_library_beside_the_program_is_found() {
    let dir = scratch("beside");
    std::fs::write(dir.join("Two.xtl"), "l:t_wo := { _r * 2 }\n").unwrap();
    let main = dir.join("main.xtl");
    let loaded = load(
        &main.display().to_string(),
        "\"t:\" u_se< \"Two\"\nt:t_wo 21\n",
    )
    .unwrap();
    assert_eq!(loaded.sources.file_count(), 2);
}

#[test]
fn an_error_in_a_library_is_located_there() {
    let dir = scratch("err");
    std::fs::write(dir.join("Bad.xtl"), "l:f_ := { _r }\n1 + 2\n").unwrap();
    let main = dir.join("main.xtl").display().to_string();
    let err = load(&main, "\"b:\" u_se< \"Bad\"\n").unwrap_err();
    assert_eq!(err.code, "expression-in-library");
    assert!(err.to_string().ends_with("Bad.xtl:2:1"), "{err}");
}

#[test]
fn located_leaves_one_file_alone() {
    let loaded = load("-e", "1").unwrap();
    let d = xetal_base::Diagnostic::new("x", "y").with_span(xetal_base::Span::new(0, 1));
    assert_eq!(
        located(&loaded.sources, d.clone()).to_string(),
        d.to_string()
    );
}
