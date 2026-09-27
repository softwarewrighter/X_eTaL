//! Round-trip properties over lexable sources.

use proptest::prelude::*;
use xetal_lex::lex;
use xetal_render::{decorate, undecorate};

const PIECES: &[&str] = &[
    "r", "r_", "t_2", "t_12", "now_@", "m.f_", "m.f^e_@", "+^r", "+^r_2", "+^reduce", "+^q",
    "+^rq_2", "f^s", "f^R_@", "+_1", "+_@", "+", "-", "*", "/", "=", "<", ">", "|", "_l", "_r",
    "@", ";", "{", "}", "(", ")", "[", "]", "42", "2.5", "-1",
];

fn source() -> impl Strategy<Value = String> {
    let sep = prop::sample::select(vec!["", " ", "  ", "\n", "\t", " \n "]);
    prop::collection::vec((prop::sample::select(PIECES), sep), 0..16)
        .prop_map(|v| v.into_iter().flat_map(|(p, s)| [p, s]).collect())
}

proptest! {
    #[test]
    fn raw_decorated_raw_is_identity(src in source()) {
        prop_assume!(lex(&src).is_ok());
        let decorated = decorate(&src).expect("lexable source decorates");
        prop_assert_eq!(undecorate(&decorated).expect("decorated inverts"), src);
    }

    #[test]
    fn decorated_output_relexes_to_the_same_tokens(src in source()) {
        prop_assume!(lex(&src).is_ok());
        let back = undecorate(&decorate(&src).unwrap()).unwrap();
        let kinds = |s: &str| lex(s).unwrap().into_iter().map(|t| t.kind).collect::<Vec<_>>();
        prop_assert_eq!(kinds(&back), kinds(&src));
    }

    #[test]
    fn undecorate_never_panics(text in any::<String>()) {
        let _ = undecorate(&text);
    }

    #[test]
    fn decorate_never_panics(src in any::<String>()) {
        let _ = decorate(&src);
    }
}
