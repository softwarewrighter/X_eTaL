//! Hygiene by choice (MC30), with stand-in macros: a name a macro writes
//! stays as written, around the call's text or not; a macro asks for a
//! fresh name (the counter `[]G_ENSYM` reads), numbered in the order
//! calls are expanded; the call's text cannot bind a declared anaphor;
//! a program cannot write the fresh namespace.

use xetal_base::Diagnostic;
use xetal_expand::{Contract, MacroCall, Macros, contracts_of, expand_with};

/// `t_wice<` binds t around its text; `f_resh<` binds a fresh name
/// around it; `w_ith<` binds it (declared); `s_hown<` binds v around
/// none of its text.
struct Binding;

impl Macros for Binding {
    fn run(&self, call: &MacroCall) -> Result<String, Diagnostic> {
        let r = call.right.unwrap_or("");
        Ok(match call.name {
            "t_wice<" => format!("{{ @ -> t := 2; t * ({r}) }} @"),
            "f_resh<" => {
                call.fresh.set(call.fresh.get() + 1);
                let t = format!("g{}:t", call.fresh.get());
                format!("{{ @ -> {t} := 2; {t} * ({r}) }} @")
            }
            "w_ith<" => format!("{{ @ -> it := 7; {r} }} @"),
            "p_air<" => format!("{{ @ -> (a, _) := (2, 3); a * ({r}) }} @"),
            "s_plit<" => format!("{{ (a, b) -> a + b + ({r}) }} (1, 2)"),
            _ => format!("{{ v -> v }} ({r})"),
        })
    }

    fn contract(&self, call: &MacroCall) -> Contract {
        Contract {
            binds: match call.name {
                "w_ith<" => vec!["it".into()],
                _ => Vec::new(),
            },
            ..Contract::default()
        }
    }
}

fn text(src: &str) -> String {
    expand_with(src, &Binding).unwrap().text().to_string()
}

#[test]
fn a_binder_around_the_calls_text_is_kept_as_written() {
    // No automatic renaming: the macro's t is the one its text sees.
    assert_eq!(
        text("@ x:t_wice< \"t + 1\""),
        "{ @ -> t := 2; t * (t + 1) } @"
    );
    assert_eq!(
        text("@ x:p_air< \"a + 1\""),
        "{ @ -> (a, _) := (2, 3); a * (a + 1) } @"
    );
}

#[test]
fn fresh_names_are_numbered_in_the_order_calls_are_expanded() {
    assert_eq!(
        text("@ x:f_resh< \"t + 1\"\n@ x:f_resh< \"t - 1\""),
        "{ @ -> g1:t := 2; g1:t * (t + 1) } @\n{ @ -> g2:t := 2; g2:t * (t - 1) } @"
    );
}

#[test]
fn the_calls_text_cannot_bind_a_declared_anaphor() {
    for src in [
        "@ x:w_ith< \"'{ it -> it } e_ach 1 2\"",
        "@ x:w_ith< \"it := 3; it\"",
        "@ x:w_ith< \"'{ (it, b) -> b } e_ach 1\"",
    ] {
        let d = expand_with(src, &Binding).unwrap_err();
        assert_eq!(d.code, "shadows-anaphor", "{src}");
    }
    // Another name, or it only read, is fine.
    assert_eq!(
        text("@ x:w_ith< \"'{ x -> x + it } e_ach 1\""),
        "{ @ -> it := 7; '{ x -> x + it } e_ach 1 } @"
    );
}

#[test]
fn a_declared_anaphor_is_kept() {
    assert_eq!(text("@ x:w_ith< \"it + 1\""), "{ @ -> it := 7; it + 1 } @");
}

#[test]
fn a_binder_around_none_of_the_calls_text_is_kept() {
    assert_eq!(text("@ x:s_hown< \"v + 1\""), "{ v -> v } (v + 1)");
}

#[test]
fn a_program_cannot_write_a_fresh_name() {
    let d = expand_with("g1:t := 2", &Binding).unwrap_err();
    assert_eq!(d.code, "reserved-namespace");
}

#[test]
fn binds_are_read_from_the_doc_block_above_a_macro() {
    let lib = "## Doc.\n## binds: it that\nm:w_ith< := { a b -> b }\n\n## binds: x\n# a plain comment ends the block\nm:o_ther< := { a b -> b }\n";
    let contracts = contracts_of(lib).unwrap();
    assert_eq!(
        contracts["w_ith<"].binds,
        vec!["it".to_string(), "that".to_string()]
    );
    assert!(contracts["o_ther<"].binds.is_empty());
}
