//! A session binds a name once, as a file does (M1), and only a session
//! may unbind one with []E_X to bind it again (M4).

use xetal_repl::{Reply, Session};

fn said(session: &mut Session, line: &str) -> (String, String) {
    match session.feed(line) {
        Reply::Done { out, err } => (out, err),
        Reply::More => panic!("{line}: more input expected"),
    }
}

#[test]
fn binding_a_name_again_is_an_error_that_names_the_way_out() {
    let mut s = Session::new("-e", 1);
    said(&mut s, "a := 42");
    let (_, err) = said(&mut s, "a := 43");
    assert!(err.contains("error[rebind]"), "{err}");
    assert_eq!(said(&mut s, "a").0, "42\n");
}

#[test]
fn erase_unbinds_a_name_so_it_can_be_bound_again() {
    let mut s = Session::new("-e", 1);
    said(&mut s, "a := 42");
    let (out, err) = said(&mut s, "[]E_X \"a\"");
    assert_eq!((out.as_str(), err.as_str()), ("", ""));
    let (_, err) = said(&mut s, "a");
    assert!(err.contains("error[erased]"), "{err}");
    said(&mut s, "a := 43");
    assert_eq!(said(&mut s, "a").0, "43\n");
}
