//! A macro library opened in the live demo is checked as one: its
//! macros' types are shown, as `xetal type` gives them (the text has no
//! file name in the page, so what it defines decides).

use std::sync::{Arc, OnceLock};

use xetal_play::{check, run};
use xetal_store::{Memory, install};

/// One store for every test here (the store in use is global).
fn store() {
    static STORE: OnceLock<Arc<Memory>> = OnceLock::new();
    STORE.get_or_init(|| {
        let store = Arc::new(Memory::default());
        install(store.clone());
        store
    });
}

const TWICE: &str = "## Twice the text.\nm:t_wice< := { @ b -> b c_at \" \" c_at b }\n";

#[test]
fn a_macro_library_shows_its_macros_and_their_types() {
    store();
    assert_eq!(check(TWICE), ["m:t_wice< : Unit -> Char -> Char"]);
    assert_eq!(run(TWICE, 1).out, "m:t_wice< : Unit -> Char -> Char\n");
}

#[test]
fn the_standard_macro_libraries_check_as_the_command_line_does() {
    store();
    for name in ["Combinators", "Macros"] {
        let text = xetal_libs::standard_macros(name).expect("a standard macro library");
        let types = check(text);
        assert!(!types.is_empty(), "{name}");
        assert!(
            types.iter().all(|t| t.starts_with("m:")),
            "{name}: {types:?}"
        );
    }
}

#[test]
fn the_system_macros_check_under_their_own_rules() {
    store();
    let text = xetal_libs::standard_macros("System").expect("System.xtlm");
    let types = check(text);
    assert!(
        types.iter().any(|t| t.starts_with("s:i_f< : ")),
        "{types:?}"
    );
    assert!(types.iter().all(|t| !t.starts_with("error")), "{types:?}");
}
