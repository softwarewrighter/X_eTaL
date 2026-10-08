//! A page's own library (ask D16): added to the store a run reads, so
//! a program imports it by name, and the program after its macros
//! expand, as `xetal expand` prints it.

use std::sync::{Arc, OnceLock};

use xetal_play::{Memory, add_library, expanded, install, run};

/// One store for every test here (the store in use is global).
fn store() {
    static STORE: OnceLock<Arc<Memory>> = OnceLock::new();
    STORE.get_or_init(|| {
        let store = Arc::new(Memory::default());
        install(store.clone());
        store
    });
}

const PLUS: &str = "## One more.\nm:p_lus1< := { @ b -> \"1 + \" c_at b }\n";
const PROGRAM: &str = "\"p:\" u_se< \"Plus\"\n@ p:p_lus1< \"41\"\n";

#[test]
fn a_program_runs_with_the_pages_own_macro_library() {
    store();
    add_library("Plus.xtlm", PLUS).unwrap();
    assert_eq!(run(PROGRAM, 1).out, "42\n");
}

#[test]
fn the_program_after_expansion_shows_what_the_macro_wrote() {
    store();
    add_library("Plus.xtlm", PLUS).unwrap();
    let text = expanded(PROGRAM).unwrap();
    assert!(text.contains("1 + 41"), "{text}");
}
