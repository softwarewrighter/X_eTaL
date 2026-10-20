//! The Rendered pane draws a notation call (MC32) as its `draws:` macro
//! wrote it: the program is loaded as a run loads it (the page's own
//! libraries from the store), each call that draws a notation side has
//! its strings replaced by one segment of class `Notation` holding the
//! LaTeX, for KaTeX to typeset; a program that does not load is drawn
//! as typed.

use std::sync::{Arc, OnceLock};

use xetal_play::{Class, add_library, decorated};
use xetal_store::{Memory, install};

fn store() {
    static STORE: OnceLock<Arc<Memory>> = OnceLock::new();
    STORE.get_or_init(|| {
        let store = Arc::new(Memory::default());
        install(store.clone());
        store
    });
}

const EQ: &str = "## takes: notation notation\n## draws: t_ex<\n## uses: parsed\nm:e_q< := { l r -> f_ormat t_ally r }\n\n## takes: text text\nm:t_ex< := { l r -> \"\\\"\" c_at l c_at \" = \" c_at r c_at \"\\\"\" }\n";

#[test]
fn a_notation_call_is_drawn_as_its_latex() {
    store();
    add_library("Eqn.xtlm", EQ).unwrap();
    let src = "\"e:\" u_se< \"Eqn\"\ny := \"L\" e:e_q< \"a + b\"\n";
    let segments = decorated(src);
    let notation: Vec<&str> = segments
        .iter()
        .filter(|s| s.class == Class::Notation)
        .map(|s| s.text.as_str())
        .collect();
    assert_eq!(notation, ["", "L = a + b"]);
    let text: String = segments.iter().map(|s| s.text.as_str()).collect();
    assert!(!text.contains("\"a + b\""), "{text}");
}

#[test]
fn a_program_that_does_not_load_is_drawn_as_typed() {
    store();
    let src = "y := \"L\" e:e_q< \"a + b\"\n";
    let segments = decorated(src);
    assert!(segments.iter().all(|s| s.class != Class::Notation));
    let text: String = segments.iter().map(|s| s.text.as_str()).collect();
    assert!(text.contains("\"a + b\""), "{text}");
}
