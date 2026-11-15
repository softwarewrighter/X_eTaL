//! Time travel (Saga 43, step debug-time-travel): stepping back, reverse
//! continue and run to step k reach the states the forward run reached,
//! `!` variables included; output is shown once; checkpoints stay few
//! on a long run.

use xetal_debug::{Debugger, Stop};

/// Load and check `src` as `t.xtl`, then run `drive` on a session.
fn session<T>(src: &str, drive: impl FnOnce(&mut Debugger) -> T) -> (T, String) {
    let mut loaded = xetal_program::load("t.xtl", src).expect("loads");
    xetal_types::check_program(&mut loaded.program).expect("checks");
    let mut out = Vec::new();
    let got = {
        let mut dbg = Debugger::new(&loaded.program, &loaded.sources, &mut out, 1);
        drive(&mut dbg)
    };
    (got, String::from_utf8_lossy(&out).into_owned())
}

/// What a stop looks like: the step, the line, and `total!` there.
fn state(d: &mut Debugger, s: &Stop) -> (usize, Option<usize>, String) {
    let total = d
        .scopes()
        .into_iter()
        .flat_map(|s| s.vars)
        .find(|v| v.name == "total!")
        .map(|v| v.value)
        .unwrap_or_default();
    (d.now(), s.place.as_ref().map(|p| p.line), total)
}

const PROGRAM: &str = "total! := 0
u:a_dd := { n ->
  total! := total! + n
  p_rint! total!
  total!
}
u:a_dd 1
u:a_dd 2
x := u:a_dd 3
total! := total! * 10
total!";

#[test]
fn back_then_forward_reaches_the_same_states() {
    let ((forward, back, again), out) = session(PROGRAM, |d| {
        let first = d.start();
        let mut forward = vec![state(d, &first)];
        for _ in 0..5 {
            let s = d.step_over();
            forward.push(state(d, &s));
        }
        let mut back = Vec::new();
        for _ in 0..5 {
            let s = d.step_back();
            back.push(state(d, &s));
        }
        let mut again = Vec::new();
        for _ in 0..5 {
            let s = d.step_over();
            again.push(state(d, &s));
        }
        (forward, back, again)
    });
    let mut expected_back = forward[..5].to_vec();
    expected_back.reverse();
    assert_eq!(back, expected_back);
    assert_eq!(again, forward[1..].to_vec());
    assert_eq!((forward[5].1, forward[5].2.as_str()), (Some(11), "60"));
    // Each line was printed once, however often its stretch ran (a
    // call's p_rint! line, then its value shown at the top level).
    assert_eq!(out, "1\n1\n3\n3\n6\n");
}

#[test]
fn reverse_continue_stops_where_the_forward_run_stopped() {
    let ((forward, back), _) = session(PROGRAM, |d| {
        d.set_breakpoints("t.xtl", &[4]);
        d.start();
        let mut forward = Vec::new();
        for _ in 0..3 {
            let s = d.resume();
            forward.push(state(d, &s));
        }
        let mut back = Vec::new();
        for _ in 0..2 {
            let s = d.reverse_continue();
            back.push(state(d, &s));
        }
        (forward, back)
    });
    assert_eq!(
        forward.iter().map(|f| f.2.as_str()).collect::<Vec<_>>(),
        ["1", "3", "6"]
    );
    assert_eq!(back, vec![forward[1].clone(), forward[0].clone()]);
}

#[test]
fn run_to_a_step_goes_back_or_forward() {
    let ((marks, there), _) = session(PROGRAM, |d| {
        d.start();
        let mut marks = Vec::new();
        for _ in 0..4 {
            let s = d.step_over();
            marks.push(state(d, &s));
        }
        let back = d.run_to(marks[1].0);
        let early = state(d, &back);
        let ahead = d.run_to(marks[3].0);
        (marks, (early, state(d, &ahead)))
    });
    assert_eq!(there.0, marks[1]);
    assert_eq!(there.1, marks[3]);
}

const LONG: &str = "total! := 0
u:c_ount := { n ->
  total! := total! + 1
  n = 0 ? total!
  u:c_ount n - 1
}
u:c_ount 300
total!";

#[test]
fn checkpoints_stay_few_on_a_long_run() {
    let ((count, back), _) = session(LONG, |d| {
        d.keep_checkpoints(20, 6);
        d.set_breakpoints("t.xtl", &[4]);
        d.start();
        let mut hits = Vec::new();
        for _ in 0..250 {
            let s = d.resume();
            hits.push(state(d, &s));
        }
        let count = d.checkpoints();
        let s = d.run_to(hits[100].0);
        (count, (state(d, &s), hits[100].clone()))
    });
    assert!(count <= 6, "{count} checkpoints");
    assert_eq!(back.0, back.1);
}
