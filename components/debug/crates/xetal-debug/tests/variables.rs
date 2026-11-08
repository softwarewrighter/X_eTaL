//! What a stopped session shows: its scopes and variables, a large
//! value a page at a time, watches evaluated where it stopped,
//! breakpoints with conditions and hit counts, and a watch's history.

use xetal_debug::{Breakpoint, Debugger, Reason, Scope, Var};

/// Load and check `src` as `t.xtl`, then run `drive` on a session.
fn session<T>(src: &str, drive: impl FnOnce(&mut Debugger) -> T) -> T {
    let mut loaded = xetal_program::load("t.xtl", src).expect("loads");
    xetal_types::check_program(&mut loaded.program).expect("checks");
    let mut out = Vec::new();
    let mut dbg = Debugger::new(&loaded.program, &loaded.sources, &mut out, 1);
    drive(&mut dbg)
}

fn find<'s>(scopes: &'s [Scope], scope: &str, name: &str) -> &'s Var {
    let s = scopes
        .iter()
        .find(|s| s.name == scope)
        .unwrap_or_else(|| panic!("no {scope} in {scopes:?}"));
    s.vars
        .iter()
        .find(|v| v.name == name)
        .unwrap_or_else(|| panic!("no {name} in {s:?}"))
}

const PROGRAM: &str = "u:d_ouble := { x ->
  y := x * 2
  y
}
a := 1 + 2
m := 4 3 r_eshape r_ange 12
b := u:d_ouble a
a + b";

#[test]
fn a_stop_in_a_call_shows_its_locals_and_the_top_level() {
    session(PROGRAM, |d| {
        d.set_breakpoints("t.xtl", &[3]);
        d.start();
        assert_eq!(d.resume().reason, Reason::Breakpoint);
        let scopes = d.scopes();
        assert_eq!(find(&scopes, "Locals", "y").value, "6");
        assert_eq!(find(&scopes, "Locals", "x").value, "3");
        assert_eq!(find(&scopes, "Top level", "a").value, "3");
        assert_eq!(find(&scopes, "Top level", "u:d_ouble").value, "<function>");
        let names: Vec<&str> = scopes[0].vars.iter().map(|v| v.name.as_str()).collect();
        assert_eq!(
            names,
            ["y", "x"],
            "innermost first, no names the compiler made"
        );
    });
}

#[test]
fn a_matrix_opens_a_page_of_rows() {
    session(PROGRAM, |d| {
        d.set_breakpoints("t.xtl", &[7]);
        d.start();
        d.resume();
        let scopes = d.scopes();
        let m = find(&scopes, "Top level", "m").clone();
        assert_eq!(m.value, "Int 4 3: 1 2 3 4 5 6 7 8 ...");
        assert_eq!((m.children, m.type_name.as_str()), (4, "Int"));
        let rows = d.children(m.id, 1, 2);
        let shown: Vec<(&str, &str)> = rows
            .iter()
            .map(|v| (v.name.as_str(), v.value.as_str()))
            .collect();
        assert_eq!(shown, [("[2]", "Int 3: 4 5 6"), ("[3]", "Int 3: 7 8 9")]);
        let items = d.children(rows[0].id, 0, 9);
        assert_eq!(items.len(), 3);
        assert_eq!(items[2].value, "6");
    });
}

#[test]
fn watches_are_evaluated_where_it_stopped() {
    session(PROGRAM, |d| {
        d.set_breakpoints("t.xtl", &[3]);
        d.start();
        d.resume();
        assert_eq!(d.evaluate("y + x").map(|v| v.value), Ok("9".to_string()));
        assert_eq!(
            d.evaluate("u:d_ouble a").map(|v| v.value),
            Ok("6".to_string())
        );
        assert_eq!(
            d.evaluate("'+ r_/ r_ange 4").map(|v| v.value),
            Ok("10".to_string())
        );
        assert!(d.evaluate("p_rint! y").unwrap_err().contains("effects"));
        assert!(d.evaluate("y / 0").unwrap_err().contains("error["));
    });
}

const FACT: &str = "u:f_act := { n ->
  n <= 1 ? 1
  n * u:f_act n - 1
}
u:f_act 5";

#[test]
fn a_condition_picks_the_hit_where_it_is_true() {
    let ns = session(FACT, |d| {
        let b = Breakpoint {
            line: 2,
            condition: Some("n = 2".into()),
            hit: None,
        };
        d.set_conditional("t.xtl", vec![b]);
        d.start();
        let mut ns = Vec::new();
        while d.resume().reason == Reason::Breakpoint {
            ns.push(d.evaluate("n").unwrap().value);
        }
        ns
    });
    assert_eq!(ns, ["2"]);
}

#[test]
fn a_hit_condition_counts_the_hits() {
    let hits = |spec: &str| {
        session(FACT, |d| {
            let b = Breakpoint {
                line: 2,
                condition: None,
                hit: Some(spec.into()),
            };
            d.set_conditional("t.xtl", vec![b]);
            d.start();
            let mut ns = Vec::new();
            while d.resume().reason == Reason::Breakpoint {
                ns.push(d.evaluate("n").unwrap().value);
            }
            ns
        })
    };
    assert_eq!(hits("3"), ["3", "2", "1"], "from the third hit on");
    assert_eq!(hits("== 2"), ["4"]);
    assert_eq!(hits("% 2"), ["4", "2"]);
}

#[test]
fn a_watch_keeps_its_value_at_every_stop() {
    let history = session(FACT, |d| {
        d.set_breakpoints("t.xtl", &[2]);
        d.start();
        d.add_watch("n * 10");
        while d.resume().reason == Reason::Breakpoint {}
        d.watch_history("n * 10")
    });
    let values: Vec<&str> = history.iter().map(|(_, v)| v.as_str()).collect();
    assert_eq!(values, ["50", "40", "30", "20", "10"]);
}
