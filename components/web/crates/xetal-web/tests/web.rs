//! The live demo's keys and programs (the language is xetal-play's).

use std::sync::Arc;

use xetal_store::Store;
use xetal_web::{Action, DEMOS, action, choices, open};

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
    let store = Arc::new(xetal_store::Memory::default());
    store.put("MyLib.xtl", "l:t_wo := { 2 }").unwrap();
    xetal_store::install(store);
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
