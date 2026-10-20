//! The contract lines of a macro's doc block (MC31): `## binds:`,
//! `## takes:` and `## uses:` directly above its definition.

use xetal_contract::{Contract, Uses, contracts_of};

const LIB: &str = "\
## Twice the text on the right.
## takes: @ text
m:t_wice< := { @ e -> e c_at \"; \" c_at e }

## The text with it bound.
## binds: it
## takes: text text
## uses: parsed
m:w_ith< := { e body -> body }

## Nothing declared.
m:p_lain< := { a b -> a }

## takes: text @
## binds: e
m:b_ody< := { b @ -> b }
";

#[test]
fn each_line_fills_its_field() {
    let c = contracts_of(LIB).unwrap();
    let twice = &c["t_wice<"];
    assert_eq!(twice.takes, Some((false, true)));
    assert_eq!(twice.uses, Uses::Copied);
    assert!(twice.binds.is_empty());
    let with = &c["w_ith<"];
    assert_eq!(with.binds, vec!["it".to_string()]);
    assert_eq!(with.takes, Some((true, true)));
    assert_eq!(with.uses, Uses::Parsed);
    let body = &c["b_ody<"];
    assert_eq!(body.takes, Some((true, false)));
    assert_eq!(body.binds, vec!["e".to_string()]);
}

#[test]
fn a_macro_without_contract_lines_copies_and_takes_anything() {
    let c = contracts_of(LIB).unwrap();
    assert_eq!(c.get("p_lain<"), Some(&Contract::default()));
    assert_eq!(Contract::default().uses, Uses::Copied);
    assert_eq!(Contract::default().takes, None);
}

#[test]
fn contract_lines_belong_to_the_definition_directly_below() {
    // A blank line, a comment or a definition ends the doc block.
    let c = contracts_of("## takes: @ @\n\nm:a_< := { @ @ -> \"1\" }\n").unwrap();
    assert_eq!(c["a_<"].takes, None);
    let c = contracts_of("## takes: @ @\nh:x_ := { _r }\nm:a_< := { @ @ -> \"1\" }\n").unwrap();
    assert!(!c.contains_key("a_<") || c["a_<"].takes.is_none());
}

#[test]
fn a_malformed_line_is_an_error_at_its_line() {
    let bad = contracts_of("## takes: text\nm:a_< := { @ e -> e }\n").unwrap_err();
    assert_eq!(bad.code, "bad-contract");
    assert!(bad.message.contains("takes:"), "{}", bad.message);
    assert_eq!(bad.span.map(|s| s.start), Some(0));
    let bad = contracts_of("## uses: both\nm:a_< := { @ e -> e }\n").unwrap_err();
    assert_eq!(bad.code, "bad-contract");
    assert!(bad.message.contains("copied") && bad.message.contains("parsed"));
    let bad = contracts_of("## takes: text @ text\nm:a_< := { @ e -> e }\n").unwrap_err();
    assert_eq!(bad.code, "bad-contract");
}

/// `code` is a side that takes text which is X_eTaL (ST5, step 112): a
/// text side, marked as code.
#[test]
fn code_is_a_text_side_that_takes_x_etal() {
    let c = contracts_of("## takes: @ code\nm:s_tage< := { @ t -> t }\n").unwrap();
    let stage = &c["s_tage<"];
    assert_eq!(stage.takes, Some((false, true)));
    assert_eq!(stage.code, (false, true));
    let c = contracts_of("## takes: code text\nm:w_< := { c t -> c }\n").unwrap();
    assert_eq!(c["w_<"].code, (true, false));
    assert_eq!(c["w_<"].takes, Some((true, true)));
}

/// `notation` is a text side in a notation the library reads (MC32),
/// drawn by the macro `## draws:` names, one of the library's own.
#[test]
fn notation_is_a_text_side_drawn_by_the_draws_macro() {
    let lib = "## takes: notation notation\n## draws: e:t_ex<\nm:e_q< := { l r -> l }\n\n## takes: text text\nm:t_ex< := { l r -> r }\n";
    let c = contracts_of(lib).unwrap();
    let eq = &c["e_q<"];
    assert_eq!(eq.takes, Some((true, true)));
    assert_eq!(eq.code, (false, false));
    assert_eq!(eq.notation, (true, true));
    assert_eq!(eq.draws.as_deref(), Some("t_ex<"));
    assert_eq!(c["t_ex<"].notation, (false, false));
    assert_eq!(c["t_ex<"].draws, None);
    // Written without the alias too.
    let c = contracts_of(
        "## takes: @ notation\n## draws: t_ex<\nm:e_q< := { @ r -> r }\nm:t_ex< := { l r -> r }\n",
    )
    .unwrap();
    assert_eq!(c["e_q<"].draws.as_deref(), Some("t_ex<"));
}

#[test]
fn a_notation_needs_its_drawer_and_a_drawer_its_notation() {
    let bad = contracts_of("## takes: text notation\nm:e_q< := { l r -> l }\n").unwrap_err();
    assert_eq!(bad.code, "bad-contract");
    assert!(bad.message.contains("draws:"), "{}", bad.message);
    let bad = contracts_of(
        "## takes: text text\n## draws: t_ex<\nm:e_q< := { l r -> l }\nm:t_ex< := { l r -> r }\n",
    )
    .unwrap_err();
    assert_eq!(bad.code, "bad-contract");
    assert!(bad.message.contains("notation"), "{}", bad.message);
    let bad = contracts_of("## takes: text notation\n## draws: n_ope<\nm:e_q< := { l r -> l }\n")
        .unwrap_err();
    assert_eq!(bad.code, "bad-contract");
    assert!(bad.message.contains("n_ope<"), "{}", bad.message);
    let bad = contracts_of("## draws: e_q<\n## takes: text notation\nm:e_q< := { l r -> l }\n")
        .unwrap_err();
    assert_eq!(bad.code, "bad-contract");
    assert!(bad.message.contains("itself"), "{}", bad.message);
}
