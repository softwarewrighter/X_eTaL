//! A debugging session over small programs: where it stops, at what
//! depth, why; breakpoints by line; over, in, out and continue; the call
//! stack; the program's output as it runs.

use xetal_debug::{Debugger, Reason, Stop};

/// Load and check `src` as `t.xtl`, then run `drive` on a session.
fn session(src: &str, drive: impl FnOnce(&mut Debugger) -> Vec<Stop>) -> (Vec<Stop>, String) {
    let mut loaded = xetal_program::load("t.xtl", src).expect("loads");
    xetal_types::check_program(&mut loaded.program).expect("checks");
    let mut out = Vec::new();
    let stops = {
        let mut dbg = Debugger::new(&loaded.program, &loaded.sources, &mut out, 1);
        drive(&mut dbg)
    };
    (stops, String::from_utf8_lossy(&out).into_owned())
}

fn lines(stops: &[Stop]) -> Vec<Option<usize>> {
    stops
        .iter()
        .map(|s| s.place.as_ref().map(|p| p.line))
        .collect()
}

const PROGRAM: &str = "u:d_ouble := { x ->
  y := x * 2
  y
}
a := 1 + 2
b := u:d_ouble a
a + b";

#[test]
fn it_stops_before_the_first_statement_and_steps_over_statements() {
    let (stops, out) = session(PROGRAM, |d| {
        let mut s = vec![d.start()];
        for _ in 0..4 {
            s.push(d.step_over());
        }
        s
    });
    assert_eq!(stops[0].reason, Reason::Entry);
    assert_eq!(lines(&stops), [Some(5), Some(6), Some(7), None, None]);
    assert_eq!(stops[3].reason, Reason::End);
    assert_eq!(out, "9\n", "the program ran to its end and printed");
}

#[test]
fn step_in_enters_the_call_and_step_out_leaves_it() {
    let (stops, _) = session(PROGRAM, |d| {
        d.set_breakpoints("t.xtl", &[6]);
        let mut s = vec![d.start(), d.resume()];
        while s.last().unwrap().depth == 0 && s.len() < 50 {
            s.push(d.step_in());
        }
        s.push(d.step_over());
        s.push(d.step_out());
        s
    });
    assert_eq!(stops[1].reason, Reason::Breakpoint);
    assert_eq!(stops[1].place.as_ref().unwrap().line, 6);
    let inside = &stops[stops.len() - 3];
    assert_eq!(
        (inside.place.as_ref().unwrap().line, inside.depth),
        (2, 1),
        "{stops:?}"
    );
    let next = &stops[stops.len() - 2];
    assert_eq!((next.place.as_ref().unwrap().line, next.depth), (3, 1));
    let out = stops.last().unwrap();
    assert_eq!(
        (out.place.as_ref().unwrap().line, out.depth),
        (7, 0),
        "back at the top level"
    );
}

#[test]
fn a_breakpoint_in_a_function_is_hit_on_each_call_and_shows_the_stack() {
    let src = "u:f_act := { n ->
  n <= 1 ? 1
  n * u:f_act n - 1
}
u:f_act 3";
    let (stops, out) = session(src, |d| {
        d.set_breakpoints("t.xtl", &[2]);
        let mut s = vec![d.start()];
        for _ in 0..4 {
            s.push(d.resume());
        }
        s
    });
    let hits: Vec<usize> = stops
        .iter()
        .filter(|s| s.reason == Reason::Breakpoint)
        .map(|s| s.depth)
        .collect();
    assert_eq!(hits, [1, 2, 3], "one hit per call, deeper each time");
    assert_eq!(stops.last().unwrap().reason, Reason::End);
    assert_eq!(out, "6\n");
}

#[test]
fn the_stack_names_each_call_innermost_first() {
    let src = "u:f_ := { x -> x + 1 }
u:g_ := { x -> u:f_ x * 2 }
u:g_ 5";
    let (_, _) = session(src, |d| {
        d.set_breakpoints("t.xtl", &[1]);
        d.start();
        let stop = d.resume();
        assert_eq!(stop.reason, Reason::Breakpoint);
        let stack = d.stack();
        let names: Vec<&str> = stack.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, ["u:f_", "u:g_", "(top level)"], "{stack:?}");
        let lines: Vec<usize> = stack.iter().map(|f| f.place.line).collect();
        assert_eq!(lines, [1, 2, 3]);
        vec![stop]
    });
}

#[test]
fn an_error_stops_the_session_with_it() {
    let (stops, _) = session("1 / 0", |d| vec![d.start(), d.resume()]);
    match &stops[1].reason {
        Reason::Error(e) => assert_eq!(e.code, "division-by-zero"),
        other => panic!("{other:?}"),
    }
}
