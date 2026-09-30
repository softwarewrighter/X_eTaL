//! The live demo's keys and programs (the language is xetal-play's).

use xetal_web::{Action, DEMOS, action};

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
