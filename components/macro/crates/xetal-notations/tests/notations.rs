//! A macro's notation sides are drawn by its `draws:` macro (MC32): the
//! program phase runs it for each call and records the drawing on the
//! call's site; `notations` lists them with the strings they replace.

use xetal_notations::notations;
use xetal_program::load;

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("xetal-notations-{}-{name}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// A tiny equation library: e_q< gives the right side's length as a
/// number, t_ex< draws both sides as "lhs = rhs" in LaTeX.
const EQ: &str = "## takes: notation notation\n## draws: t_ex<\n## uses: parsed\nm:e_q< := { l r -> f_ormat t_ally r }\n\n## takes: text text\nm:t_ex< := { l r -> \"\\\"\" c_at l c_at \" = \\\\\\\\sum \" c_at r c_at \"\\\"\" }\n";

#[test]
fn a_notation_calls_drawing_is_recorded_on_its_site() {
    let dir = scratch("drawn");
    std::fs::write(dir.join("Eq.xtlm"), EQ).unwrap();
    let main = dir.join("main.xtl").display().to_string();
    let src = "\"e:\" u_se< \"Eq\"\ny := \"L\" e:e_q< \"a + b\"\n";
    let loaded = load(&main, src).unwrap();
    let drawn = notations(0, &loaded.sources);
    assert_eq!(drawn.len(), 1);
    let n = &drawn[0];
    assert_eq!(n.latex, "L = \\sum a + b");
    // The strings it replaces: the left and the right side, as written.
    assert_eq!(&src[n.left.unwrap().start..n.left.unwrap().end], "\"L\"");
    assert_eq!(
        &src[n.right.unwrap().start..n.right.unwrap().end],
        "\"a + b\""
    );
    assert_eq!(&src[n.call.start..n.call.end], "\"L\" e:e_q< \"a + b\"");
}

#[test]
fn a_call_of_a_macro_without_draws_is_not_a_notation() {
    let dir = scratch("plain");
    std::fs::write(dir.join("Eq.xtlm"), EQ).unwrap();
    let main = dir.join("main.xtl").display().to_string();
    let src = "\"e:\" u_se< \"Eq\"\n\"L\" e:t_ex< \"a\"\n";
    let loaded = load(&main, src).unwrap();
    assert!(notations(0, &loaded.sources).is_empty());
}
