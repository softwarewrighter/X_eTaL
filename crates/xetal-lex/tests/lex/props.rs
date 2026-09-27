//! Property tests: the lexer never panics, and on success its tokens are
//! ordered, non-overlapping and separated only by whitespace.

use proptest::prelude::*;
use xetal_lex::{TokenKind, lex};

const PIECES: &[&str] = &[
    "r", "r_", "t_2", "t_12", "now_@", "m.f_", "+^r", "+^r_2", "f^s", "+", "-", "*", "/", "=", "<",
    ">", "|", "_l", "_r", "@", ";", "{", "}", "(", ")", "[", "]", "42", "2.5", "-1", "\n",
];

fn gaps_are_whitespace(src: &str) -> Result<(), TestCaseError> {
    if let Ok(tokens) = lex(src) {
        let mut pos = 0;
        for t in &tokens {
            prop_assert!(t.span.start >= pos && t.span.end > t.span.start);
            prop_assert!(
                src[pos..t.span.start]
                    .chars()
                    .all(|c| matches!(c, ' ' | '\t' | '\r'))
            );
            pos = t.span.end;
        }
        prop_assert!(src[pos..].chars().all(|c| matches!(c, ' ' | '\t' | '\r')));
    }
    Ok(())
}

proptest! {
    #[test]
    fn never_panics_on_any_string(src in any::<String>()) {
        let _ = lex(&src);
    }

    #[test]
    fn never_panics_on_ascii_soup(src in "[ -~\n\t]{0,40}") {
        gaps_are_whitespace(&src)?;
    }

    #[test]
    fn spaced_valid_pieces_always_lex(pieces in prop::collection::vec(prop::sample::select(PIECES), 0..20)) {
        let src = pieces.join(" ");
        let tokens = lex(&src).map_err(|e| TestCaseError::fail(format!("{src:?}: {e:?}")))?;
        prop_assert_eq!(tokens.len(), pieces.len());
        gaps_are_whitespace(&src)?;
    }

    #[test]
    fn each_token_relexes_to_itself(pieces in prop::collection::vec(prop::sample::select(PIECES), 1..10)) {
        let src = pieces.join(" ");
        for t in lex(&src).expect("valid pieces lex") {
            let alone = lex(&src[t.span.start..t.span.end]).expect("token text lexes");
            prop_assert_eq!(alone.len(), 1);
            prop_assert_eq!(&alone[0].kind, &t.kind);
            prop_assert!(!matches!(t.kind, TokenKind::Newline) || &src[t.span.start..t.span.end] == "\n");
        }
    }
}
