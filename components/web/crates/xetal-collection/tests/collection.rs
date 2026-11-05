//! The live demo's X_eTaL-libraries: with the collection installed, a
//! program imports Equations (and what it imports) with nothing saved,
//! runs, and its notation call is drawn as math; the vendored demos
//! are there, from the pinned commit.

use std::sync::{Arc, OnceLock};

/// One empty store and the collection, for this test binary.
fn installed() {
    static ONCE: OnceLock<()> = OnceLock::new();
    ONCE.get_or_init(|| {
        xetal_store::install(Arc::new(xetal_store::Memory::default()));
        xetal_collection::install();
    });
}

const GELU: &str = "\"e:\" u_se< \"Equations\"
pi := 3.141592653589793
u:g_elu := { x -> x * 0.5 + \"L[] | \" e:e_q< \"integral u:=0..x by simpson 32 | exp(-(u^2) / 2) / sqrt(2 * pi)\" }
u:g_elu 1.0";

#[test]
fn a_program_imports_equations_from_the_collection() {
    installed();
    let run = xetal_play::run(GELU, 1);
    assert_eq!(run.err, "", "{run:?}");
    assert_eq!(run.out.trim(), "0.8413447486334517", "Simpson on 32 steps");
}

#[test]
fn its_notation_call_is_drawn_as_math() {
    installed();
    let math: Vec<String> = xetal_play::decorated(GELU)
        .into_iter()
        .filter(|s| s.class == xetal_play::Class::Notation && !s.text.is_empty())
        .map(|s| s.text)
        .collect();
    assert_eq!(
        math,
        vec!["L = \\int_{0}^{x} \\frac{\\exp(-\\frac{u^{2}}{2})}{\\sqrt{2 \\, \\pi}} \\, du"]
    );
}

#[test]
fn the_vendored_files_and_their_pin() {
    assert_eq!(xetal_collection::PIN.len(), 40, "a full commit");
    assert!(xetal_collection::library("Equations.xtlm").is_some());
    assert!(
        xetal_collection::library("Stats.xtl").is_none(),
        "a standard library is not here"
    );
    assert!(
        xetal_collection::demo("Equations/integrals.xtl").is_some_and(|t| t.contains("e:e_q<"))
    );
}
