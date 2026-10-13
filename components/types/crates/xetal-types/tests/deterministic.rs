//! Inference gives the same answer in every run (ask G20): checking a
//! program never depends on a hash map's order, which Rust changes from
//! one map to the next. Each case is checked 30 times in this process;
//! every map made is seeded afresh, so an order-dependent answer would
//! show two results.

use xetal_types::check_source;

fn every_time(src: &str) -> String {
    let show = |r: Result<Vec<String>, xetal_base::Diagnostic>| match r {
        Ok(lines) => lines.join("\n"),
        Err(d) => d.to_string(),
    };
    let first = show(check_source(src));
    for _ in 0..30 {
        assert_eq!(
            show(check_source(src)),
            first,
            "a second answer for:\n{src}"
        );
    }
    first
}

#[test]
fn the_first_undefined_name_is_reported() {
    let src = "u:f_ := { x -> x + h:b + h:a }\nu:g_ := { x -> h:c }\n";
    let shown = every_time(src);
    assert!(
        shown.contains("undefined-name") && shown.contains("at 19..22"),
        "{shown}"
    );
}

#[test]
fn a_group_of_functions_and_values_closes_in_one_order() {
    let src = "l:d_raw := { (a, md) -> (md = h:lim) + a + h:g_ md }\n\
               h:lim := 2\n\
               h:g_ := { x -> x * 0 + h:lim }\n\
               h:a_ := { x -> x + h:v }\nh:v := h:b_ 1\nh:b_ := { y -> h:a_ y }\n";
    every_time(src);
}
