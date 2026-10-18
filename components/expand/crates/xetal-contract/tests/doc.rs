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
