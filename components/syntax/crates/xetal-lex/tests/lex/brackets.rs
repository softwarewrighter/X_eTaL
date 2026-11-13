//! Bracket pairs and depths (D-11 applied groups, D-12 rainbow
//! delimiters).

use xetal_lex::{Bracket, applied_parens, bracket_depths, lex};

fn depths(src: &str) -> Vec<Bracket> {
    let tokens = lex(src).unwrap();
    bracket_depths(&tokens).into_iter().flatten().collect()
}

#[test]
fn every_kind_counts_toward_the_depth() {
    use Bracket::Depth as D;
    assert_eq!(
        depths("[['+ r_/ - t_ally] * { (_r) }]"),
        [D(0), D(1), D(1), D(1), D(2), D(2), D(1), D(0)]
    );
}

#[test]
fn a_pair_has_one_depth_and_strays_are_unmatched() {
    use Bracket::{Depth as D, Unmatched as U};
    assert_eq!(depths("(a]"), [U, U]);
    assert_eq!(depths("((a)"), [U, D(1), D(1)]);
    assert_eq!(depths("a)"), [U]);
    // A closer that does not close the innermost opener is a stray.
    assert_eq!(depths("[(a])"), [U, D(1), U, D(1)]);
}

#[test]
fn only_brackets_get_one() {
    let tokens = lex("x + (1)").unwrap();
    let got = bracket_depths(&tokens);
    assert_eq!(got.len(), tokens.len());
    assert_eq!(got.iter().filter(|b| b.is_some()).count(), 2);
}

#[test]
fn applied_groups_are_found() {
    let tokens = lex("((a f_ b)_ c)_ d").unwrap();
    let marked = applied_parens(&tokens).iter().filter(|m| **m).count();
    assert_eq!(marked, 4);
}
