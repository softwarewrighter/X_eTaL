//! A program that reads the keyboard ([]R_EAD, []K_EY ...) can run only
//! under Run, which waits for typed lines; the page turns Notebook and
//! Step off for it rather than let them fail at the first read.

use xetal_play::reads_input;

const DEMOS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../demos/");

fn demo(name: &str) -> String {
    std::fs::read_to_string(format!("{DEMOS}{name}")).expect("demo")
}

#[test]
fn the_games_that_ask_for_input_read_it() {
    assert!(reads_input(&demo("classics/mastermind-play.xtl")));
    assert!(reads_input(&demo("tttml-play.xtl")));
}

#[test]
fn a_program_that_reads_nothing_or_only_mentions_it_does_not() {
    assert!(!reads_input(&demo("tour.xtl")));
    assert!(!reads_input("# []R_EAD in a comment\n1 + 2\n"));
    assert!(reads_input("[]K_EY @\n"));
}
