//! Hygiene (MC30), with stand-in macros: binders a macro writes around
//! the call's text are renamed, in order; declared anaphora and binders
//! around none of the call's text are not; a program cannot write the
//! fresh namespace.

use xetal_base::Diagnostic;
use xetal_expand::{MacroCall, Macros, binds_of, expand_with};

/// `t_wice<` binds t around its text; `w_ith<` binds it (declared);
/// `s_hown<` binds v around none of its text.
struct Binding;

impl Macros for Binding {
    fn run(&self, call: &MacroCall) -> Result<String, Diagnostic> {
        let r = call.right.unwrap_or("");
        Ok(match call.name {
            "t_wice<" => format!("{{ @ -> t := 2; t * ({r}) }} @"),
            "w_ith<" => format!("{{ @ -> it := 7; {r} }} @"),
            "p_air<" => format!("{{ @ -> (a, _) := (2, 3); a * ({r}) }} @"),
            "s_plit<" => format!("{{ (a, b) -> a + b + ({r}) }} (1, 2)"),
            _ => format!("{{ v -> v }} ({r})"),
        })
    }

    fn binds(&self, call: &MacroCall) -> Vec<String> {
        match call.name {
            "w_ith<" => vec!["it".into()],
            _ => Vec::new(),
        }
    }
}

fn text(src: &str) -> String {
    expand_with(src, &Binding).unwrap().text().to_string()
}

#[test]
fn a_binder_around_the_calls_text_is_renamed_in_order() {
    assert_eq!(
        text("@ x:t_wice< \"t + 1\"\n@ x:t_wice< \"t - 1\""),
        "{ @ -> g1:t := 2; g1:t * (t + 1) } @\n{ @ -> g2:t := 2; g2:t * (t - 1) } @"
    );
}

#[test]
fn a_tuple_patterns_names_are_binders_too() {
    // TU11: a pattern in a local binding and in a parameter list.
    assert_eq!(
        text("@ x:p_air< \"a + 1\""),
        "{ @ -> (g1:a, _) := (2, 3); g1:a * (a + 1) } @"
    );
    assert_eq!(
        text("@ x:s_plit< \"a * b\""),
        "{ (g1:a, g2:b) -> g1:a + g2:b + (a * b) } (1, 2)"
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
    let binds = binds_of(lib);
    assert_eq!(
        binds.get("w_ith<"),
        Some(&vec!["it".to_string(), "that".to_string()])
    );
    assert_eq!(binds.get("o_ther<"), None);
}
