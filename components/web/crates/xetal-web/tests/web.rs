//! The live demo's keys and programs (the language is xetal-play's).

use std::sync::{Arc, OnceLock};

use xetal_store::{Memory, Store};
use xetal_web::{Action, DEMOS, action, choices, open, seed};

/// One store for every test (the store is global, and tests run at
/// once), with the demos' own libraries seeded, as the page does.
fn store() -> Arc<Memory> {
    static STORE: OnceLock<Arc<Memory>> = OnceLock::new();
    STORE
        .get_or_init(|| {
            let store = Arc::new(Memory::default());
            xetal_store::install(store.clone());
            seed();
            store
        })
        .clone()
}

#[test]
fn run_and_zoom_keys() {
    assert_eq!(action("Enter", true), Some(Action::Run));
    assert_eq!(action("r", true), Some(Action::Run));
    assert_eq!(action(".", true), Some(Action::Zoom));
    assert_eq!(action("Enter", false), None);
    assert_eq!(action("t", true), None, "Ctrl-T is the browser's");
}

#[test]
fn every_demo_checks() {
    store();
    for demo in DEMOS {
        let lines = xetal_play::check(demo.text);
        assert!(
            !lines.iter().any(|l| l.starts_with("error[")),
            "{}: {lines:?}",
            demo.name
        );
    }
    assert_eq!(DEMOS[0].name, "tour.xtl");
    assert_eq!((DEMOS[1].name, DEMOS[1].text), ("(empty)", ""));
}

#[test]
fn open_offers_the_demos_the_libraries_and_the_saved_files() {
    store().put("MyLib.xtl", "l:t_wo := { 2 }").unwrap();
    let list = choices(&["MyLib.xtl".to_string()]);
    let label = |v: &str| {
        list.iter()
            .find(|(_, value, _)| value == v)
            .map(|(g, _, l)| (*g, l.clone()))
    };
    assert_eq!(label("demo:0"), Some(("Demos", "tour.xtl".into())));
    assert_eq!(label("lib:Stats"), Some(("Libraries", "Stats.xtl".into())));
    assert_eq!(
        label("file:MyLib.xtl"),
        Some(("Your files", "MyLib.xtl".into()))
    );
    assert_eq!(open("lib:Stats").unwrap().0, "Stats.xtl");
    assert!(open("lib:Stats").unwrap().1.contains("l:m_ean"));
    assert_eq!(
        open("file:MyLib.xtl").unwrap(),
        ("MyLib.xtl".into(), "l:t_wo := { 2 }".into())
    );
    assert!(open("file:nothing.xtl").is_none());
}

#[test]
fn the_demos_own_libraries_are_among_your_files_and_edits_are_kept() {
    let store = store();
    assert!(store.get("Hello.xtl").unwrap().contains("l:h_ello"));
    assert!(store.get("Greetings.xtl").unwrap().contains("l:g_reet"));
    store
        .put("Greetings.xtl", "l:g_reet := { n -> n }")
        .unwrap();
    seed();
    assert_eq!(
        store.get("Greetings.xtl").unwrap(),
        "l:g_reet := { n -> n }"
    );
    let hello = DEMOS
        .iter()
        .find(|d| d.name == "hello-library.xtl")
        .unwrap();
    assert!(
        xetal_play::run(hello.text, 1)
            .out
            .contains("hello X\u{332}\u{1d49}T\u{1d43}L")
    );
}

/// Until the live demo has a Draw pane, no demo it lists draws.
#[test]
fn no_listed_demo_draws_until_there_is_a_draw_pane() {
    for demo in DEMOS {
        assert!(!demo.text.contains("[]S_HOW"), "{} draws", demo.name);
    }
}
